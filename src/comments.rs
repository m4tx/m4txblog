//! Comments on blog posts.
//!
//! Every comment is checked by Akismet. Comments considered spam end up with
//! the [`CommentStatus::Spam`] status; the rest are published right away,
//! unless they contain multiple links (or Akismet is unavailable), in which
//! case they're held for moderation. Comments are managed in the admin panel,
//! and the site owner can be notified about new comments by e-mail.
//!
//! To minimize the amount of personal data stored, the IP address, user agent
//! and referrer are only sent to Akismet and never saved.

use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::sync::OnceLock;

use async_trait::async_trait;
use chrono::{DateTime, FixedOffset};
use cot::admin::{AdminModel, AdminModelManager, DefaultAdminModelManager, Pagination};
use cot::auth::Auth;
use cot::common_types::{Email, Url};
use cot::db::impl_sqlite::SqliteValueRef;
use cot::db::query::{Expr, ExprSort};
use cot::db::{
    Auto, ColumnType, Database, DatabaseField, DbValue, ForeignKey, FromDbValue, LimitedString,
    Model, SqlxValueRef, ToDbValue, model,
};
use cot::email::EmailMessage;
use cot::form::fields::{SelectAsFormField, SelectChoice};
use cot::form::{Form, FormContext, FormErrorTarget, FormFieldValidationError, FormResult};
use cot::http::header::{REFERER, USER_AGENT};
use cot::request::{Request, RequestExt};
use serde::Deserialize;

use crate::akismet::{Akismet, CommentCheck, Verdict};

/// Replies nested deeper than this are shown at this depth.
const MAX_DEPTH: usize = 2;

/// Comments with more links than this are held for moderation, as that's
/// typical for spam that gets past Akismet.
const MAX_LINKS: usize = 1;

static AKISMET: OnceLock<Option<Akismet>> = OnceLock::new();
static NOTIFICATIONS: OnceLock<Option<NotificationConfig>> = OnceLock::new();

/// The `[comment_notifications]` section of the project config.
#[derive(Debug, Clone, Deserialize)]
pub struct NotificationConfig {
    /// The sender of the notification e-mails.
    from: Email,
    /// Where to send the notifications.
    to: Email,
}

/// Sets up the Akismet client and e-mail notifications. Should be called
/// once, at startup.
pub fn init(akismet: Option<Akismet>, notifications: Option<NotificationConfig>) {
    AKISMET
        .set(akismet)
        .expect("Akismet has already been initialized");
    NOTIFICATIONS
        .set(notifications)
        .expect("notifications have already been initialized");
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SelectChoice, SelectAsFormField)]
pub enum CommentStatus {
    #[select_choice(id = "pending", name = "Pending")]
    Pending,
    #[select_choice(id = "approved", name = "Approved")]
    Approved,
    #[select_choice(id = "spam", name = "Spam")]
    Spam,
    #[select_choice(id = "trash", name = "Trash")]
    Trash,
}

impl DatabaseField for CommentStatus {
    const TYPE: ColumnType = ColumnType::String(16);
}

impl FromDbValue for CommentStatus {
    fn from_sqlite(value: SqliteValueRef<'_>) -> cot::db::Result<Self> {
        let value = value.get::<String>()?;
        <Self as SelectChoice>::from_str(&value).map_err(cot::db::DatabaseError::value_decode)
    }
}

impl ToDbValue for CommentStatus {
    fn to_db_value(&self) -> DbValue {
        self.id().into()
    }
}

#[derive(Debug, Clone, Form, AdminModel)]
#[model]
pub struct Comment {
    #[model(primary_key)]
    pub id: Auto<i64>,
    pub post_permalink: LimitedString<255>,
    pub post_language: LimitedString<8>,
    /// Replies stay (as top-level comments) when their parent is deleted.
    #[model(foreign_key(on_delete = "set_none"))]
    pub parent: Option<ForeignKey<Comment>>,
    pub author_name: LimitedString<100>,
    pub author_email: Option<LimitedString<254>>,
    pub author_url: Option<LimitedString<500>>,
    #[form(opts(multiline = true))]
    pub content: String,
    pub status: CommentStatus,
    pub is_site_owner: bool,
    pub created_at: DateTime<FixedOffset>,
}

impl Display for Comment {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        const EXCERPT_LENGTH: usize = 80;

        let mut excerpt: String = self.content.chars().take(EXCERPT_LENGTH).collect();
        if self.content.chars().count() > EXCERPT_LENGTH {
            excerpt.push('…');
        }
        write!(
            f,
            "[{}] {} on {} ({}): {}",
            self.status.to_string().to_uppercase(),
            self.author_name,
            self.post_permalink,
            self.post_language,
            excerpt.replace('\n', " "),
        )
    }
}

/// Lists the comments newest first (so that the ones waiting for moderation
/// are at the top), optionally filtered with a `?status=pending` query param.
#[derive(Debug, Default)]
pub struct CommentAdminManager {
    default: DefaultAdminModelManager<Comment>,
}

impl CommentAdminManager {
    fn filtered_query(request: &Request) -> cot::Result<cot::db::query::Query<Comment>> {
        let mut query = Comment::objects();
        let status = request
            .uri()
            .query()
            .unwrap_or_default()
            .split('&')
            .find_map(|pair| pair.strip_prefix("status="))
            .and_then(|status| <CommentStatus as SelectChoice>::from_str(status).ok());
        if let Some(status) = status {
            query.filter(Expr::eq(Expr::field("status"), Expr::value(status)));
        }
        Ok(query)
    }
}

#[async_trait]
impl AdminModelManager for CommentAdminManager {
    fn name(&self) -> &str {
        self.default.name()
    }

    fn url_name(&self) -> &str {
        self.default.url_name()
    }

    async fn get_objects(
        &self,
        request: &Request,
        pagination: Pagination,
    ) -> cot::Result<Vec<Box<dyn AdminModel>>> {
        let db = request.context().database();
        let comments = Self::filtered_query(request)?
            .order_by([
                <Comment as Model>::Fields::created_at.desc(),
                <Comment as Model>::Fields::id.desc(),
            ])
            .limit(pagination.limit())
            .offset(pagination.offset())
            .all(db)
            .await?;
        #[expect(trivial_casts)] // Upcast to the correct Box type
        Ok(comments
            .into_iter()
            .map(|comment| Box::new(comment) as Box<dyn AdminModel>)
            .collect())
    }

    async fn get_total_object_counts(&self, request: &Request) -> cot::Result<u64> {
        let db = request.context().database();
        Ok(Self::filtered_query(request)?.count(db).await?)
    }

    async fn get_object_by_id(
        &self,
        request: &Request,
        id: &str,
    ) -> cot::Result<Option<Box<dyn AdminModel>>> {
        self.default.get_object_by_id(request, id).await
    }

    fn form_context(&self) -> Box<dyn FormContext> {
        self.default.form_context()
    }

    async fn form_context_from_object(&self, object: Box<dyn AdminModel>) -> Box<dyn FormContext> {
        self.default.form_context_from_object(object).await
    }

    async fn save_from_request(
        &self,
        request: &mut Request,
        object_id: Option<&str>,
    ) -> cot::Result<Option<Box<dyn FormContext>>> {
        self.default.save_from_request(request, object_id).await
    }

    async fn remove_by_id(&self, request: &mut Request, object_id: &str) -> cot::Result<()> {
        self.default.remove_by_id(request, object_id).await
    }
}

/// A comment, as displayed on a post page.
#[derive(Debug, Clone)]
pub struct DisplayComment {
    pub id: i64,
    pub author_name: String,
    pub author_url: Option<String>,
    pub is_site_owner: bool,
    pub created_at: DateTime<FixedOffset>,
    pub content_html: String,
    pub depth: usize,
    /// The author of the parent comment, if the reply is shown at a lower
    /// depth than it actually is (so it's not obvious which comment it replies
    /// to).
    pub in_reply_to: Option<String>,
}

/// Returns the approved comments for a post, as a flattened tree (i.e. replies
/// come right after their parents).
pub async fn approved_comments(
    db: &Database,
    permalink: &str,
    language: &str,
) -> cot::Result<Vec<DisplayComment>> {
    let mut comments = Comment::objects()
        .filter(Expr::and(
            Expr::eq(
                Expr::field("post_permalink"),
                Expr::value(permalink.to_owned()),
            ),
            Expr::and(
                Expr::eq(
                    Expr::field("post_language"),
                    Expr::value(language.to_owned()),
                ),
                Expr::eq(Expr::field("status"), Expr::value(CommentStatus::Approved)),
            ),
        ))
        .all(db)
        .await?;
    comments.sort_by_key(|comment| (comment.created_at, comment.id()));

    Ok(flatten_comment_tree(&comments))
}

fn flatten_comment_tree(comments: &[Comment]) -> Vec<DisplayComment> {
    let by_id: HashMap<i64, &Comment> = comments
        .iter()
        .map(|comment| (comment.id(), comment))
        .collect();
    let mut children: HashMap<Option<i64>, Vec<&Comment>> = HashMap::new();
    for comment in comments {
        // Replies to comments that aren't displayed are shown as top-level ones
        let parent = comment.parent_id().filter(|id| by_id.contains_key(id));
        children.entry(parent).or_default().push(comment);
    }

    let mut result = Vec::with_capacity(comments.len());
    let mut stack: Vec<(&Comment, usize)> = children
        .get(&None)
        .map(|roots| roots.iter().rev().map(|comment| (*comment, 0)).collect())
        .unwrap_or_default();
    while let Some((comment, depth)) = stack.pop() {
        let in_reply_to = if depth == MAX_DEPTH {
            comment
                .parent_id()
                .and_then(|id| by_id.get(&id))
                .map(|parent| parent.author_name.to_string())
        } else {
            None
        };
        result.push(DisplayComment {
            id: comment.id(),
            author_name: comment.author_name.to_string(),
            author_url: comment.author_url.as_ref().map(ToString::to_string),
            is_site_owner: comment.is_site_owner,
            created_at: comment.created_at,
            content_html: render_markdown(&comment.content),
            depth,
            in_reply_to,
        });
        if let Some(replies) = children.get(&Some(comment.id())) {
            let child_depth = (depth + 1).min(MAX_DEPTH);
            stack.extend(replies.iter().rev().map(|reply| (*reply, child_depth)));
        }
    }
    result
}

impl Comment {
    pub fn id(&self) -> i64 {
        match self.id {
            Auto::Fixed(id) => id,
            Auto::Auto => panic!("comment hasn't been saved yet"),
        }
    }

    fn parent_id(&self) -> Option<i64> {
        self.parent
            .as_ref()
            .and_then(|parent| match parent.primary_key() {
                Auto::Fixed(id) => Some(*id),
                Auto::Auto => None,
            })
    }
}

/// Renders a comment's Markdown to (safe) HTML.
///
/// Raw HTML is escaped, dangerous URLs are removed, images are converted to
/// links, and all links get `rel="nofollow ugc noopener"`.
pub fn render_markdown(content: &str) -> String {
    use comrak::nodes::NodeValue;
    use comrak::{Arena, Options, format_html, parse_document};

    let mut options = Options::default();
    options.extension.autolink = true;
    options.extension.strikethrough = true;
    options.render.hardbreaks = true;

    let arena = Arena::new();
    let root = parse_document(&arena, content, &options);
    for node in root.descendants() {
        let mut data = node.data.borrow_mut();
        if let NodeValue::Image(link) = &data.value {
            data.value = NodeValue::Link(link.clone());
        }
    }

    let mut html = String::new();
    format_html(root, &options, &mut html).expect("formatting to a String can't fail");
    // Raw HTML is escaped, so all `<a ` tags come from comrak itself
    html.replace("<a href=", "<a rel=\"nofollow ugc noopener\" href=")
}

#[derive(Debug, Form)]
pub struct CommentForm {
    pub parent: Option<i64>,
    pub author_name: LimitedString<100>,
    pub author_email: Option<Email>,
    pub author_url: Option<Url>,
    #[form(opts(max_length = 10000))]
    pub content: String,
    #[form(opts(must_be_true = true))]
    pub consent: bool,
    /// A honeypot field, hidden with CSS. Bots tend to fill all the fields.
    pub subject: Option<String>,
}

/// The values and errors of a submitted comment form, used to re-render it
/// when the validation fails.
#[derive(Debug, Clone, Default)]
pub struct CommentFormState {
    values: HashMap<String, String>,
    errors: HashMap<String, Vec<String>>,
}

impl CommentFormState {
    fn from_context(context: &dyn FormContext) -> Self {
        let mut state = Self::default();
        for field in context.fields() {
            let name = field.dyn_id().to_owned();
            if let Some(value) = field.dyn_value() {
                state.values.insert(name.clone(), value.to_owned());
            }
            let errors = context.errors_for(FormErrorTarget::Field(&name));
            if !errors.is_empty() {
                state
                    .errors
                    .insert(name, errors.iter().map(ToString::to_string).collect());
            }
        }
        let form_errors = context.errors_for(FormErrorTarget::Form);
        if !form_errors.is_empty() {
            state.errors.insert(
                String::new(),
                form_errors.iter().map(ToString::to_string).collect(),
            );
        }
        state
    }

    fn with_form_error(mut self, field: &str, error: impl Into<String>) -> Self {
        self.errors
            .entry(field.to_owned())
            .or_default()
            .push(error.into());
        self
    }

    pub fn value(&self, field: &str) -> &str {
        self.values.get(field).map_or("", String::as_str)
    }

    pub fn errors(&self, field: &str) -> &[String] {
        self.errors.get(field).map_or(&[], Vec::as_slice)
    }

    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    /// Returns the Bootstrap class to add to an invalid field.
    pub fn invalid_class(&self, field: &str) -> &'static str {
        if self.errors(field).is_empty() {
            ""
        } else {
            " is-invalid"
        }
    }

    pub fn is_checked(&self, field: &str) -> bool {
        matches!(self.value(field), "true" | "on" | "1")
    }
}

/// What happened to a submitted comment.
#[derive(Debug, Clone)]
pub enum CommentSubmission {
    /// The form is invalid and should be displayed again.
    Invalid(CommentFormState),
    /// The comment has been saved (or silently dropped as spam) and the user
    /// should be told it's awaiting moderation.
    Pending,
    /// The comment has been published (posted by the site owner).
    Published { id: i64 },
}

/// Information about the post a comment is being submitted for.
#[derive(Debug, Clone)]
pub struct CommentTarget<'a> {
    pub permalink: &'a str,
    pub language: &'a str,
    pub post_url: &'a str,
    pub comments_open: bool,
}

/// Validates, spam-checks and saves a submitted comment.
pub async fn submit_comment(
    request: &mut Request,
    db: &Database,
    auth: &Auth,
    target: &CommentTarget<'_>,
) -> cot::Result<CommentSubmission> {
    let user_ip = client_ip(request);
    let user_agent = header_value(request, USER_AGENT);
    let referrer = header_value(request, REFERER);

    let form = match CommentForm::from_request(request).await? {
        FormResult::Ok(form) => form,
        FormResult::ValidationError(context) => {
            return Ok(CommentSubmission::Invalid(CommentFormState::from_context(
                &context,
            )));
        }
    };

    if form
        .subject
        .as_deref()
        .is_some_and(|value| !value.is_empty())
    {
        tracing::info!("dropping a comment with the honeypot field filled");
        return Ok(CommentSubmission::Pending);
    }

    let state_for_errors = || {
        let mut state = CommentFormState::default();
        let values = [
            (
                "parent",
                form.parent.map(|p| p.to_string()).unwrap_or_default(),
            ),
            ("author_name", form.author_name.to_string()),
            (
                "author_email",
                form.author_email
                    .as_ref()
                    .map(|email| email.as_str().to_owned())
                    .unwrap_or_default(),
            ),
            (
                "author_url",
                form.author_url
                    .as_ref()
                    .map(|url| url.as_str().to_owned())
                    .unwrap_or_default(),
            ),
            ("content", form.content.clone()),
            ("consent", form.consent.to_string()),
        ];
        for (key, value) in values {
            state.values.insert(key.to_owned(), value);
        }
        state
    };

    if !target.comments_open {
        return Ok(CommentSubmission::Invalid(
            state_for_errors().with_form_error("", "Comments are closed for this post."),
        ));
    }
    let author_name = form.author_name.trim();
    let content = form.content.trim();
    if author_name.is_empty() {
        return Ok(CommentSubmission::Invalid(
            state_for_errors().with_form_error(
                "author_name",
                FormFieldValidationError::Required.to_string(),
            ),
        ));
    }
    if content.is_empty() {
        return Ok(CommentSubmission::Invalid(
            state_for_errors()
                .with_form_error("content", FormFieldValidationError::Required.to_string()),
        ));
    }

    let author_url = form.author_url.as_ref().map(|url| url.as_str());
    if author_url.is_some_and(|url| !url.starts_with("http://") && !url.starts_with("https://")) {
        return Ok(CommentSubmission::Invalid(
            state_for_errors().with_form_error(
                "author_url",
                "Only http:// and https:// addresses are allowed.",
            ),
        ));
    }

    let parent = if let Some(parent_id) = form.parent {
        let parent = Comment::objects()
            .filter(Expr::eq(Expr::field("id"), Expr::value(parent_id)))
            .get(db)
            .await?;
        match parent {
            Some(parent)
                if parent.status == CommentStatus::Approved
                    && parent.post_permalink.as_str() == target.permalink
                    && parent.post_language.as_str() == target.language =>
            {
                Some(parent)
            }
            _ => {
                return Ok(CommentSubmission::Invalid(
                    state_for_errors()
                        .with_form_error("", "The comment you're replying to doesn't exist."),
                ));
            }
        }
    } else {
        None
    };

    let is_site_owner = auth.user().is_authenticated();
    let author_email = form.author_email.as_ref().map(|email| email.as_str());

    let status = if is_site_owner {
        CommentStatus::Approved
    } else {
        let check = CommentCheck {
            user_ip: &user_ip,
            user_agent: &user_agent,
            referrer: &referrer,
            permalink: target.post_url,
            is_reply: parent.is_some(),
            author: author_name,
            author_email: author_email.unwrap_or_default(),
            author_url: author_url.unwrap_or_default(),
            content,
            language: target.language,
        };
        match spam_check(&check).await {
            Some(Verdict::Discard) => {
                tracing::info!("discarding a comment marked as blatant spam by Akismet");
                return Ok(CommentSubmission::Pending);
            }
            Some(Verdict::Spam) => CommentStatus::Spam,
            Some(Verdict::Ham) if count_links(content) <= MAX_LINKS => CommentStatus::Approved,
            // Too many links, or Akismet is unavailable
            Some(Verdict::Ham) | None => CommentStatus::Pending,
        }
    };

    let mut comment = Comment {
        id: Auto::auto(),
        post_permalink: LimitedString::new(target.permalink).map_err(cot::Error::internal)?,
        post_language: LimitedString::new(target.language).map_err(cot::Error::internal)?,
        parent: parent.map(ForeignKey::from),
        author_name: LimitedString::new(author_name).expect("already validated"),
        author_email: author_email
            .map(LimitedString::new)
            .transpose()
            .map_err(|_| cot::Error::internal("e-mail too long"))?,
        author_url: author_url
            .map(LimitedString::new)
            .transpose()
            .map_err(|_| cot::Error::internal("URL too long"))?,
        content: content.to_owned(),
        status,
        is_site_owner,
        created_at: chrono::Utc::now().fixed_offset(),
    };
    comment.insert(db).await?;
    tracing::info!(
        "new comment #{} on {} ({}): {:?}",
        comment.id(),
        target.permalink,
        target.language,
        status
    );

    if !is_site_owner && status != CommentStatus::Spam {
        notify_about_comment(request, &comment, target.post_url);
    }

    if status == CommentStatus::Approved {
        Ok(CommentSubmission::Published { id: comment.id() })
    } else {
        Ok(CommentSubmission::Pending)
    }
}

/// Roughly counts the links in a comment.
fn count_links(content: &str) -> usize {
    let with_scheme = content.matches("://").count();
    let without_scheme = content
        .match_indices("www.")
        .filter(|(index, _)| !content[..*index].ends_with("//"))
        .count();
    with_scheme + without_scheme
}

/// Sends an e-mail about a new comment to the site owner (in the background,
/// so that a slow mail server doesn't slow down posting comments).
fn notify_about_comment(request: &Request, comment: &Comment, post_url: &str) {
    let Some(config) = NOTIFICATIONS.get().and_then(Option::as_ref) else {
        return;
    };

    let admin_url = cot::reverse!(
        request,
        "cot_admin:edit_model_instance",
        model_name = "comment",
        pk = comment.id()
    )
    .map(|path| format!("{}{path}", crate::consts::WEBSITE_URL.trim_end_matches('/')))
    .unwrap_or_default();
    let status = match comment.status {
        CommentStatus::Approved => "published",
        _ => "awaiting moderation",
    };
    let body = format!(
        "{author} commented on {post_url}#comment-{id} ({status}):\n\n\
         {content}\n\n\
         ---\n\
         E-mail: {email}\n\
         Website: {url}\n\
         Manage: {admin_url}\n",
        author = comment.author_name,
        id = comment.id(),
        content = comment.content,
        email = comment
            .author_email
            .as_ref()
            .map_or("-", |email| email.as_str()),
        url = comment.author_url.as_ref().map_or("-", |url| url.as_str()),
    );
    let message = EmailMessage::builder()
        .from(config.from.clone())
        .to(vec![config.to.clone()])
        .subject(format!(
            "New comment by {} on {} ({status})",
            comment.author_name, comment.post_permalink
        ))
        .body(body)
        .build();
    let message = match message {
        Ok(message) => message,
        Err(error) => {
            tracing::error!("could not build the notification e-mail: {error}");
            return;
        }
    };

    let email = request.context().email().clone();
    tokio::spawn(async move {
        if let Err(error) = email.send(message).await {
            tracing::error!("could not send the notification e-mail: {error}");
        }
    });
}

async fn spam_check(check: &CommentCheck<'_>) -> Option<Verdict> {
    let Some(akismet) = AKISMET.get().and_then(Option::as_ref) else {
        tracing::warn!("Akismet is not configured; skipping the spam check");
        return None;
    };
    match akismet.check_comment(check).await {
        Ok(verdict) => Some(verdict),
        Err(error) => {
            tracing::warn!("Akismet check failed: {error}");
            None
        }
    }
}

fn header_value(request: &Request, header: cot::http::header::HeaderName) -> String {
    request
        .headers()
        .get(header)
        .and_then(|value| value.to_str().ok())
        .unwrap_or_default()
        .to_owned()
}

/// Returns the client's IP address, as reported by the reverse proxy.
///
/// The app is only supposed to be reachable through the reverse proxy, so the
/// header can be trusted.
fn client_ip(request: &Request) -> String {
    let real_ip = header_value(
        request,
        cot::http::header::HeaderName::from_static("x-real-ip"),
    );
    if !real_ip.is_empty() {
        return real_ip;
    }
    let forwarded_for = header_value(
        request,
        cot::http::header::HeaderName::from_static("x-forwarded-for"),
    );
    forwarded_for
        .split(',')
        .next()
        .map(str::trim)
        .filter(|ip| !ip.is_empty())
        .unwrap_or("127.0.0.1")
        .to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn links_are_counted() {
        assert_eq!(count_links("no links here"), 0);
        assert_eq!(count_links("see https://example.com"), 1);
        assert_eq!(
            count_links("https://www.example.com and www.example.org"),
            2
        );
        assert_eq!(
            count_links("[a](http://a.example) [b](https://b.example)"),
            2
        );
    }

    #[test]
    fn markdown_is_sanitized() {
        let html = render_markdown(
            "Hi <script>alert(1)</script> [x](javascript:alert(1)) ![img](https://example.com/a.png) https://example.com",
        );
        assert!(!html.contains("<script>"), "{html}");
        assert!(!html.contains("javascript:"), "{html}");
        assert!(!html.contains("<img"), "{html}");
        assert!(
            html.contains(
                r#"<a rel="nofollow ugc noopener" href="https://example.com/a.png">img</a>"#
            ),
            "{html}"
        );
        assert!(
            html.contains(r#"<a rel="nofollow ugc noopener" href="https://example.com">"#),
            "{html}"
        );
    }

    fn comment(id: i64, parent: Option<i64>, minute: u32) -> Comment {
        Comment {
            id: Auto::fixed(id),
            post_permalink: LimitedString::new("post").unwrap(),
            post_language: LimitedString::new("en").unwrap(),
            parent: parent.map(|id| ForeignKey::PrimaryKey(Auto::fixed(id))),
            author_name: LimitedString::new(format!("author{id}")).unwrap(),
            author_email: None,
            author_url: None,
            content: format!("comment {id}"),
            status: CommentStatus::Approved,
            is_site_owner: false,
            created_at: DateTime::parse_from_rfc3339(&format!("2026-01-01T00:{minute:02}:00Z"))
                .unwrap(),
        }
    }

    #[test]
    fn comment_tree_is_flattened() {
        let comments = [
            comment(1, None, 0),
            comment(2, None, 1),
            comment(3, Some(1), 2),
            comment(4, Some(3), 3),
            comment(5, Some(4), 4),
            comment(6, Some(1), 5),
            comment(7, Some(999), 6),
        ];
        let flat: Vec<_> = flatten_comment_tree(&comments)
            .into_iter()
            .map(|c| (c.id, c.depth, c.in_reply_to))
            .collect();
        assert_eq!(
            flat,
            vec![
                (1, 0, None),
                (3, 1, None),
                (4, 2, Some("author3".to_owned())),
                (5, 2, Some("author4".to_owned())),
                (6, 1, None),
                (2, 0, None),
                (7, 0, None),
            ]
        );
    }
}
