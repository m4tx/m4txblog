use std::sync::LazyLock;

use indexmap::IndexMap;
use m4txblog_common::md_pages::MdPage;
use m4txblog_macros::md_page;

static POSTS: LazyLock<IndexMap<String, MdPage>> = LazyLock::new(|| {
    let posts = vec![md_page!(
        "2025-02-18-welcome-cot-the-rust-web-framework-for-lazy-developers"
    )];

    posts
        .into_iter()
        .map(|post| (post.link.clone(), post))
        .collect()
});

pub fn get_posts() -> &'static IndexMap<String, MdPage> {
    &POSTS
}
