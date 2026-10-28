//! Spam checking with [Akismet](https://akismet.com/developers/).

use std::time::Duration;

use instant_akismet::{AkismetClient, AkismetOptions, CheckResult, Comment, CommentType, Error};
use serde::Deserialize;

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

#[derive(Debug)]
pub struct Akismet {
    client: AkismetClient,
    is_test: bool,
}

impl Akismet {
    pub fn new(config: AkismetConfig) -> Self {
        let http_client = reqwest::Client::builder()
            .timeout(Duration::from_secs(10))
            .build()
            .expect("failed to build the HTTP client");
        let options = AkismetOptions {
            user_agent: concat!("m4txblog/", env!("CARGO_PKG_VERSION")).to_owned(),
            ..AkismetOptions::default()
        };
        Self {
            client: AkismetClient::new(config.blog, config.api_key, http_client, options),
            is_test: config.is_test,
        }
    }

    /// Checks whether the API key is valid.
    pub async fn verify_key(&self) -> Result<(), Error> {
        self.client.verify_key().await
    }

    /// Asks Akismet whether a comment is spam.
    pub async fn check_comment(&self, check: &CommentCheck<'_>) -> Result<CheckResult, Error> {
        let comment_type = if check.is_reply {
            CommentType::Reply
        } else {
            CommentType::Comment
        };
        let mut comment = Comment::new(&self.client.blog, check.user_ip)
            .user_agent(check.user_agent)
            .referrer(check.referrer)
            .permalink(check.permalink)
            .comment_type(comment_type)
            .comment_author(check.author)
            .comment_author_email(check.author_email)
            .comment_author_url(check.author_url)
            .comment_content(check.content)
            .blog_lang(check.language)
            .blog_charset("UTF-8");
        if self.is_test {
            comment = comment.is_test(true);
        }
        self.client.check_comment(comment).await
    }
}
