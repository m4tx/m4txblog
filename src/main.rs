mod posts;

use cot::bytes::Bytes;
use cot::cli::CliMetadata;
use cot::config::ProjectConfig;
use cot::http::request::Parts;
use cot::project::{App, MiddlewareContext, Project, RegisterAppsContext, RootHandlerBuilder};
use cot::request::RequestExt;
use cot::request::extractors::{FromRequestParts, Path};
use cot::response::{Response, ResponseExt};
use cot::router::{Route, Router, Urls};
use cot::static_files::StaticFilesMiddleware;
use cot::{AppBuilder, Body, BoxedHandler, StatusCode, static_files};
use indexmap::IndexMap;
use m4txblog_common::md_pages::MdPage;
use m4txblog_macros::md_page;
use rinja::Template;

use crate::posts::get_posts;

#[derive(Debug, Clone)]
struct BaseContext {
    urls: Urls,
    route_name: String,
}

impl FromRequestParts for BaseContext {
    async fn from_request_parts(parts: &mut Parts) -> cot::Result<Self> {
        let urls = Urls::from_request_parts(parts).await?;
        let route_name = parts.route_name().unwrap_or_default().to_owned();

        Ok(Self { urls, route_name })
    }
}

#[derive(Debug, Template)]
#[template(path = "index.html")]
struct IndexTemplate<'a> {
    posts: &'a IndexMap<String, MdPage>,
    base_context: &'a BaseContext,
}

async fn index(base_context: BaseContext) -> cot::Result<Response> {
    let posts = get_posts();

    let index_template = IndexTemplate {
        posts,
        base_context: &base_context,
    };
    let rendered = index_template.render()?;

    Ok(Response::new_html(StatusCode::OK, Body::fixed(rendered)))
}

#[derive(Debug, Template)]
#[template(path = "post.html")]
struct PostTemplate<'a> {
    post: &'a MdPage,
    base_context: &'a BaseContext,
}

async fn post(base_context: BaseContext, Path(page): Path<String>) -> cot::Result<Response> {
    page_response(&base_context, &page)
}

fn page_response(base_context: &BaseContext, page: &str) -> cot::Result<Response> {
    let post_map = get_posts();
    let post = post_map.get(page).ok_or_else(cot::Error::not_found)?;

    let guide_template = PostTemplate { post, base_context };

    let rendered = guide_template.render()?;
    Ok(Response::new_html(StatusCode::OK, Body::fixed(rendered)))
}

#[derive(Debug, Template)]
#[template(path = "md_page.html")]
struct MdPageTemplate<'a> {
    page: &'a MdPage,
    base_context: &'a BaseContext,
}

async fn about(base_context: BaseContext) -> cot::Result<Response> {
    let template = MdPageTemplate {
        page: &md_page!("about"),
        base_context: &base_context,
    };

    Ok(Response::new_html(
        StatusCode::OK,
        Body::fixed(template.render()?),
    ))
}

struct CotSiteApp;

impl App for CotSiteApp {
    fn name(&self) -> &'static str {
        "m4txblog"
    }

    fn router(&self) -> Router {
        Router::with_urls([
            Route::with_handler_and_name("/", index, "index"),
            Route::with_handler_and_name("/about-me/", about, "about"),
            Route::with_handler_and_name("/blog/{page}/", post, "post"),
        ])
    }

    fn static_files(&self) -> Vec<(String, Bytes)> {
        static_files!(
            "css/main.css",
            "js/color-modes.js",
            "images/favicon-32.png",
            "images/favicon-180.png",
            "images/favicon-192.png",
            "images/favicon-512.png",
            "images/site.webmanifest"
        )
    }
}

struct CotSiteProject;

impl Project for CotSiteProject {
    fn cli_metadata(&self) -> CliMetadata {
        cot::cli::metadata!()
    }

    fn config(&self, _config_name: &str) -> cot::Result<ProjectConfig> {
        // we don't need to load any config
        Ok(ProjectConfig::default())
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

#[cot::main]
fn main() -> impl Project {
    CotSiteProject
}
