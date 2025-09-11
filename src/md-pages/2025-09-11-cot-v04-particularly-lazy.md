---
title: "Cot v0.4: Particularly Lazy"
permalink: "cot-v04-particularly-lazy"
date: 2025-09-11 18:59:14+0000
---

Well, it's been a while since the last release, but it's finally here! 11 September 2025 marks the release of Cot v0.4, **the most significant update to the Rust web framework for lazy developers**. Which, to be honest, is not surprising, given that the last release was in May. But hey, better late, than never, right?

Let's get straight to the point!

## What's new

### Error handling overhaul

The biggest change in v0.4 is a **complete restructure of our error handling system** that makes error management much more powerful and developer-friendly. I mentioned working on this in the previous blogpost about [error handling in Rust web frameworks](/blog/towards-rust-web-best-errors/); feel free to check it out for more context!

The key improvements include:

- **Proper HTTP status codes**: errors now return appropriate HTTP status codes instead of just 404 and 500;
- **Simpler API with less duplication**: simplified API for defining custom error handlers with full access to project context, error data, and middleware;
- **Performance optimizations**: error struct size reduced from 110+ bytes to just 8 bytes by storing error content on the heap and optimizing for the happy path;
- **Smart content negotiation**: debug-friendly HTML error pages are only shown when `Accept: text/html` header is present, preventing HTML flooding when testing APIs with `curl` or other tools;
- **Cleaner architecture**: `cot::Error` is now essentially a wrapper over `std::error::Error`, similarly to `anyhow::Error`, which simplifies internal error code.

#### Comparison

Before:

```rust
struct CustomNotFound;
impl ErrorPageHandler for CustomNotFound {
    fn handle(&self) -> cot::Result<Response> {
        Ok(Response::new_html(
            StatusCode::NOT_FOUND,
            Body::fixed(include_str!("404.html")),
        ))
    }
}

struct CustomServerError;
impl ErrorPageHandler for CustomServerError {
    fn handle(&self) -> cot::Result<Response> {
        Ok(Response::new_html(
            StatusCode::INTERNAL_SERVER_ERROR,
            Body::fixed(include_str!("500.html")),
        ))
    }
}

struct MyProject;
impl Project for MyProject {
    fn not_found_handler(&self) -> Box<dyn ErrorPageHandler> {
        Box::new(CustomNotFound)
    }

    fn server_error_handler(&self) -> Box<dyn ErrorPageHandler> {
        Box::new(CustomServerError)
    }
}
```

After:

```rust
impl Project for HelloProject {
    // ...

    fn error_handler(&self) -> DynErrorPageHandler {
        DynErrorPageHandler::new(error_page_handler)
    }
}

async fn error_page_handler(
    request_head: &RequestHead,
    error: RequestError
) -> cot::Result<impl IntoResponse> {
    let response = ServiceErrorResponse {
        uri: request_head.uri().to_string(),
        message: error.to_string(),
    };
    Json(response).with_status(error.status_code())
}
```

As you can see, not only is the new API much simpler, but it also gives you access to the actual error that occurred, as well as anything that you would typically retrieve via extractors, such as request headers, cookies, router URLs, static files, and more. Neat! And allows to create much nicer error pages on the cot.rs website, too:

[![cot.rs displaying the "Not Found" page](/static/images/blog/2025-09-11-cot-v04-particularly-lazy-error-page.png)](/static/images/blog/2025-09-11-cot-v04-particularly-lazy-error-page.png)

### Support for multiple session stores and database session backend

The latest version of the framework introduces support for dynamically **changing the session store based on the configuration**. This means you can now switch between different session storage backends without changing your application code—for instance, use the in-memory for development and Redis for production:

```toml
# config/prod.toml
[middlewares.session.store]
type = "cache"
uri = "redis://localhost:6379"
```

The main reason for this change is introducing a **database-backed session store**. While the in-memory store is fine for development and testing, it's not suitable for production use, as it doesn't persist sessions across server restarts. The database-backed store solves these problems by storing session data in the database, making it persistent and scalable. That's why it's the default choice now both for the default development and production configurations, as it's essentially good enough until you need the best performance possible.

### Form enhancements

Cot v0.4 introduces support for **uploading files via HTML forms**. Just add a field of type `cot::form::fields::InMemoryUploadedFile` to your form struct, and Cot will handle the rest!

```rust
#[derive(Debug, Form)]
struct FileForm {
    #[form(opt(max_length = 100))]
    title: String,
    #[form(opt(accept = vec!["image/*".to_string()]))] // accept only images
    file: InMemoryUploadedFile,
}
```

The rest looks just like [the typical HTML form handling in Cot](https://cot.rs/guide/v0.4/forms/#form-trait)!

In addition to file uploads, a notable improvement is the **support for the [`chrono` library](https://docs.rs/chrono/latest/chrono/) types**:

```rust
#[derive(Form)]
struct EventForm {
    title: String,
    event_date: chrono::NaiveDate, // Date picker
    event_time: chrono::Time, // Time picker
    datetime: chrono::DateTime<chrono::FixedOffset>, // DateTime picker
    recurring_days: chrono::WeekdaySet, // Multi-select weekdays
}
```

[![Datetime form examples](/static/images/blog/2025-09-11-cot-v04-particularly-lazy-forms.png)](/static/images/blog/2025-09-11-cot-v04-particularly-lazy-forms.png)

### FromRequestParts derive macro

A common pattern when writing web apps in Cot is using the same extractors in multiple handlers:

```rust
async fn index(urls: Urls, static_files: StaticFiles, route_name: RouteName) -> cot::Result<Html> {
    let index_template = IndexTemplate {
        urls: &urls,
        static_files: &static_files,
        route_name: &route_name,
    };

    Ok(Html::new(index_template.render()?))
}

async fn about(urls: Urls, static_files: StaticFiles, route_name: RouteName) -> cot::Result<Html> {
    let about_template = AboutTemplate {
        urls: &urls,
        static_files: &static_files,
        route_name: &route_name,
    };

    Ok(Html::new(about_template.render()?))
}
```

See the duplication? To fix this, Cot v0.4 introduces the `FromRequestParts` derive macro that allows you to **group multiple extractors into a single struct**:

```rust
#[derive(FromRequestParts)]
struct BaseContext {
    urls: Urls,
    static_files: StaticFiles,
    route_name: RouteName,
}

async fn index(base_context: BaseContext) -> cot::Result<Html> {
    let index_template = IndexTemplate { base_context };
    Ok(Html::new(index_template.render()?))
}

async fn about(base_context: BaseContext) -> cot::Result<Html> {
    let about_template = AboutTemplate { base_context };
    Ok(Html::new(about_template.render()?))
}
```

### Changed under the hood

There are several other changes under the hood to improve the framework reliability and lessen the maintenance burden. Some of the notable ones include:

* **Added `cargo-semver-checks` run in each PR** (in addition to the release PRs) to ensure that no accidental breaking changes are introduced even before the PR is merged.
* **Created a benchmarking infrastructure** to track framework performance over time and ensure that no accidental performance regressions are introduced.
* **Added automatic `HTML/Jinja2` formatting** via `pre-commit` hooks to ensure consistent formatting of all HTML templates.
* **Removed the massive `cot::error::ErrorRepr` enum** to encourage using local error types instead of a single global error type.

This sort of changes is not very visible to the end user, but they almost always pay dividends in the long run. The more processes are automated, the less likely it is that something will be forgotten or overlooked.

## Upgrade guide

This release is the first one to include [an upgrade guide](https://cot.rs/guide/v0.4/upgrade-guide/). While we try to use compiler features (such as the `#[deprecated]` attribute) to make sure that breaking changes are communicated in a way that developers can easily find them, sometimes it's just simply not possible. The upgrade guide is meant to fill this gap. Make sure to check it out if when upgrading the framework!

## Project highlight: _chombo-gen_

As part of making sure there's sufficient real-world usage of Cot, I've ported a side-project of mine, [ChomboGen](https://hand.chombo.club/), from Rocket to Cot. ChomboGen is a web service that generates images of [Riichi Mahjong](https://en.wikipedia.org/wiki/Japanese_mahjong) hands based on user input. The backend is stupidly simple, but the project is a good example of a small web service that does one thing well—check out the [source code](https://github.com/m4tx/chombo-gen/tree/master/chombo-gen-backend) and [the service itself](https://hand.chombo.club/) if you're curious! There's also [an example of the automatically generated Swagger UI](https://hand.chombo.club/swagger/).

Another project, arguably more interesting, is already being ported, so remember to check out the Cot v0.5 blog post when it's out!

## What's next

One key takeaway from the release is that making it after a 3-month-long break is difficult. It's much easier to remember the key details of the changes when you are actively working on them. The plan to avoid this is to make smaller, more frequent releases in the future—ideally, every month. This will also help with getting feedback from users faster and iterating on features more quickly.

Check out the latest guide at [the Cot website](https://cot.rs/guide/v0.4/) to start building webapps with Cot! If you feel like helping Cot grow, you are encouraged to contribute to the project by [reporting bugs, suggesting features, or writing code](https://github.com/cot-rs/cot). Any help is very much appreciated!

Lastly, have a look at [the release notes](https://github.com/cot-rs/cot/releases/tag/cot-v0.4.0) to see the full list of changes in Cot v0.4. It also includes all the contributors for this release—thank you all for being part of the Cot community and making this release as great as it is!
