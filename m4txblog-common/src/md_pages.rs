use std::fmt::Write;
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

    pub fn comments_open(&self) -> bool {
        !self.is_archived()
    }

    pub fn has_discussion_links(&self) -> bool {
        self.reddit_link.is_some() || self.hackernews_link.is_some()
    }

    /// Whether the page is long enough to show a table of contents.
    ///
    /// The table of contents only contains the top two levels of sections, so
    /// only these are counted.
    pub fn has_toc(&self) -> bool {
        const MIN_TOC_SECTIONS: usize = 3;

        let section_count: usize = self
            .sections
            .iter()
            .map(|section| 1 + section.children.len())
            .sum();
        section_count >= MIN_TOC_SECTIONS
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
    ) -> std::fmt::Result {
        if heading.level == 1 {
            return write!(output, "<h{}>", heading.level);
        }

        let anchor = {
            let mut anchorizer = self.anchorizer.lock().unwrap();
            anchorizer.anchorize(&heading.content)
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
            "<h{} id=\"{}\"><a class=\"anchor-link\" href=\"#{}\" aria-label=\"Link to this section: ",
            heading.level, anchor, anchor
        )?;
        comrak::html::escape(output, &heading.content)?;
        write!(output, "\"></a>")
    }

    fn exit(&self, output: &mut dyn Write, heading: &HeadingMeta) -> std::fmt::Result {
        write!(output, "</h{}>", heading.level)
    }
}
