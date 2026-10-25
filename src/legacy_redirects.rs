//! Redirects from the old WordPress blog at m4tx.pl.
//!
//! The old blog was a WordPress multisite: the Polish site was served at `/`
//! (with posts at `/blog/YYYY/MM/slug/`) and the English one at `/en/` (with
//! posts at `/en/YYYY/MM/slug/`). The mapping between the old posts and the new
//! ones is stored in `legacy_redirects.toml`, which is generated from the old
//! database by `scripts/generate_legacy_redirects.py`.
//!
//! Requests whose `Host` is one of the configured legacy hosts are always
//! redirected to the new domain: either to the new location of the
//! post/page, or to the same path if there's no known equivalent (which then
//! typically results in a 404 on the new domain).

use std::collections::HashMap;
use std::sync::{Arc, LazyLock};

use cot::http::header::{HOST, LOCATION};
use cot::middleware::Next;
use cot::project::MiddlewareContext;
use cot::request::{Request, RequestExt};
use cot::response::{Response, ResponseExt};
use cot::{Body, StatusCode};
use percent_encoding::percent_decode_str;
use serde::Deserialize;

use crate::posts::get_post_map;

const APP_NAME: &str = "m4txblog";

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

    let target = resolve(request.uri().path(), request.uri().query());
    let path = match target {
        Target::Gone => {
            return Ok(Response::builder()
                .status(StatusCode::GONE)
                .body(Body::empty())
                .expect("valid response"));
        }
        Target::SamePath => request
            .uri()
            .path_and_query()
            .map_or_else(|| "/".to_owned(), ToString::to_string),
        target => target_path(&request, &target)?,
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
    page: Vec<LegacyPage>,
}

#[derive(Debug, Deserialize)]
struct LegacyPost {
    site: Site,
    wp_id: u64,
    slug: String,
    permalink: String,
    language: String,
}

#[derive(Debug, Deserialize)]
struct LegacyPage {
    site: Site,
    wp_id: u64,
    path: String,
    route: String,
}

#[derive(Debug)]
struct LegacyIndex {
    posts_by_slug: HashMap<(Site, String), Target>,
    posts_by_id: HashMap<(Site, u64), Target>,
    pages_by_path: HashMap<(Site, String), Target>,
    pages_by_id: HashMap<(Site, u64), Target>,
}

static LEGACY_INDEX: LazyLock<LegacyIndex> = LazyLock::new(|| {
    let data: LegacyData = toml::from_str(include_str!("../legacy_redirects.toml"))
        .expect("invalid legacy_redirects.toml");

    let mut index = LegacyIndex {
        posts_by_slug: HashMap::new(),
        posts_by_id: HashMap::new(),
        pages_by_path: HashMap::new(),
        pages_by_id: HashMap::new(),
    };
    for post in data.post {
        let target = Target::Post {
            permalink: post.permalink,
            language: post.language,
        };
        index
            .posts_by_slug
            .insert((post.site, normalize(&post.slug)), target.clone());
        index.posts_by_id.insert((post.site, post.wp_id), target);
    }
    for page in data.page {
        let target = Target::Route(page.route);
        index
            .pages_by_path
            .insert((page.site, normalize(&page.path)), target.clone());
        index.pages_by_id.insert((page.site, page.wp_id), target);
    }
    index
});

#[derive(Debug, Clone, PartialEq, Eq)]
enum Target {
    /// A blog post in the given language.
    Post { permalink: String, language: String },
    /// A named route of the blog app (with no parameters).
    Route(String),
    /// The same path on the new domain.
    SamePath,
    /// WordPress-specific endpoints that don't make sense anymore.
    Gone,
}

impl Target {
    fn route(name: &str) -> Self {
        Self::Route(name.to_owned())
    }
}

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

fn query_param<'a>(query: Option<&'a str>, name: &str) -> Option<&'a str> {
    query?
        .split('&')
        .filter_map(|pair| pair.split_once('=').or(Some((pair, ""))))
        .find(|(key, _)| *key == name)
        .map(|(_, value)| value)
}

/// Finds out where an old URL should be redirected to.
fn resolve(path: &str, query: Option<&str>) -> Target {
    let index = &*LEGACY_INDEX;

    let normalized = normalize(path);
    let (site, rest) = match normalized.strip_prefix("en") {
        Some(rest) if rest.is_empty() || rest.starts_with('/') => {
            (Site::En, rest.trim_start_matches('/'))
        }
        _ => (Site::Pl, normalized.as_str()),
    };
    let segments: Vec<&str> = rest.split('/').filter(|s| !s.is_empty()).collect();

    // `/?p=123`, `/?page_id=123`, `/?feed=rss2`
    if segments.is_empty() {
        let id = |name| query_param(query, name).and_then(|id| id.parse::<u64>().ok());
        if let Some(target) = id("p").and_then(|id| index.posts_by_id.get(&(site, id))) {
            return target.clone();
        }
        if let Some(target) = id("page_id").and_then(|id| index.pages_by_id.get(&(site, id))) {
            return target.clone();
        }
        if query_param(query, "feed").is_some() {
            return Target::route("feed");
        }
        return Target::route("index");
    }

    // Posts are matched by slug only; any trailing segments (`feed/`,
    // `comment-page-2/`, `amp/`, ...) are ignored
    let post_slug = match (site, segments.as_slice()) {
        (Site::Pl, ["blog", year, month, slug, ..]) | (Site::En, [year, month, slug, ..])
            if is_year(year) && is_month(month) =>
        {
            Some(*slug)
        }
        _ => None,
    };
    if let Some(slug) = post_slug {
        if let Some(target) = index.posts_by_slug.get(&(site, slug.to_owned())) {
            return target.clone();
        }
        // Someone might have guessed the URL using the new slug
        let language = site.language();
        let has_post = get_post_map()
            .get(slug)
            .is_some_and(|posts| posts.iter().any(|post| post.language == language));
        return if has_post {
            Target::Post {
                permalink: slug.to_owned(),
                language: language.to_owned(),
            }
        } else {
            Target::SamePath
        };
    }

    match (site, segments.as_slice()) {
        (_, ["feed", ..] | ["comments", "feed", ..]) | (Site::Pl, ["blog", "feed", ..]) => {
            Target::route("feed")
        }
        (Site::Pl, ["blog"]) => Target::route("index"),
        (Site::Pl, ["blog", "kategorie" | "tagi" | "page", ..])
        | (Site::En, ["category" | "tag", ..])
        | (_, ["author" | "page", ..]) => Target::route("archive"),
        (Site::Pl, ["blog", year] | ["blog", year, _]) | (Site::En, [year] | [year, _])
            if is_year(year) =>
        {
            Target::route("archive")
        }
        (_, ["wp-admin" | "wp-includes" | "wp-json", ..])
        | (_, ["wp-login.php" | "xmlrpc.php" | "wp-cron.php" | "wp-signup.php"]) => Target::Gone,
        _ => index
            .pages_by_path
            .get(&(site, segments.join("/")))
            .cloned()
            .unwrap_or(Target::SamePath),
    }
}

fn target_path(request: &Request, target: &Target) -> cot::Result<String> {
    let path = match target {
        Target::Post {
            permalink,
            language,
        } => {
            let is_default_language = get_post_map()
                .get(permalink)
                .is_some_and(|posts| posts[0].language == *language);
            if is_default_language {
                cot::reverse!(request, "m4txblog:post", page = permalink)?
            } else {
                cot::reverse!(
                    request,
                    "m4txblog:post_with_lang",
                    page = permalink,
                    lang = language
                )?
            }
        }
        Target::Route(name) => {
            request
                .router()
                .reverse(Some(APP_NAME), name, &cot::reverse_param_map!())?
        }
        Target::SamePath | Target::Gone => unreachable!("handled by the caller"),
    };
    Ok(path)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn post(permalink: &str, language: &str) -> Target {
        Target::Post {
            permalink: permalink.to_owned(),
            language: language.to_owned(),
        }
    }

    #[test]
    fn every_post_target_exists() {
        let post_map = get_post_map();
        for target in LEGACY_INDEX.posts_by_id.values() {
            let Target::Post {
                permalink,
                language,
            } = target
            else {
                panic!("unexpected target {target:?}");
            };
            let posts = post_map
                .get(permalink)
                .unwrap_or_else(|| panic!("no post with permalink {permalink:?}"));
            assert!(
                posts.iter().any(|post| post.language == *language),
                "no {language:?} version of {permalink:?}"
            );
        }
        assert_eq!(LEGACY_INDEX.posts_by_id.len(), 102);
    }

    #[test]
    fn polish_posts() {
        assert_eq!(
            resolve("/blog/2011/08/kurs-gtk-rozdzial-10/", None),
            post("kurs-gtk-rozdzial-10", "pl"),
            "the new slug should work, too"
        );
        assert_eq!(
            resolve("/blog/2011/08/kurs-gtk-%e2%80%93-rozdzial-10/", None),
            post("kurs-gtk-rozdzial-10", "pl")
        );
        assert_eq!(
            resolve("/blog/2011/08/kurs-gtk-%E2%80%93-rozdzial-10", None),
            post("kurs-gtk-rozdzial-10", "pl")
        );
        assert_eq!(
            resolve("/blog/2011/08/kurs-gtk-–-rozdzial-10/comment-page-1/", None),
            post("kurs-gtk-rozdzial-10", "pl")
        );
        assert_eq!(
            resolve("/blog/2010/09/nigdy-wiecej-gadu-gadu/", None),
            post("nigdy-wiecej-gadu-gadu", "pl")
        );
        assert_eq!(
            resolve(
                "/blog/2013/01/java-konwersja-obrazkow-swt-na-awt-i-vice-versa-z-przezroczystoscia/",
                None
            ),
            post(
                "java-swt-to-awt-and-vice-versa-image-conversion-with-transparency-support",
                "pl"
            )
        );
        assert_eq!(
            resolve("/blog/2012/02/duplicate-lines-finder-2-0/", None),
            post("new-application-duplicate-lines-finder-2-0", "pl")
        );
        assert_eq!(
            resolve("/", Some("p=305")),
            post("nigdy-wiecej-gadu-gadu", "pl")
        );
    }

    #[test]
    fn english_posts() {
        assert_eq!(
            resolve("/en/2011/02/hello-world/", None),
            post("hello-world-2", "en")
        );
        assert_eq!(
            resolve(
                "/en/2013/01/java-swt-to-awt-and-vice-versa-image-conversion-with-transparency-support/",
                None
            ),
            post(
                "java-swt-to-awt-and-vice-versa-image-conversion-with-transparency-support",
                "en"
            )
        );
        assert_eq!(resolve("/en/", Some("p=11")), post("hello-world-2", "en"));
        // This one only exists in the Polish blog
        assert_eq!(
            resolve("/en/2010/09/nigdy-wiecej-gadu-gadu/", None),
            Target::SamePath
        );
    }

    #[test]
    fn pages() {
        assert_eq!(resolve("/o-mnie/", None), Target::route("about"));
        assert_eq!(resolve("/en/about-me/", None), Target::route("about"));
        assert_eq!(resolve("/cookies/", None), Target::route("privacy"));
        assert_eq!(resolve("/", Some("page_id=2")), Target::route("about"));
        assert_eq!(
            resolve("/en/go-to-m4txblog-in-polish/", None),
            Target::route("index")
        );
        // Old app pages don't have an equivalent
        assert_eq!(
            resolve("/aplikacje-i-gry/komputerowe/smsprice/", None),
            Target::SamePath
        );
    }

    #[test]
    fn listings_and_feeds() {
        assert_eq!(resolve("/", None), Target::route("index"));
        assert_eq!(resolve("/en/", None), Target::route("index"));
        assert_eq!(resolve("/en", None), Target::route("index"));
        assert_eq!(resolve("/blog/", None), Target::route("index"));
        assert_eq!(resolve("/feed/", None), Target::route("feed"));
        assert_eq!(resolve("/feed/atom/", None), Target::route("feed"));
        assert_eq!(resolve("/en/feed/", None), Target::route("feed"));
        assert_eq!(resolve("/comments/feed/", None), Target::route("feed"));
        assert_eq!(resolve("/", Some("feed=rss2")), Target::route("feed"));
        assert_eq!(
            resolve("/blog/kategorie/kurs-gtkplus/", None),
            Target::route("archive")
        );
        assert_eq!(resolve("/blog/tagi/java/", None), Target::route("archive"));
        assert_eq!(resolve("/en/tag/java/", None), Target::route("archive"));
        assert_eq!(resolve("/blog/2011/", None), Target::route("archive"));
        assert_eq!(resolve("/blog/2011/07/", None), Target::route("archive"));
        assert_eq!(resolve("/en/2013/01/", None), Target::route("archive"));
        assert_eq!(resolve("/page/3/", None), Target::route("archive"));
    }

    #[test]
    fn wordpress_endpoints() {
        assert_eq!(resolve("/wp-login.php", None), Target::Gone);
        assert_eq!(resolve("/xmlrpc.php", None), Target::Gone);
        assert_eq!(resolve("/wp-admin/edit.php", None), Target::Gone);
        assert_eq!(resolve("/en/wp-admin/", None), Target::Gone);
    }
}
