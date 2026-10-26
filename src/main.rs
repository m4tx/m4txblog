mod akismet;
mod comments;
pub mod consts;
mod feed;
mod legacy_redirects;
mod migrations;
mod posts;
pub(crate) mod template_util;

use std::sync::Arc;

use async_trait::async_trait;
use cot::admin::{AdminApp, AdminModelManager};
use cot::auth::Auth;
use cot::auth::db::{DatabaseUser, DatabaseUserApp};
use cot::cli::clap::{Arg, ArgMatches, Command};
use cot::cli::{Cli, CliMetadata, CliTask};
use cot::common_types::Password;
use cot::db::Database;
use cot::db::migrations::{MigrationEngine, SyncDynMigration};
use cot::error::NotFound;
use cot::error::handler::{DynErrorPageHandler, RequestError};
use cot::html::Html;
use cot::http::header::LOCATION;
use cot::middleware::{AuthMiddleware, SessionMiddleware, TrailingSlashMiddleware, from_fn};
use cot::project::{
    App, MiddlewareContext, Project, ProjectContext, RegisterAppsContext, RootHandler,
    RootHandlerBuilder, WithConfig,
};
use cot::request::extractors::{FromRequestHead, Path, StaticFiles, UrlQuery};
use cot::request::{Request, RequestExt, RequestHead};
use cot::response::{IntoResponse, Response, ResponseExt};
use cot::router::{Route, Router, Urls, method};
use cot::session::db::SessionApp;
use cot::static_files::{StaticFile, StaticFilesMiddleware};
use cot::{AppBuilder, Body, Bootstrapper, StatusCode, Template, static_files};
use m4txblog_common::md_pages::MdPage;
use m4txblog_macros::{md_page, static_files_dir};
use serde::Deserialize;
use tracing_subscriber::util::SubscriberInitExt;

use crate::akismet::{Akismet, AkismetConfig};
use crate::comments::{
    CommentAdminManager, CommentFormState, CommentSubmission, CommentTarget, DisplayComment,
    approved_comments, submit_comment,
};
use crate::feed::blog_feed;
use crate::legacy_redirects::{LegacyRedirectsConfig, redirect_legacy};
use crate::posts::{PostListEntry, get_archive_entries, get_homepage_entries, get_post_map};

#[derive(Debug, Clone, FromRequestHead)]
struct BaseContext {
    urls: Urls,
    static_files: StaticFiles,
    route_name: RouteName,
}

#[derive(Debug, Clone)]
struct RouteName(String);

impl FromRequestHead for RouteName {
    async fn from_request_head(head: &RequestHead) -> cot::Result<Self> {
        let route_name = head.route_name().unwrap_or_default().to_owned();
        Ok(Self(route_name))
    }
}

impl PartialEq<str> for RouteName {
    fn eq(&self, other: &str) -> bool {
        self.0 == other
    }
}

#[derive(Debug, Template)]
#[template(path = "index.html")]
struct IndexTemplate<'a> {
    posts: Vec<PostListEntry<'static>>,
    base_context: &'a BaseContext,
}

async fn index(base_context: BaseContext) -> cot::Result<Html> {
    let posts = get_homepage_entries();

    let index_template = IndexTemplate {
        posts,
        base_context: &base_context,
    };

    Ok(Html::new(index_template.render()?))
}

#[derive(Debug, Template)]
#[template(path = "archive.html")]
struct ArchiveTemplate<'a> {
    posts: Vec<PostListEntry<'static>>,
    base_context: &'a BaseContext,
}

async fn archive(base_context: BaseContext) -> cot::Result<Html> {
    let posts = get_archive_entries();

    let archive_template = ArchiveTemplate {
        posts,
        base_context: &base_context,
    };

    Ok(Html::new(archive_template.render()?))
}

#[derive(Debug, Template)]
#[template(path = "post.html")]
struct PostTemplate<'a> {
    post: &'a MdPage,
    other_languages: &'a [String],
    comments: &'a [DisplayComment],
    comment_form: &'a CommentFormState,
    comment_pending: bool,
    reply_to: Option<&'a DisplayComment>,
    base_context: &'a BaseContext,
}

#[derive(Debug, Default, Deserialize)]
struct PostQuery {
    /// Set to `pending` after posting a comment that awaits moderation.
    comment: Option<String>,
    /// The ID of the comment being replied to (when JavaScript is disabled).
    replytocom: Option<i64>,
}

async fn post(
    base_context: BaseContext,
    db: Database,
    Path(page): Path<String>,
    UrlQuery(query): UrlQuery<PostQuery>,
) -> cot::Result<Html> {
    let post = find_post(&page, None)?;
    render_post(
        &base_context,
        &db,
        post,
        &query,
        &CommentFormState::default(),
    )
    .await
}

async fn post_with_lang(
    base_context: BaseContext,
    db: Database,
    Path((page, lang)): Path<(String, String)>,
    UrlQuery(query): UrlQuery<PostQuery>,
) -> cot::Result<Html> {
    let post = find_post(&page, Some(&lang))?;
    render_post(
        &base_context,
        &db,
        post,
        &query,
        &CommentFormState::default(),
    )
    .await
}

fn find_post(page: &str, lang: Option<&str>) -> cot::Result<&'static MdPage> {
    let post_list = get_post_map().get(page).ok_or_else(NotFound::new)?;

    if Some(post_list[0].language.as_str()) == lang {
        // the default language should be returned only by the `post` route
        return Err(NotFound::new().into());
    }

    let post = if let Some(lang) = lang {
        post_list
            .iter()
            .find(|post| post.language == lang)
            .ok_or_else(NotFound::new)?
    } else {
        &post_list[0]
    };
    Ok(post)
}

/// Returns the URL path of a post (in the given language version).
fn post_path(urls: &Urls, post: &MdPage) -> cot::Result<String> {
    let is_default_language = get_post_map()
        .get(&post.link)
        .is_some_and(|posts| posts[0].language == post.language);
    let path = if is_default_language {
        cot::reverse!(urls, "post", page = post.link)?
    } else {
        cot::reverse!(
            urls,
            "post_with_lang",
            page = post.link,
            lang = post.language
        )?
    };
    Ok(path)
}

async fn render_post(
    base_context: &BaseContext,
    db: &Database,
    post: &MdPage,
    query: &PostQuery,
    comment_form: &CommentFormState,
) -> cot::Result<Html> {
    let other_languages = get_post_map()[&post.link]
        .iter()
        .filter(|other_post| other_post.language != post.language)
        .map(|post| post.language.clone())
        .collect::<Vec<_>>();
    let comments = approved_comments(db, &post.link, &post.language).await?;

    let reply_to_id = comment_form
        .value("parent")
        .parse::<i64>()
        .ok()
        .or(query.replytocom);
    let reply_to = reply_to_id.and_then(|id| comments.iter().find(|comment| comment.id == id));

    let template = PostTemplate {
        post,
        other_languages: &other_languages,
        comments: &comments,
        comment_form,
        comment_pending: query.comment.as_deref() == Some("pending"),
        reply_to,
        base_context,
    };

    Ok(Html::new(template.render()?))
}

async fn post_comment(
    base_context: BaseContext,
    db: Database,
    auth: Auth,
    Path((page, lang)): Path<(String, String)>,
    mut request: Request,
) -> cot::Result<Response> {
    let post = get_post_map()
        .get(&page)
        .and_then(|posts| posts.iter().find(|post| post.language == lang))
        .ok_or_else(NotFound::new)?;
    let path = post_path(&base_context.urls, post)?;
    let post_url = format!("{}{}", consts::WEBSITE_URL.trim_end_matches('/'), path);

    let target = CommentTarget {
        permalink: &post.link,
        language: &post.language,
        post_url: &post_url,
        comments_open: post.comments_open(),
    };
    match submit_comment(&mut request, &db, &auth, &target).await? {
        CommentSubmission::Invalid(form_state) => {
            let html =
                render_post(&base_context, &db, post, &PostQuery::default(), &form_state).await?;
            html.with_status(StatusCode::UNPROCESSABLE_ENTITY)
                .into_response()
        }
        CommentSubmission::Pending => {
            see_other(&format!("{path}?comment=pending#comments")).into_response()
        }
        CommentSubmission::Published { id } => {
            see_other(&format!("{path}#comment-{id}")).into_response()
        }
    }
}

fn see_other(location: &str) -> Response {
    Response::builder()
        .status(StatusCode::SEE_OTHER)
        .header(LOCATION, location)
        .body(Body::empty())
        .expect("valid response")
}

#[derive(Debug, Template)]
#[template(path = "md_page.html")]
struct MdPageTemplate<'a> {
    page: &'a MdPage,
    base_context: &'a BaseContext,
}

async fn about(base_context: BaseContext) -> cot::Result<Html> {
    let template = MdPageTemplate {
        page: &md_page!("about"),
        base_context: &base_context,
    };

    Ok(Html::new(template.render()?))
}

async fn privacy(base_context: BaseContext) -> cot::Result<Html> {
    let template = MdPageTemplate {
        page: &md_page!("privacy"),
        base_context: &base_context,
    };

    Ok(Html::new(template.render()?))
}

struct CotSiteApp;

#[async_trait]
impl App for CotSiteApp {
    fn name(&self) -> &'static str {
        "m4txblog"
    }

    async fn init(&self, context: &mut ProjectContext) -> cot::Result<()> {
        let akismet = match context.config().extra.get("akismet") {
            Some(config) => {
                let config: AkismetConfig = config.clone().try_into().map_err(|error| {
                    cot::Error::internal(format!("invalid `[akismet]` config: {error}"))
                })?;
                let akismet = Akismet::new(config);
                match akismet.verify_key().await {
                    Ok(()) => tracing::info!("Akismet API key verified"),
                    Err(instant_akismet::Error::Invalid(reason)) => {
                        tracing::error!("invalid Akismet API key: {reason}");
                    }
                    Err(error) => tracing::warn!("could not verify the Akismet API key: {error}"),
                }
                Some(akismet)
            }
            None => {
                tracing::warn!("Akismet is not configured; comments won't be checked for spam");
                None
            }
        };
        let notifications = match context.config().extra.get("comment_notifications") {
            Some(config) => Some(config.clone().try_into().map_err(|error| {
                cot::Error::internal(format!("invalid `[comment_notifications]` config: {error}"))
            })?),
            None => None,
        };
        comments::init(akismet, notifications);
        Ok(())
    }

    fn admin_model_managers(&self) -> Vec<Box<dyn AdminModelManager>> {
        vec![Box::new(CommentAdminManager::default())]
    }

    fn migrations(&self) -> Vec<Box<SyncDynMigration>> {
        cot::db::migrations::wrap_migrations(migrations::MIGRATIONS)
    }

    fn router(&self) -> Router {
        Router::with_urls([
            Route::with_handler_and_name("/", index, "index"),
            Route::with_handler_and_name("/feed/", blog_feed, "feed"),
            Route::with_handler_and_name("/about-me/", about, "about"),
            Route::with_handler_and_name("/privacy/", privacy, "privacy"),
            Route::with_handler_and_name("/archive/", archive, "archive"),
            Route::with_handler_and_name("/blog/{page}/", post, "post"),
            Route::with_handler_and_name("/blog/{page}/{lang}/", post_with_lang, "post_with_lang"),
            Route::with_handler_and_name(
                "/comments/{page}/{lang}/",
                method::post(post_comment),
                "post_comment",
            ),
        ])
    }

    fn static_files(&self) -> Vec<StaticFile> {
        let mut files = static_files!(
            "favicon.ico",
            "static/css/main.css",
            "static/js/color-modes.js",
            "static/images/favicon-32.png",
            "static/images/favicon-180.png",
            "static/images/favicon-192.png",
            "static/images/favicon-512.png",
            "static/images/site.webmanifest",
        );
        files.append(&mut static_files_dir!("static/images/blog/"));
        files
    }
}

struct CotSiteProject;

impl Project for CotSiteProject {
    fn cli_metadata(&self) -> CliMetadata {
        cot::cli::metadata!()
    }

    fn register_tasks(&self, cli: &mut Cli) {
        cli.add_task(CreateAdmin);
    }

    fn register_apps(&self, modules: &mut AppBuilder, _app_context: &RegisterAppsContext) {
        modules.register(DatabaseUserApp::new());
        modules.register(SessionApp::new());
        modules.register_with_views(AdminApp::new(), "/admin/");
        modules.register_with_views(CotSiteApp, "");
    }

    fn middlewares(&self, handler: RootHandlerBuilder, context: &MiddlewareContext) -> RootHandler {
        let legacy_redirects_config = Arc::new(LegacyRedirectsConfig::from_context(context));

        handler
            // Innermost, so that it doesn't redirect static files: e.g. `/admin/admin.css`
            // would otherwise be redirected to the `/admin/{model_name}/` route
            .middleware(TrailingSlashMiddleware::from_context(context))
            .middleware(StaticFilesMiddleware::from_context(context))
            .middleware(AuthMiddleware::new())
            .middleware(SessionMiddleware::from_context(context))
            .middleware(cot::middleware::LiveReloadMiddleware::from_context(context))
            // Added last, so that it's the outermost one and runs first
            .middleware(from_fn(move |request, next| {
                redirect_legacy(Arc::clone(&legacy_redirects_config), request, next)
            }))
            .build()
    }

    fn error_handler(&self) -> DynErrorPageHandler {
        DynErrorPageHandler::new(handle_error)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct CreateAdmin;

const CREATE_ADMIN_SUBCOMMAND: &str = "create-admin";

#[async_trait(?Send)]
impl CliTask for CreateAdmin {
    fn subcommand(&self) -> Command {
        Command::new(CREATE_ADMIN_SUBCOMMAND)
            .about("Creates a new superuser; the password is read from the standard input")
            .arg(Arg::new("username").required(true))
    }

    async fn execute(
        &mut self,
        matches: &ArgMatches,
        bootstrapper: Bootstrapper<WithConfig>,
    ) -> cot::Result<()> {
        let username = matches
            .get_one::<String>("username")
            .expect("username is required");

        eprint!("Password: ");
        let mut password = String::new();
        std::io::stdin()
            .read_line(&mut password)
            .map_err(cot::Error::internal)?;
        let password = password.trim_end_matches(['\r', '\n']);
        if password.is_empty() {
            return Err(cot::Error::internal("the password cannot be empty"));
        }

        let bootstrapper = bootstrapper.boot().await?;
        let context = bootstrapper.context();
        let db = context.database();

        // Make sure the user table exists, even if the server has never been
        // run
        let mut migrations = Vec::new();
        for app in context.apps() {
            migrations.extend(app.migrations());
        }
        MigrationEngine::new(migrations)?.run(db).await?;

        DatabaseUser::create_user(db, username, Password::new(password)).await?;
        eprintln!("Created user {username:?}");
        Ok(())
    }
}

async fn handle_error(
    base_context: BaseContext,
    error: RequestError,
) -> cot::Result<impl IntoResponse> {
    #[derive(Debug, Template)]
    #[template(path = "error.html")]
    struct ErrorTemplate<'a> {
        base_context: &'a BaseContext,
        error: RequestError,
    }

    let status_code = error.status_code();
    let error_template = ErrorTemplate {
        base_context: &base_context,
        error,
    };
    let rendered = error_template.render()?;

    Ok(Html::new(rendered).with_status(status_code))
}

#[cot::main]
fn main() -> impl Project {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .finish()
        .init();

    CotSiteProject
}
