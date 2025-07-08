use std::io::Write;
use std::sync::Mutex;

use comrak::adapters::{HeadingAdapter, HeadingMeta};
use comrak::nodes::Sourcepos;
use serde::Deserialize;

#[derive(Debug, Clone)]
pub struct MdPage {
    pub link: String,
    pub title: String,
    pub date: chrono::DateTime<chrono::FixedOffset>,
    pub content_html: String,
    pub language: String,
    pub sections: Vec<Section>,
    pub category: Option<String>,
    pub tags: Vec<String>,
    pub reddit_link: Option<String>,
    pub hackernews_link: Option<String>,
}

impl MdPage {
    pub fn is_archived(&self) -> bool {
        const ARCHIVE_YEARS: i64 = 5;

        self.years_ago() >= ARCHIVE_YEARS
    }

    pub fn years_ago(&self) -> i64 {
        let time_delta = chrono::Utc::now().fixed_offset() - self.date;
        time_delta.num_days() / 365
    }

    pub fn accepts_comments(&self) -> bool {
        self.reddit_link.is_some() || self.hackernews_link.is_some()
    }
}

impl From<&MdPage> for MdPageLink {
    fn from(value: &MdPage) -> Self {
        Self {
            link: value.link.clone(),
            title: value.title.clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct MdPageLink {
    pub link: String,
    pub title: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FrontMatter {
    pub title: String,
    pub permalink: String,
    pub date: chrono::DateTime<chrono::FixedOffset>,
    pub language: Option<String>,
    pub category: Option<String>,
    pub tags: Option<Vec<String>>,
    pub reddit_link: Option<String>,
    pub hackernews_link: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Section {
    pub level: u8,
    pub title: String,
    pub anchor: String,
    pub children: Vec<Self>,
}

#[derive(Debug)]
pub struct MdPageHeadingAdapter {
    pub anchorizer: Mutex<comrak::Anchorizer>,
    pub sections: Mutex<Vec<Section>>,
}

impl HeadingAdapter for MdPageHeadingAdapter {
    fn enter(
        &self,
        output: &mut dyn Write,
        heading: &HeadingMeta,
        _sourcepos: Option<Sourcepos>,
    ) -> std::io::Result<()> {
        if heading.level == 1 {
            return write!(output, "<h{}>", heading.level);
        }

        let anchor = {
            let mut anchorizer = self.anchorizer.lock().unwrap();
            anchorizer.anchorize(heading.content.clone())
        };

        {
            let section = Section {
                level: heading.level,
                title: heading.content.clone(),
                anchor: anchor.clone(),
                children: vec![],
            };
            let mut sections = self.sections.lock().unwrap();
            sections.push(section);
        }

        write!(
            output,
            "<h{} id=\"{}\"><a class=\"anchor-link\" href=\"#{}\" aria-label=\"Link to this section: {}\"></a>",
            heading.level, anchor, anchor, heading.content
        )
    }

    fn exit(&self, output: &mut dyn Write, heading: &HeadingMeta) -> std::io::Result<()> {
        write!(output, "</h{}>", heading.level)
    }
}
