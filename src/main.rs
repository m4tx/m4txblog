mod posts;

use cot::bytes::Bytes;
use cot::cli::CliMetadata;
use cot::config::ProjectConfig;
use cot::project::{App, Project, RootHandlerBuilder, WithApps, WithConfig};
use cot::request::{Request, RequestExt};
use cot::response::{Response, ResponseExt};
use cot::router::{Route, Router};
use cot::static_files::StaticFilesMiddleware;
use cot::{static_files, AppBuilder, Body, BoxedHandler, ProjectContext, StatusCode};
use indexmap::IndexMap;
use m4txblog_common::md_pages::MdPage;
use m4txblog_macros::md_page;
use rinja::Template;

use crate::posts::get_posts;

#[derive(Debug, Template)]
#[template(path = "index.html")]
struct IndexTemplate<'a> {
    posts: &'a IndexMap<String, MdPage>,
    request: &'a Request,
}

async fn index(request: Request) -> cot::Result<Response> {
    let posts = get_posts();

    let index_template = IndexTemplate {
        posts,
        request: &request,
    };
    let rendered = index_template.render()?;

    Ok(Response::new_html(StatusCode::OK, Body::fixed(rendered)))
}

#[derive(Debug, Template)]
#[template(path = "post.html")]
struct PostTemplate<'a> {
    post: &'a MdPage,
    request: &'a Request,
}

async fn post(request: Request) -> cot::Result<Response> {
    let page = request.path_params().parse()?;

    page_response(&request, page)
}

fn page_response(request: &Request, page: &str) -> cot::Result<Response> {
    let post_map = get_posts();
    let post = post_map.get(page).ok_or_else(cot::Error::not_found)?;

    let guide_template = PostTemplate { post, request };

    let rendered = guide_template.render()?;
    Ok(Response::new_html(StatusCode::OK, Body::fixed(rendered)))
}

#[derive(Debug, Template)]
#[template(path = "md_page.html")]
struct MdPageTemplate<'a> {
    page: &'a MdPage,
    request: &'a Request,
}

async fn about(request: Request) -> cot::Result<Response> {
    let template = MdPageTemplate {
        page: &md_page!("about"),
        request: &request,
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

    fn register_apps(&self, modules: &mut AppBuilder, _app_context: &ProjectContext<WithConfig>) {
        modules.register_with_views(CotSiteApp, "");
    }

    fn middlewares(
        &self,
        handler: RootHandlerBuilder,
        context: &ProjectContext<WithApps>,
    ) -> BoxedHandler {
        let handler = handler.middleware(StaticFilesMiddleware::from_app_context(context));
        #[cfg(debug_assertions)]
        let handler = handler.middleware(cot::middleware::LiveReloadMiddleware::new());
        handler.build()
    }
}

#[cot::main]
fn main() -> impl Project {
    CotSiteProject
}
