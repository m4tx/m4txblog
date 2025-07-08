---
title: "The journey towards the best error handling in Rust web frameworks"
permalink: "towards-rust-web-best-errors"
date: 2025-07-08 7:30:00+0000
---

I am working on a Rust web framework called [Cot](https://cot.rs/). This is a framework heavily inspired by Django and influenced by many Rust web frameworks, such as Actix, Axum, Poem, Rocket, and others. The main goal is to provide a state of the art developer experience, and part of that is to provide the best error handling possible with minimal developer effort.

Errors are often one of the most frustrating parts of programming. In web framework they may lead to lost data and confused users. Despite that, they are often overlooked not only in the applications, but also in the frameworks themselves. This is very well explained in Luca Palmieri's blogpost ["Rust web frameworks have subpar error reporting"](https://www.lpalmieri.com/posts/rust-web-frameworks-have-subpar-error-reporting/), which was a source of inspiration not only for this post, but also for the error handling design in Cot. I highly recommend reading it if you haven't already!

For quite a long time, error handling in Cot has been pretty rough, too. I decided to finally do something about it and implement a proper error handling system that would be easy to use, consistent, and flexible.

## Specification

So what do we expect from our dream error handling in a web framework? Let's try to list the requirements:

* Error handlers should be **easy to write and register**.
* Error handlers should have **access to as much information as possible**. This includes _the original request_, the error itself, middleware-injected context, and any other relevant data (such as the project configuration).
* It shouldn't make any difference for the user where the error happened, whether it was in the request handler, middleware, or somewhere else. The error should be handled **in a consistent way**. We shouldn't be able to "forget" about handling an error in a middleware, for example, or handle it in a different way than in the request handler (unless we want to, of course).
* Error handlers **can be affected by the middleware**. If you want to put compression middleware in front of your error handler, you should be able to do that.

Our example handler could look like this:

```rust
async fn handle_error(request: Request, urls: Urls, error: cot::Error) -> impl IntoResponse {
    let message = format!(
        "An error occurred. You might want to go back to <a href="{}">the homepage</a>.",
        cot::reverse!(urls, "index")
    );

    error!(?request, ?error, "Error occurred processing request");

    Html::new(message).with_status(error.status_code())
}
```

## The elephant in the room

On paper, the above requirements don't sound too difficult to achieve. However, there is just one problem: we use [`tower`] to handle our middleware. Or more specifically, we use [`tower_http`]. All is fine and dandy until we realize that `tower_http` is built in a way that [requires you to pass an owned request object into the middleware](https://docs.rs/tower-http/latest/tower_http/compression/struct.Compression.html#impl-Service%3CRequest%3CReqBody%3E%3E-for-Compression%3CS,+P%3E), and there is no way to work around this, as the request type is fixed and cannot be changed. This means that the request is consumed by the middleware, and you can't access it later in the error handler.

## Solutions

Let's go through the possible solutions to this problem.

### Drop `tower_http`

The most straightforward solution is to drop `tower_http` and use `tower` directly. `tower` itself is very flexible in a sense that it allows you to use any type of request and response. However, this means that we would have to drop a large number of middleware that are already implemented in `tower_http` and other crates, such as compression, CORS, session support, and many others. This also limits the usefulness of the framework, given that the ecosystem of `tower_http` is quite rich.

If we want to go even further, we could even drop `tower` and implement our own middleware system. This means doing even more work that has already been done.

### Clone the request

Another quite obvious solution is to clone the request (sans the body) just in case error happens, and we need to access it later. This is a very simple solution, but it has a few drawbacks:

* Cloning the request is not free, and it will result in performance overhead. For applications handling thousands of requests per second, this overhead can become significant, especially when most requests complete successfully and never need the cloned data.
* It may lead to weird bugs, inconsistencies, and confusion. For example, if the request is modified in the middleware, the cloned request will not reflect those changes. This may lead to unexpected behavior in the error handler, as it will not have access to the modified request. Some middleware may also not expect this—we can imagine some sort of a "make-idempotent" middleware that drops request if a message with given ID was already processed. This can be worked around by only registering such middleware for the regular request handlers, but it still adds complexity to the framework.

### Change `tower_http`

An interesting solution would be to change `tower_http` to allow passing a mutable reference to the request rather than an owned object. We could go even further: change the middleware implementation to allow generic request and response types, as long as they implement some trait that allows us to access the request and response data in a unified way. This would allow us to use `tower_http` with framework that didn't use `tower` at all, as long as we provide trait implementations for appropriate types.

While this approach would benefit the entire Rust ecosystem by making tower_http more flexible, it faces significant adoption challenges. It would require a lot of work and coordination with the `tower_http` maintainers, which don't necessarily have to agree with this solution.

## Comparison between frameworks

Let's quickly see at a few different Rust web frameworks and how they approach this problem:

* [**Axum**](https://docs.rs/axum/latest/axum/), being built directly on top of `tower_http`, is very much affected by the problem. However, an interesting solution is that [the error handling service first runs the dependency injection system for the error handler, and then runs the request handler itself](https://github.com/tokio-rs/axum/blob/fb64e72de98d229fc3911a6c441514d9c452b108/axum/src/error_handling/mod.rs#L151-L205). If no error occurred, the data extracted from the request is dropped; otherwise, it is used to handle the error. This is marginally better than cloning the entire request parts, and has mostly the same drawbacks (except that the performance is slightly better).
* [**Actix Web**](https://actix.rs/) is not directly built on top of `tower`, but its middleware system is pretty much identical. Its [`ErrorHandlers`](https://docs.rs/actix-web/4.11.0/actix_web/middleware/struct.ErrorHandlers.html#method.default_handler) type only gives your handler access to the erroneous response, but you can achieve similar functionality to Axum by writing your own error handling middleware.
* [**Poem**](https://docs.rs/poem/latest/poem/)'s error handling and middleware system is again, [follows a similar pattern to Axum and Actix Web](https://docs.rs/poem/latest/poem/endpoint/trait.EndpointExt.html#method.catch_error).
* [**Rocket**](https://rocket.rs/) does things quite differently. Its [middleware system](https://rocket.rs/guide/v0.5/fairings/#example) gives access to mutable references to the `Request` object, which then can be accessed in the [error catchers](https://rocket.rs/guide/v0.5/requests/#error-catchers). This ticks all the boxes for our requirements, but it is not built on top of `tower` and its ecosystem, which means that it is not compatible with the existing `tower_http` middleware.
* [**Pavex**](https://pavex.dev/)'s middleware and error handling seem to be more powerful versions of what Rocket has—in particular, it [allows you to access your regular dependency injection system in the error handlers](https://pavex.dev/docs/guide/errors/error_handlers/). As Pavex was written by Luca Palmieri, the author of the blog post I mentioned earlier, it is not surprising that it has a very good error handling system. However, similarly to Rocket, it is not built on top of `tower` and its ecosystem, which means that it is not compatible with the existing `tower_http` middleware.

This can be summarized in the following table:

<table class="table">
    <thead>
        <tr>
        <th>Framework</th>
        <th>Tower compatibility</th>
        <th>Access to request in error handler</th>
        </tr>
    </thead>
    <tbody>
        <tr>
            <td>Axum</td>
            <td>✅</td>
            <td>⚠️ With a performance overhead</td>
        </tr>
        <tr>
            <td>Actix Web</td>
            <td>✅</td>
            <td>⚠️ Not built-in and with a performance overhead️</td>
        </tr>
        <tr>
            <td>Poem</td>
            <td>✅</td>
            <td>⚠️ Not built-in and with a performance overhead️</td>
        </tr>
        <tr>
            <td>Rocket</td>
            <td>❌</td>
            <td>✅</td>
        </tr>
        <tr>
            <td>Pavex</td>
            <td>❌</td>
            <td>✅</td>
        </tr>
    </tbody>
</table>

## Final thoughts

For now, cloning the request parts seems to be the best solution for Cot. It is simple to implement, and it allows us to use the existing `tower_http` middleware ecosystem. Perhaps in the future we will think about replacing `tower_http` with something built in-house, but for now it should be good enough.

As a closing thought, I'm happy to be wrong about everything I've said in this post. If you have a better idea, or if you think that the current solution is good enough, please let me know! I'm always open to discussion and would love to hear your thoughts on this topic.

[`tower`]: https://docs.rs/tower/latest/tower/index.html
[`tower_http`]: https://docs.rs/tower-http/latest/tower_http/
