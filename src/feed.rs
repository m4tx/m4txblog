use std::sync::LazyLock;

use atom_syndication::{
    ContentBuilder, Entry, EntryBuilder, Feed, FeedBuilder, GeneratorBuilder, LinkBuilder, Person,
    PersonBuilder, TextBuilder,
};
use cot::response::{Response, ResponseExt};
use cot::{Body, StatusCode};
use m4txblog_common::md_pages::MdPage;

use crate::posts::get_unarchived_post_map;

static BLOG_FEED: LazyLock<Feed> = LazyLock::new(|| {
    let posts: Vec<_> = get_unarchived_post_map()
        .clone()
        .into_values()
        .map(|post_list| post_list.into_iter().next().unwrap())
        .collect();
    let builder = BlogFeedBuilder::new(&posts);
    builder.build()
});

pub async fn blog_feed() -> cot::Result<Response> {
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/atom+xml; charset=utf-8")
        .body(Body::fixed(BLOG_FEED.to_string()))
        .expect("Failed to build the blog feed"))
}

#[derive(Debug)]
struct BlogFeedBuilder<'a> {
    posts: &'a [MdPage],
}

const WEBSITE_URL: &str = "https://mackow.ski/";
const LANG: &str = "en";

impl<'a> BlogFeedBuilder<'a> {
    fn new(posts: &'a [MdPage]) -> Self {
        assert!(!posts.is_empty());
        Self { posts }
    }

    fn build(&self) -> Feed {
        let mut feed = FeedBuilder::default();
        feed.title("Mateusz Maćkowski's blog")
            .id(WEBSITE_URL)
            .base(WEBSITE_URL.to_owned())
            .link(
                LinkBuilder::default()
                    .href(format!("{WEBSITE_URL}feed/"))
                    .rel("self")
                    .mime_type("application/atom+xml".to_owned())
                    .build(),
            )
            .link(
                LinkBuilder::default()
                    .href(WEBSITE_URL.to_owned())
                    .rel("alternate")
                    .mime_type("text/html".to_owned())
                    .build(),
            )
            .updated(self.last_updated())
            .authors(Self::get_authors())
            .generator(
                GeneratorBuilder::default()
                    .value(crate::consts::GENERATOR)
                    .uri("https://github.com/m4tx/m4txblog".to_owned())
                    .build(),
            )
            .rights(
                TextBuilder::default()
                    .value("© 2010-2025 Mateusz Maćkowski. All texts on the CC-BY-ND license.")
                    .lang(LANG.to_owned())
                    .build(),
            )
            .lang(LANG.to_owned())
            .logo(format!("{WEBSITE_URL}static/images/favicon-512.png"));

        for post in &self.posts[..self.posts.len().min(20)] {
            feed.entry(Self::post_to_entry(post));
        }

        feed.build()
    }

    fn post_to_entry(post: &MdPage) -> Entry {
        let url = format!("{WEBSITE_URL}blog/{}/", post.link);
        EntryBuilder::default()
            .title(&*post.title)
            .id(url.clone())
            .link(
                LinkBuilder::default()
                    .href(url)
                    .rel("alternate")
                    .mime_type("text/html".to_owned())
                    .build(),
            )
            .authors(Self::get_authors())
            .updated(post.date)
            .published(post.date)
            .content(
                ContentBuilder::default()
                    .lang(LANG.to_owned())
                    .content_type("html".to_owned())
                    .value(post.content_html.to_owned())
                    .build(),
            )
            .build()
    }

    fn get_authors() -> [Person; 1] {
        [PersonBuilder::default()
            .name("Mateusz Maćkowski")
            .email("mateusz@mackowski.org".to_owned())
            .uri(WEBSITE_URL.to_owned())
            .build()]
    }

    fn last_updated(&self) -> chrono::DateTime<chrono::FixedOffset> {
        self.posts
            .iter()
            .map(|post| post.date)
            .max()
            .expect("no posts")
    }
}
