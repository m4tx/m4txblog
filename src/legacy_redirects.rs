//! Redirects from the old WordPress blog at m4tx.pl.
//!
//! The old blog was a WordPress multisite: the Polish site was served at `/`
//! (with posts at `/blog/YYYY/MM/slug/`) and the English one at `/en/` (with
//! posts at `/en/YYYY/MM/slug/`). The mapping between the old posts and the new
//! ones is stored in `legacy_redirects.toml`, which is generated from the old
//! database by `scripts/generate_legacy_redirects.py`.
//!
//! Requests whose `Host` is one of the configured legacy hosts are always
//! redirected to the new domain: either to the new location of the post, or
//! to the same path for anything that isn't a post URL.

use std::collections::HashMap;
use std::sync::{Arc, LazyLock};

use cot::http::header::{HOST, LOCATION};
use cot::middleware::Next;
use cot::project::MiddlewareContext;
use cot::request::Request;
use cot::response::{Response, ResponseExt};
use cot::{Body, StatusCode};
use percent_encoding::percent_decode_str;
use serde::Deserialize;

use crate::posts::get_post_map;

/// The `[legacy_redirects]` section of the project config.
#[derive(Debug, Clone, Deserialize)]
pub struct LegacyRedirectsConfig {
    /// Hosts that should be redirected, e.g. `["m4tx.pl", "www.m4tx.pl"]`.
    hosts: Vec<String>,
    /// The base URL of the new blog, e.g. `"https://mackow.ski"`.
    target: String,
}

impl LegacyRedirectsConfig {
    /// Reads the config from the project's `[legacy_redirects]` section.
    ///
    /// If the section is missing, the redirects are disabled.
    ///
    /// # Panics
    ///
    /// Panics if the section exists but is invalid.
    pub fn from_context(context: &MiddlewareContext) -> Self {
        let Some(value) = context.config().extra.get("legacy_redirects") else {
            return Self {
                hosts: Vec::new(),
                target: String::new(),
            };
        };
        let config: Self = value
            .clone()
            .try_into()
            .expect("invalid `[legacy_redirects]` config");
        Self {
            hosts: config
                .hosts
                .into_iter()
                .map(|host| host.to_ascii_lowercase())
                .collect(),
            target: config.target.trim_end_matches('/').to_owned(),
        }
    }
}

/// Middleware function that redirects requests to the legacy hosts.
pub async fn redirect_legacy(
    config: Arc<LegacyRedirectsConfig>,
    request: Request,
    next: Next,
) -> cot::Result<Response> {
    let host = request
        .headers()
        .get(HOST)
        .and_then(|host| host.to_str().ok())
        .map(|host| host.split(':').next().unwrap_or(host).to_ascii_lowercase());
    let Some(host) = host else {
        return next.run(request).await;
    };
    if !config.hosts.contains(&host) {
        return next.run(request).await;
    }

    let path = match resolve(request.uri().path()) {
        Some(post) => post_path(&request, &post)?,
        None => request
            .uri()
            .path_and_query()
            .map_or_else(|| "/".to_owned(), ToString::to_string),
    };

    Ok(Response::builder()
        .status(StatusCode::MOVED_PERMANENTLY)
        .header(LOCATION, format!("{}{path}", config.target))
        .body(Body::empty())
        .expect("valid response"))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize)]
#[serde(rename_all = "lowercase")]
enum Site {
    /// The Polish blog, served at `/`.
    Pl,
    /// The English blog, served at `/en/`.
    En,
}

impl Site {
    fn language(self) -> &'static str {
        match self {
            Self::Pl => "pl",
            Self::En => "en",
        }
    }
}

#[derive(Debug, Deserialize)]
struct LegacyData {
    post: Vec<LegacyPost>,
}

#[derive(Debug, Deserialize)]
struct LegacyPost {
    site: Site,
    slug: String,
    permalink: String,
    language: String,
}

/// A blog post in the given language.
#[derive(Debug, Clone, PartialEq, Eq)]
struct PostTarget {
    permalink: String,
    language: String,
}

/// The new posts, keyed by the site and the slug of the old post.
static LEGACY_POSTS: LazyLock<HashMap<(Site, String), PostTarget>> = LazyLock::new(|| {
    let data: LegacyData = toml::from_str(include_str!("../legacy_redirects.toml"))
        .expect("invalid legacy_redirects.toml");

    data.post
        .into_iter()
        .map(|post| {
            let target = PostTarget {
                permalink: post.permalink,
                language: post.language,
            };
            ((post.site, normalize(&post.slug)), target)
        })
        .collect()
});

/// Percent-decodes and lowercases a path (WordPress slugs are lowercase, but
/// percent-encoded characters can be in either case).
fn normalize(path: &str) -> String {
    percent_decode_str(path)
        .decode_utf8_lossy()
        .trim_matches('/')
        .to_lowercase()
}

fn is_year(segment: &str) -> bool {
    segment.len() == 4 && segment.bytes().all(|b| b.is_ascii_digit())
}

fn is_month(segment: &str) -> bool {
    segment.len() == 2 && segment.bytes().all(|b| b.is_ascii_digit())
}

/// Finds the post an old post URL should be redirected to.
///
/// Returns `None` if the path isn't a known post URL.
fn resolve(path: &str) -> Option<PostTarget> {
    let normalized = normalize(path);
    let segments: Vec<&str> = normalized.split('/').filter(|s| !s.is_empty()).collect();

    // Posts are matched by slug only; any trailing segments (`feed/`,
    // `comment-page-2/`, `amp/`, ...) are ignored
    let (site, slug) = match segments.as_slice() {
        ["blog", year, month, slug, ..] if is_year(year) && is_month(month) => (Site::Pl, *slug),
        ["en", year, month, slug, ..] if is_year(year) && is_month(month) => (Site::En, *slug),
        _ => return None,
    };
    if let Some(target) = LEGACY_POSTS.get(&(site, slug.to_owned())) {
        return Some(target.clone());
    }

    // Someone might have guessed the URL using the new slug
    let language = site.language();
    let has_post = get_post_map()
        .get(slug)
        .is_some_and(|posts| posts.iter().any(|post| post.language == language));
    has_post.then(|| PostTarget {
        permalink: slug.to_owned(),
        language: language.to_owned(),
    })
}

fn post_path(request: &Request, post: &PostTarget) -> cot::Result<String> {
    let is_default_language = get_post_map()
        .get(&post.permalink)
        .is_some_and(|posts| posts[0].language == post.language);
    let path = if is_default_language {
        cot::reverse!(request, "m4txblog:post", page = post.permalink)?
    } else {
        cot::reverse!(
            request,
            "m4txblog:post_with_lang",
            page = post.permalink,
            lang = post.language
        )?
    };
    Ok(path)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn post(permalink: &str, language: &str) -> Option<PostTarget> {
        Some(PostTarget {
            permalink: permalink.to_owned(),
            language: language.to_owned(),
        })
    }

    #[test]
    fn every_post_target_exists() {
        let post_map = get_post_map();
        for PostTarget {
            permalink,
            language,
        } in LEGACY_POSTS.values()
        {
            let posts = post_map
                .get(permalink)
                .unwrap_or_else(|| panic!("no post with permalink {permalink:?}"));
            assert!(
                posts.iter().any(|post| post.language == *language),
                "no {language:?} version of {permalink:?}"
            );
        }
        assert_eq!(LEGACY_POSTS.len(), 102);
    }

    #[test]
    fn polish_posts() {
        assert_eq!(
            resolve("/blog/2011/08/kurs-gtk-rozdzial-10/"),
            post("kurs-gtk-rozdzial-10", "pl"),
            "the new slug should work, too"
        );
        assert_eq!(
            resolve("/blog/2011/08/kurs-gtk-%e2%80%93-rozdzial-10/"),
            post("kurs-gtk-rozdzial-10", "pl")
        );
        assert_eq!(
            resolve("/blog/2011/08/kurs-gtk-%E2%80%93-rozdzial-10"),
            post("kurs-gtk-rozdzial-10", "pl")
        );
        assert_eq!(
            resolve("/blog/2011/08/kurs-gtk-–-rozdzial-10/comment-page-1/"),
            post("kurs-gtk-rozdzial-10", "pl")
        );
        assert_eq!(
            resolve("/blog/2010/09/nigdy-wiecej-gadu-gadu/"),
            post("nigdy-wiecej-gadu-gadu", "pl")
        );
        assert_eq!(
            resolve(
                "/blog/2013/01/java-konwersja-obrazkow-swt-na-awt-i-vice-versa-z-przezroczystoscia/"
            ),
            post(
                "java-swt-to-awt-and-vice-versa-image-conversion-with-transparency-support",
                "pl"
            )
        );
        assert_eq!(
            resolve("/blog/2012/02/duplicate-lines-finder-2-0/"),
            post("new-application-duplicate-lines-finder-2-0", "pl")
        );
    }

    #[test]
    fn english_posts() {
        assert_eq!(
            resolve("/en/2011/02/hello-world/"),
            post("hello-world-2", "en")
        );
        assert_eq!(
            resolve(
                "/en/2013/01/java-swt-to-awt-and-vice-versa-image-conversion-with-transparency-support/"
            ),
            post(
                "java-swt-to-awt-and-vice-versa-image-conversion-with-transparency-support",
                "en"
            )
        );
        // This one only exists in the Polish blog
        assert_eq!(resolve("/en/2010/09/nigdy-wiecej-gadu-gadu/"), None);
    }

    #[test]
    fn other_urls() {
        for path in [
            "/",
            "/en/",
            "/blog/",
            "/feed/",
            "/o-mnie/",
            "/blog/kategorie/kurs-gtkplus/",
            "/blog/tagi/java/",
            "/blog/2011/07/",
            "/blog/2099/01/missing/",
            "/wp-login.php",
            "/wp-admin/edit.php",
            "/wp-content/uploads/2011/03/amarok_screen.jpeg",
        ] {
            assert_eq!(resolve(path), None, "{path}");
        }
    }
}
