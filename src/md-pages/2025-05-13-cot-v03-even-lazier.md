---
title: "Cot v0.3: Even Lazier"
permalink: "cot-v03-even-lazier"
date: 2025-05-13 12:30:00+0000
reddit_link: https://www.reddit.com/r/rust/comments/1klvjzf/cot_v03_even_lazier/
---

13 May 2025 marks the release of Cot v0.3, a significant update to the Rust web framework for lazy developers. This release introduces several new features, improvements, and bug fixes, making it even easier to build web applications with Cot.

## What's new

### OpenAPI

The adoption of extractors for request handlers in [v0.2](/blog/introducing-cot-v02/) paved a path to a very important feature introduced in Cot v0.3: automatic OpenAPI spec generation. This feature allows you to generate an OpenAPI specification for your API endpoints automatically, making it almost zero effort to generate Swagger UI automatically for your RESTful endpoints that will always be kept in sync with the source code.

What does this mean in practice?

When you define a request handler, you can register it in a router by using a `api_get`, `api_post`, and similar functions. Then, if all the parameters and return types are compatible with OpenAPI, Cot will automatically generate an OpenAPI spec for you. Then you can register a Swagger UI app in your project, and it will automatically use the generated OpenAPI spec to display the API documentation. An example can be found below:

```rust
#[derive(Deserialize, schemars::JsonSchema)]
struct AddRequest {
    a: i32,
    b: i32,
}

#[derive(Serialize, schemars::JsonSchema)]
struct AddResponse {
    result: i32,
}

// example API endpoint displayed in Swagger UI
async fn add(Json(add_request): Json<AddRequest>) -> cot::Result<Json<AddResponse>> {
    let response = AddResponse {
        result: add_request.a + add_request.b,
    };

    Json(response)
}

struct AddApp;
impl App for AddApp {
    fn router(&self) -> Router {
        Router::with_urls([Route::with_api_handler("/add/", api_post(add))])
    }

    // ...
}

struct ApiProject;
impl Project for ApiProject {
    fn register_apps(&self, apps: &mut AppBuilder, _context: &RegisterAppsContext) {
        apps.register_with_views(SwaggerUi::new(), "/swagger");
        apps.register_with_views(AddApp, "");
    }

    // ...
}
```

That's it! It's that simple. The OpenAPI spec will be generated automatically based on the request and response types, and the Swagger UI will display it for you. You can read about the details on the [Cot official guide](https://cot.rs/guide/latest/openapi/). The OpenAPI is probably still missing some features, but we'll iron these out in the next releases. If you have any issues or suggestions, please let us know! And thanks to the amazing projects: [aide](https://docs.rs/aide/latest/aide/) and [schemars](https://graham.cool/schemars/) that made this possible!

### IntoResponse

In addition to extractors introduced in [v0.2](/blog/introducing-cot-v02/), the `IntoResponse` trait has been added to the framework. This trait allows you to convert any type into a response, making it easier to return different types of responses from your request handlers. In v0.3 it's been implemented for types such as `Html` and `Json`, so instead of:

```rust
async fn hello() -> cot::Result<Response> {
    Ok(Response::new_html(StatusCode::OK, Body::fixed("Hello World!")))
}
```

you can now write:

```rust
async fn hello() -> Html {
    Html::new("Hello World!")
}
```

Neat!

### Static file content hashing

The static file framework has been improved to support content hashing. The default Cot project template will now automatically add a hash as a query parameter in the static files URLs. This means that, for instance, for `images/logo.png`, the URL will look like `/static/images/logo.png?v=e3b0c44298fc`, where `e3b0c44298fc` is a hash of the file contents. This way, if the file changes, the URL will change too, and the browser will fetch the new version.

This is a big win for production performance, as it allows you to use aggressive caching strategies without worrying about cache invalidation. This means that you can set long cache expiration times for your static files (also the new default in the project template!), and the browser will only fetch the new version when the file changes—as the URL will change, too.

The behavior is fully configurable in the project config file:

```toml
[static_files]
url = "/static/"  # URL prefix for static files
rewrite = "query_param"  # set to "none" to disable hashing
cache_timeout = "1year"  # "Cache-Control" header value
```

### Askama

Rinja, a templating engine used in the previous versions of Cot, has merged with Askama, the library it was a fork of. This came with a big list of changes—have a look at [the blog post about the merge](https://blog.guillaume-gomez.fr/articles/2025-03-19+Askama+and+Rinja+merge). We've upgraded the templating engine in Cot to use Askama, which means that you can now use the latest features and improvements from Askama in your Cot projects!

### Other changes

There's been quite a pack of changes contributed by the community, including:

* Form support for the Email field, contributed by [@ElijahAhianyo](https://github.com/ElijahAhianyo) in [PR #286](https://github.com/cot-rs/cot/pull/286)
* Form support for floating point types, contributed by [@ElijahAhianyo](https://github.com/ElijahAhianyo) in [PR #307](https://github.com/cot-rs/cot/pull/307)
* Table names in the database models are now prefixed with the app name that defines them, contributed by [@yk23](https://github.com/yk23) in [PR #257](https://github.com/cot-rs/cot/pull/257)
* Fix `ForeignKey<Self>` not building, contributed by [@eibrahim95](https://github.com/eibrahim95) in [PR #313](https://github.com/cot-rs/cot/pull/313)

Thanks!

As usual, all the other features and improvements are listed in the [release notes](https://github.com/cot-rs/cot/blob/master/CHANGELOG.md).  Have a look to see all the changes Cot v0.3 brings to the table!

## RustWeek

This week (13–16 May 2025), [the Rust conference in the Netherlands](https://rustweek.org/) is happening. If you are there, don't hesitate to say hi! We'll go around in Cot T-shirts, so you can spot us easily. We'll be happy to chat about Cot, Rust, or anything else you want to discuss.

## What's next

Remember to check out the latest guide at [the Cot website](https://cot.rs/guide/v0.3/)! If you want to help the framework grow, you can try to contribute to the project by [reporting bugs, suggesting features, or contributing code](https://github.com/cot-rs/cot). We are always looking for help, and we appreciate any contribution you can make!
