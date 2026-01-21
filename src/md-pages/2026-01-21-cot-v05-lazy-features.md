---
title: "Cot v0.5: New Features for Lazy Web Developers"
permalink: "cot-v05-new-features-for-lazy-web-developers"
date: 2026-01-21 18:52:21+0000
reddit_link: https://www.reddit.com/r/rust/comments/1qja80v/cot_v05_released_new_features_for_lazy_web/
---

21 January 2026 is the day of the release of Cot v0.5, the Rust web framework for lazy developers. This time, the release focuses on adding new features rather than changing too much in the underlying architecture. That will _[spoiler alert!]_ come soon in Cot v0.6!

## What's new

### Cache system

Cot v0.5 introduces a **flexible caching system** that allows you to store and retrieve data quickly, reducing the load on your database and improving response times. The system is designed to be pluggable, so you can easily switch between backends like `memory` (for development) or `redis` (for production) without changing your application code.

To get started, configure the cache in your `config/*.toml`:

```toml
[cache.store]
type = "redis"
url = "redis://127.0.0.1:6379"
```

Then, you can access the cache using the `Cache` extractor:

```rust
use cot::cache::Cache;
use cot::html::Html;

async fn cache_example(cache: Cache) -> cot::Result<Html> {
    // Insert a value (uses default expiration if set in config, or infinite)
    cache.insert("user_1_name", "Alice").await?;

    // Get a value
    let name: Option<String> = cache.get("user_1_name").await?;

    if let Some(n) = name {
        println!("Found user: {}", n);
    }

    Ok(Html::new("OK"))
}
```

### Email support

**Sending emails** is a common requirement, and Cot now provides a unified interface for it, powered by the popular [`lettre`](https://crates.io/crates/lettre) crate. You can switch between different transports (like SMTP or Console) just by changing your configuration.

Enable the `email` feature in your `Cargo.toml`, and then configure it:

```toml
[email]
from = "no-reply@example.com"

[email.transport]
type = "smtp"
url = "smtp://user:password@localhost:587"
```

Sending an email is as simple as using the `Email` extractor:

```rust
use cot::common_types::Email;
use cot::email::{Email as EmailService, EmailMessage};
use cot::html::Html;

async fn send_welcome_email(email_sender: EmailService) -> cot::Result<Html> {
    let message = EmailMessage::builder()
        .to(vec![Email::try_from("user@example.com").unwrap()])
        .subject("Welcome to Cot!")
        .body("Hello, welcome to our service!")
        .build()?;

    email_sender.send(message).await?;

    Ok(Html::new("Email sent!"))
}
```

By default, the Console transport is used for development - **it simply prints emails to stdout**, so you don't have to set up an SMTP server for local testing!

### `bulk_insert` in the ORM

For those needing to insert large amounts of data, the ORM now supports `bulk_insert`. This method is significantly faster than calling `insert` multiple times as **it combines all instances into a single SQL `INSERT` statement**.

```rust
let mut todos = vec![
    TodoItem { id: Auto::auto(), title: "Task 1".into() },
    TodoItem { id: Auto::auto(), title: "Task 2".into() },
    TodoItem { id: Auto::auto(), title: "Task 3".into() },
];

TodoItem::bulk_insert(&db, &mut todos).await?;

// After insertion, all todos have populated IDs
assert!(todos[0].id.is_fixed());
```

### `SelectAsFormField` derive macro

Working with forms has just got slightly easier now. The new `SelectAsFormField` derive macro allows you to implement `AsFormField` for your enums, making them directly usable in `SelectField` or `SelectMultipleField`.

```rust
use cot::form::fields::{SelectAsFormField, SelectChoice};

#[derive(SelectChoice, SelectAsFormField, Debug, Clone, PartialEq, Eq, Hash)]
enum Status {
    Draft,
    Published,
    Archived,
}

// `Status` can now be used with `SelectField<Status>`!
```

### `cot::Template` macro

A nice-to-have addition in this release is that Cot now re-exports the `Template` trait and the `#[derive(Template)]` macro from Askama. This means you can import them directly from Cot, without needing to depend on Askama explicitly in your `Cargo.toml`.

### Framework comparison table

A long overdue change not in the framework itself, but on its website, is the addition of a comparison table of popular Rust web frameworks. You can find it at [cot.rs/guide/latest/framework-comparison/](https://cot.rs/guide/latest/framework-comparison/). It compares Cot with Actix Web, Axum, Rocket, and Loco to help you make an informed decision for your next project.

## Upgrade guide

When upgrading from Cot v0.4 to v0.5, please refer to the [upgrade guide](https://cot.rs/guide/v0.5/upgrade-guide/). It contains all the necessary information about breaking changes and how to adapt your code.

## What's next

As mentioned earlier, Cot v0.6 will bring some significant architectural changes to the framework. The main focus will be improving modularity, hopefully leading to faster compile times and an easier-to-understand codebase. Stay tuned!

Check out the latest guide at [the Cot website](https://cot.rs/guide/v0.5/) to start developing with Cot! I'd also like to encourage you to contribute to the project by [reporting bugs, suggesting features, or writing code](https://github.com/cot-rs/cot). This is a community-driven project, and your contributions are highly appreciated.

If you want to see more details, have a look at [the release notes](https://github.com/cot-rs/cot/releases/tag/cot-v0.5.0). Thank you all for your help!
