//! A minimal client for the [Akismet](https://akismet.com/developers/) spam
//! checking API.

use std::time::Duration;

use serde::Deserialize;

const API_URL: &str = "https://rest.akismet.com/1.1";

/// The `[akismet]` section of the project config.
#[derive(Debug, Clone, Deserialize)]
pub struct AkismetConfig {
    /// The Akismet API key.
    api_key: String,
    /// The front page of the blog, e.g. `"https://mackow.ski/"`.
    blog: String,
    /// Whether to mark the requests as test ones (so that they don't affect
    /// Akismet's training).
    #[serde(default)]
    is_test: bool,
}

/// What Akismet thinks about a comment.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Verdict {
    /// Not spam.
    Ham,
    /// Spam.
    Spam,
    /// Blatant spam that can be discarded without even storing it.
    Discard,
}

/// The data about a comment that is sent to Akismet.
#[derive(Debug, Clone)]
pub struct CommentCheck<'a> {
    pub user_ip: &'a str,
    pub user_agent: &'a str,
    pub referrer: &'a str,
    pub permalink: &'a str,
    pub is_reply: bool,
    pub author: &'a str,
    pub author_email: &'a str,
    pub author_url: &'a str,
    pub content: &'a str,
    pub language: &'a str,
}

#[derive(Debug, Clone)]
pub struct Akismet {
    client: reqwest::Client,
    config: AkismetConfig,
}

impl Akismet {
    pub fn new(config: AkismetConfig) -> Self {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(10))
            .user_agent(concat!("m4txblog/", env!("CARGO_PKG_VERSION")))
            .build()
            .expect("failed to build the HTTP client");
        Self { client, config }
    }

    /// Checks whether the API key is valid.
    pub async fn verify_key(&self) -> Result<bool, reqwest::Error> {
        let response = self
            .client
            .post(format!("{API_URL}/verify-key"))
            .form(&[
                ("api_key", self.config.api_key.as_str()),
                ("blog", self.config.blog.as_str()),
            ])
            .send()
            .await?
            .error_for_status()?
            .text()
            .await?;
        Ok(response == "valid")
    }

    /// Asks Akismet whether a comment is spam.
    pub async fn check_comment(&self, comment: &CommentCheck<'_>) -> Result<Verdict, AkismetError> {
        let comment_type = if comment.is_reply { "reply" } else { "comment" };
        let mut params = vec![
            ("api_key", self.config.api_key.as_str()),
            ("blog", self.config.blog.as_str()),
            ("user_ip", comment.user_ip),
            ("user_agent", comment.user_agent),
            ("referrer", comment.referrer),
            ("permalink", comment.permalink),
            ("comment_type", comment_type),
            ("comment_author", comment.author),
            ("comment_author_email", comment.author_email),
            ("comment_author_url", comment.author_url),
            ("comment_content", comment.content),
            ("blog_lang", comment.language),
            ("blog_charset", "UTF-8"),
        ];
        if self.config.is_test {
            params.push(("is_test", "true"));
        }

        let response = self
            .client
            .post(format!("{API_URL}/comment-check"))
            .form(&params)
            .send()
            .await?
            .error_for_status()?;
        let discard = response
            .headers()
            .get("X-akismet-pro-tip")
            .is_some_and(|value| value == "discard");
        let debug_help = response
            .headers()
            .get("X-akismet-debug-help")
            .and_then(|value| value.to_str().ok())
            .map(ToOwned::to_owned);

        match response.text().await?.as_str() {
            "true" if discard => Ok(Verdict::Discard),
            "true" => Ok(Verdict::Spam),
            "false" => Ok(Verdict::Ham),
            other => Err(AkismetError::InvalidResponse(format!(
                "{other} ({})",
                debug_help.as_deref().unwrap_or("no debug help")
            ))),
        }
    }
}

#[derive(Debug)]
pub enum AkismetError {
    Http(reqwest::Error),
    InvalidResponse(String),
}

impl std::fmt::Display for AkismetError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Http(error) => write!(f, "HTTP error: {error}"),
            Self::InvalidResponse(response) => write!(f, "invalid response: {response}"),
        }
    }
}

impl std::error::Error for AkismetError {}

impl From<reqwest::Error> for AkismetError {
    fn from(error: reqwest::Error) -> Self {
        Self::Http(error)
    }
}
