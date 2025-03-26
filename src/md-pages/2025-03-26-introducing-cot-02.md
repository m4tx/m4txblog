---
title: "Introducing Cot v0.2"
permalink: "introducing-cot-v02"
date: 2025-03-26 7:30:00+0000
---

I'm happy to announce that [Cot](https://cot.rs/) v0.2 has been released today! The first bigger release after the public announcement brings a fair number of new features, improvements, and bug fixes. Most importantly, we've gathered a lot of feedback from people during the announcement, which we are continuously integrating into the project. Let's dive into the details!

## Announcement

We've posted news about releasing the framework in a few places where the Rust community gathers: [Reddit](https://www.reddit.com/r/rust/comments/1isd428/welcome_cot_the_rust_web_framework_for_lazy/), [Hacker News](https://news.ycombinator.com/item?id=43089468), and the [Rust users forum](https://users.rust-lang.org/t/welcome-cot-the-rust-web-framework-for-lazy-developers/125795). The majority of feedback was overwhelmingly positive, the project gained some initial popularity, and we've collected a lot of ideas for the future development. We're grateful for all the comments and suggestions! We've applied some of them into the project already, but you'll see most of the changes in future releases. We've also started to see some contributions from the community, which is a great sign that people are interested in the project!

## What's new

Perhaps the biggest change in this release is the adoption of extractors for request handlers. This change allows you to extract data, such as path parameters, request body, form data, and others, from the request and pass it to the handler as arguments. This is a common pattern in Rust web frameworks nowadays (mainly in Axum and dependent ones), and we're happy to bring it to Cot. This change makes the request handlers much more ergonomic and easier to test, but this is also a part of a bigger plan to introduce automatic [OpenAPI spec](https://www.openapis.org/) generation soon. The change moves the dependencies of each request handler to the type system, which makes it accessible for the framework to introspect the handler and generate the spec automatically. This means that soon you'll be able to have a nice API documentation and even try out the API directly from the browser—all without any additional work from your side!

Among other bigger changes, there is a support now for removing models and fields in the automatic migration generator in our ORM. The admin panel has now a basic support for pagination. `SessionMiddleware` now can be configured not to use `Secure` cookie for storing session ID, which fixes problems with signing in on localhost in WebKit-based browsers, such as Safari or GNOME Web. These changes were all contributed by the community—thanks a lot for this!

In addition to these changes, there was a fair number of improvements, fixes, and tiny additions. We're now at 100% documentation coverage. We've migrated the codebase to Rust 2024 edition. Serialization and deserialization errors now include the path in the object where the error happened, which should be helpful for debugging invalid requests. You can read about these changes and many other in [the changelog](https://github.com/cot-rs/cot/blob/master/CHANGELOG.md) if you're curious!

## Thoughts

One thought that I've got after improving the development processes with various tools, including [release-plz](https://release-plz.dev/) is that maintaining [semver compatibility](https://doc.rust-lang.org/cargo/reference/semver.html) is surprisingly difficult, especially when you're doing this manually. It's easy to miss that one structure that now stops implementing `UnwindSafe`, that one method added to a `trait` which hasn't been [sealed](https://predr.ag/blog/definitive-guide-to-sealed-traits-in-rust/), that one type not being `Send` anymore... the list goes on and on.

We see this problem in the ecosystem, too. During the development of the v0.2 crate, we've stumbled upon two dependencies which became broken because the authors inadvertently introduced breaking changes in minor versions. I don't want to point fingers at them because that's not the point—the point is, it's super useful to integrate the tooling in your workflow (such as release-plz I've mentioned, or [cargo-semver-checks](https://github.com/obi1kenobi/cargo-semver-checks) that it uses internally) to avoid causing problems for the consumers of your crates.

Part of the reason for our frequent release policy is that we want to keep the number of these changes under control between subsequent releases. It's still pretty early in the project, so breakages are expected, but we'll try to stabilize important APIs as soon as possible to avoid surprises.

## What's next

We'll continue to bring steady improvements for the time being. I was a little bit short on time to develop Cot for the past month because a lot has happened in my life lately, but it seems like the second maintainer (Marek) and other contributors are doing a great job at developing the framework. We're hoping to release v0.3 in about month, which will hopefully bring the OpenAPI spec generator. In the meantime, don't forget to check out the latest guide at [the Cot website](https://cot.rs/guide/v0.2/)! If you want to help the framework grow, you can also take a stab at one of the [open issues](https://github.com/cot-rs/cot/issues) and contribute!
