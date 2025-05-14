mod comments;
pub mod consts;
mod feed;
mod migrations;
mod posts;

use askama::Template;
use async_trait::async_trait;
use cot::cli::clap::{ArgMatches, Command};
use cot::cli::{Cli, CliMetadata, CliTask};
use cot::db::migrations::SyncDynMigration;
use cot::html::Html;
use cot::http::request::Parts;
use cot::project::{
    App, MiddlewareContext, Project, RegisterAppsContext, RootHandlerBuilder, WithConfig,
};
use cot::request::RequestExt;
use cot::request::extractors::{FromRequestParts, Path, StaticFiles};
use cot::router::{Route, Router, Urls};
use cot::static_files::{StaticFile, StaticFilesMiddleware};
use cot::{AppBuilder, Bootstrapper, BoxedHandler, static_files};
use indexmap::IndexMap;
use m4txblog_common::md_pages::MdPage;
use m4txblog_macros::{md_page, static_files_dir};
use tracing_subscriber::util::SubscriberInitExt;

use crate::feed::blog_feed;
use crate::posts::{get_archived_post_map, get_post_map, get_unarchived_post_map};

#[derive(Debug, Clone)]
struct BaseContext {
    urls: Urls,
    static_files: StaticFiles,
    route_name: String,
}

impl FromRequestParts for BaseContext {
    async fn from_request_parts(parts: &mut Parts) -> cot::Result<Self> {
        let urls = Urls::from_request_parts(parts).await?;
        let static_files = StaticFiles::from_request_parts(parts).await?;
        let route_name = parts.route_name().unwrap_or_default().to_owned();

        Ok(Self {
            urls,
            static_files,
            route_name,
        })
    }
}

#[derive(Debug, Template)]
#[template(path = "index.html")]
struct IndexTemplate<'a> {
    posts: &'a IndexMap<String, Vec<MdPage>>,
    base_context: &'a BaseContext,
}

async fn index(base_context: BaseContext) -> cot::Result<Html> {
    let posts = get_unarchived_post_map();

    let index_template = IndexTemplate {
        posts,
        base_context: &base_context,
    };

    Ok(Html::new(index_template.render()?))
}

#[derive(Debug, Template)]
#[template(path = "archive.html")]
struct ArchiveTemplate<'a> {
    posts: &'a IndexMap<String, Vec<MdPage>>,
    base_context: &'a BaseContext,
}

async fn archive(base_context: BaseContext) -> cot::Result<Html> {
    let posts = get_archived_post_map();

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
    base_context: &'a BaseContext,
}

async fn post(base_context: BaseContext, Path(page): Path<String>) -> cot::Result<Html> {
    page_response(&base_context, &page, None)
}

async fn post_with_lang(
    base_context: BaseContext,
    Path((page, lang)): Path<(String, String)>,
) -> cot::Result<Html> {
    page_response(&base_context, &page, Some(&lang))
}

fn page_response(base_context: &BaseContext, page: &str, lang: Option<&str>) -> cot::Result<Html> {
    let post_map = get_post_map();
    let post_list = post_map.get(page).ok_or_else(cot::Error::not_found)?;

    if Some(post_list[0].language.as_str()) == lang {
        // the default language should be returned only by the `post` route
        return Err(cot::Error::not_found());
    }

    let post = if let Some(lang) = lang {
        post_list
            .iter()
            .find(|post| post.language == lang)
            .ok_or_else(cot::Error::not_found)?
    } else {
        &post_list[0]
    };
    let other_languages = post_list
        .iter()
        .filter(|other_post| other_post.language != post.language)
        .map(|post| post.language.clone())
        .collect::<Vec<_>>();

    let guide_template = PostTemplate {
        post,
        other_languages: &other_languages,
        base_context,
    };

    Ok(Html::new(guide_template.render()?))
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

struct CotSiteApp;

impl App for CotSiteApp {
    fn name(&self) -> &'static str {
        "m4txblog"
    }

    fn migrations(&self) -> Vec<Box<SyncDynMigration>> {
        cot::db::migrations::wrap_migrations(migrations::MIGRATIONS)
    }

    fn router(&self) -> Router {
        Router::with_urls([
            Route::with_handler_and_name("/", index, "index"),
            Route::with_handler_and_name("/feed/", blog_feed, "feed"),
            Route::with_handler_and_name("/about-me/", about, "about"),
            Route::with_handler_and_name("/archive/", archive, "archive"),
            Route::with_handler_and_name("/blog/{page}/", post, "post"),
            Route::with_handler_and_name("/blog/{page}/{lang}/", post_with_lang, "post_with_lang"),
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
        modules.register_with_views(CotSiteApp, "");
    }

    fn middlewares(
        &self,
        handler: RootHandlerBuilder,
        context: &MiddlewareContext,
    ) -> BoxedHandler {
        let handler = handler.middleware(StaticFilesMiddleware::from_context(context));
        #[cfg(debug_assertions)]
        let handler = handler.middleware(cot::middleware::LiveReloadMiddleware::new());
        handler.build()
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct CreateAdmin;

const CREATE_ADMIN_SUBCOMMAND: &str = "create-admin";

#[async_trait(?Send)]
impl CliTask for CreateAdmin {
    fn subcommand(&self) -> Command {
        Command::new(CREATE_ADMIN_SUBCOMMAND).about("Creates a new superuser")
    }

    async fn execute(
        &mut self,
        _matches: &ArgMatches,
        _bootstrapper: Bootstrapper<WithConfig>,
    ) -> cot::Result<()> {
        todo!();
    }
}

#[cot::main]
fn main() -> impl Project {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .finish()
        .init();

    CotSiteProject
}
