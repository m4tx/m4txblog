---
title: "Welcome, Cot: the Rust web framework for lazy developers"
permalink: "cot-the-rust-web-framework-for-lazy-developers"
date: 2024-02-18 12:30:00+0000
---

[Cot] was born out of frustration - the kind that every Rust developer feels when searching for a batteries-included, Django-like web framework that just handles the basics for you. While Rust is a really mature language, the web ecosystem is still sort of lacking. Let's change that.

## Motivation

There is a variety of web *libraries* for Rust. There is [axum](https://github.com/tokio-rs/axum), [Rocket](https://rocket.rs/), [Actix](https://actix.rs/), and many more. But most of them are very low-level and are pretty rough even for basic stuff. Cot tries to fill in this gap and tries to create a web framework for lazy developers. While you might be familiar with web servers, routers, and middlewares, what about an administration panel that lets you easily modify your application's models? What about an ORM (Object-Relational Mapping) that allows you to use your database almost as easily as native Rust object, and handle the migrations, automatically, too? Do you have an unmatched ease of use and the speed to get started? That's the gap Cot tries to fill in. And yes, there's Loco (which, by the way, has started after the idea for Cot was born), which is a great framework, but we're hoping to eventually deliver better experience for easier development by providing more features, better APIs, and stronger decisions.

## Cot

[Cot] is a modern, easy-to-use, batteries-included web framework for the Rust programming language. It is inspired by [Django](https://www.djangoproject.com/), so that it should seem familiar if you've ever used it, but also easy to learn if you haven't. Not only does it try to provide you with the tools to "just" create a web application. It tries to provide you much more: sessions, authentication, templates, admin panel – all of that sprinkled with a bit of type-safety of Rust and an extensive documentation. It even goes as far as creating its own ORM and automatic migration system, because we think the existing ORMs are not as easy to use as we would like.

Don't get me wrong, we're not reinventing everything here. Cot is built on the shoulders of giants – it boasts axum, sea-query, tower, serde, and many other great projects under the hood. We've spent a lot of time though making sure it all works well together and provides plenty of features in between those projects to get you started quickly.

## The first release

Cot is still in its early stages. The first release is a milestone, marking the point where it should start to be usable by other people, but it still isn't mature enough to go anywhere near production. That said, the very text you are reading right now is being served by a blog engine written in Cot, so it's already in a quite usable state.

## What's missing

A lot of stuff:

* The ORM is very lacking at the moment and the automatic migration generator only works with a small subset of possible operations,
* Request Handler API is far from being ergonomic and there's no automatic OpenAPI docs generation,
* The admin panel lacks pagination, filtering, or search, and missing support for many data types,
* There is no support for static file processors (such as SCSS compilers) and the static files framework doesn't yet handle content hashing,
* There is no background tasks or scheduling, no web socket, or permission system,
* Probably much more!

We're hoping to fix most of the above in the v0.2 release. When? We'll see, but we're hoping in a month or two. We're aiming for frequent releases that will allow people to provide feedback more often, hopefully limiting the API breakages in the long run.

## News

As my personal blog, this will be your go-to source for Cot's development news. I'll be sharing regular updates, so bookmark this page to stay in the loop!

## Contributions

With this release, Cot is ready for community involvement! While we've laid the foundation, there's an exciting road ahead, and we'd love your help in shaping the framework's future. If you are interested in web development, Rust, or just want to help, feel free to join the [Discord server](https://discord.cot.rs/) and say hi. We are a friendly bunch and we will be happy to help you get started. There are a lot of [good first issue] and [help wanted] issues in the repository, so you can start contributing right away. If that's too much, we will be very grateful for any feedback you can provide. Just open an issue or write on Discord. If you feel like the path Cot is taking is something that resonates with you, you can also support us financially on [Open Collective](https://opencollective.com/cot/). Whatever it is, we are looking forward to hearing from you!

## Final thoughts

We're excited to hear your thoughts - both praise and constructive criticism are welcome! Join us in building the next generation of Rust web development. Get started with [our official guide](https://cot.rs/guide/latest/) and see you in our [Discord server] and [GitHub repository](https://github.com/cot-rs/cot)!

[Cot]: https://cot.rs
[Discord server]: https://discord.cot.rs/
[good first issue]: https://github.com/cot-rs/cot/issues?q=is%3Aissue%20state%3Aopen%20label%3A%22good%20first%20issue%22
[help wanted]: https://github.com/cot-rs/cot/issues?q=is%3Aissue%20state%3Aopen%20label%3A%22help%20wanted%22
