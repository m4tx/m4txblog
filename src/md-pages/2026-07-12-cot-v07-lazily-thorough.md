---
title: "Cot v0.7: Lazily Thorough"
permalink: "cot-v07-lazily-thorough"
date: 2026-07-13 19:30:00+0000
---

Following up on [our previous announcement](/blog/cot-v06-lazy-underneath/), Cot v0.7 is here! This release packs in a wide range of improvements across forms, the ORM, security, and developer ergonomics. Let's dig in.

## What's new?

### Richer HTML form attributes

Forms are central to almost every web app, and in v0.7 we've made Cot's form system more expressive. All the major field types: `String`, `Password`, `Email`, `Url`, number, and float fields, now expose a much larger set of HTML attributes that you can configure through field options:

- `placeholder` - hint text shown when the field is empty
- `readonly` - prevents user edits without disabling the field
- `autocomplete` - controls browser autofill behaviour
- `autocapitalize` - controls auto-capitalisation on mobile keyboards
- `dir` and `dirname` - text direction for RTL language support
- `list` - connects the field to a `<datalist>` for suggestions
- `min_length`, `size` - input sizing controls
- `step` - for numeric/date fields

The `FileField` gets the new `capture` attribute, letting you specify whether the device camera or microphone should be used as the source.

```rust
use cot::form::fields::{AutoComplete, List, StringFieldOptions};

#[derive(Form)]
struct ProfileForm {
    #[form(opts(
        max_length = 100,
        placeholder = "Your full name",
        autocomplete = AutoComplete::Name,
        list = List::new(["Alice", "Bob", "Charlie"]),
    ))]
    name: String,

    #[form(opts(
        placeholder = "you@example.com",
        autocomplete = AutoComplete::Email,
        readonly = true,
    ))]
    email: String,
}
```

### Custom database column names

The `#[model]` macro now accepts a `field_name` attribute, letting you decouple the Rust field name from the underlying database column name:

```rust
#[model]
struct Article {
    #[model(primary_key)]
    id: Auto<i32>,

    #[model(field_name = "body_text")]
    content: String,

    #[model(field_name = "pub_ts")]
    published_at: chrono::DateTime<chrono::Utc>,
}
```

This is particularly handy when working with an existing database schema where column names don't match Rust naming conventions, or when you want to rename a Rust field without touching the underlying schema.

### Configurable `on_delete` and `on_update` for foreign keys

Foreign key fields now let you control referential integrity policies directly from the model definition:

```rust
use cot::db::{ForeignKey, ForeignKeyOnDeletePolicy, ForeignKeyOnUpdatePolicy};

#[model]
struct Comment {
    #[model(primary_key)]
    id: Auto<i32>,

    #[model(
        on_delete = ForeignKeyOnDeletePolicy::Cascade,
        on_update = ForeignKeyOnUpdatePolicy::Cascade,
    )]
    post: ForeignKey<Post>,

    body: String,
}
```

Previously, Cot always generated `RESTRICT` for both policies. Now you can choose `Cascade`, `SetNull`, `SetDefault`, or `NoAction`-giving you full control over what happens when a referenced row is updated or deleted.

### Secure `Password` and `SecretKey`

`Password` and `SecretKey` now back their storage with the [`securer_string`](https://crates.io/crates/securer-string) crate. Under the hood, this means:

- The memory holding the password or key is **zeroed out on drop**, reducing the risk of secrets lingering in memory.
- Comparing two `Password` instances with `==` now uses **constant-time comparison** to prevent timing attacks.
- The memory is protected using the `mlock` system call to prevent the secrets from being swapped to disk.

```rust
let p1 = Password::new("hunter2");
let p2 = Password::new("hunter2");

assert_eq!(p1, p2); // constant-time comparison

println!("{p1:?}"); // prints: Password(..)
```

### Improved error output

`cot::Error` now prints a much friendlier message when you call `.unwrap()` or `.expect()` on a `cot::Result`. Instead of a raw `Debug` blob, you'll see the full human-readable error chain:

```
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value:
could not connect to database

caused by:
   0: connection refused (os error 111)
```

[![Screenshot placeholder: improved cot error output in a terminal](/static/images/blog/2026-07-13-cot-v07-lazily-thorough-error-output.png)](/static/images/blog/2026-07-13-cot-v07-lazily-thorough-error-output.png)

If you do need the raw `Debug` output-for example, in unit tests-you can still get it with the alternate formatter `{:#?}`.

### Extra project configuration

`ProjectConfig` now has a catch-all `extra` field that captures any configuration keys Cot doesn't recognise. This is the idiomatic way to add project-specific settings to the standard `config/*.toml` files without fighting the parser:

```toml
# config/base.toml
[my_app]
analytics_key = "UA-XXXXXXXXX-1"
feature_flags = ["dark_mode", "beta_ui"]
```

```rust
use cot::config::ProjectConfig;
use serde::Deserialize;

#[derive(Deserialize)]
struct MyAppConfig {
    analytics_key: String,
    feature_flags: Vec<String>,
}

async fn handler(config: ProjectConfig) -> cot::Result<Html> {
    let app_config: MyAppConfig = config
        .extra
        .get("my_app")
        .expect("missing [my_app] section")
        .clone()
        .try_into()?;

    // ...
}
```

### Guide code snippets are now tested

As promised in the v0.6 blog post, all code examples in the [Cot guide](https://cot.rs/guide/latest/) are now automatically compiled and tested as part of the CI pipeline. This means that the guide will never silently drift out of sync with the framework-if a change breaks a guide example, the build fails. This is a huge win for the maintainability of the documentation, and a proof that the documentation is a first-class citizen in Cot.

As part of the documentation improvements, the ORM guide has got an entire new page about making queries. We'll be adding more content to the guide in the coming months, so stay tuned!

### Other improvements

- **`DatabaseUser::set_username` and `set_password`**: you can now update credentials on an existing `DatabaseUser` without recreating it. Remember to call `.save(&db).await` afterwards.
- **`Serialize`/`Deserialize`/`JsonSchema` for internal types**: `Auto<T>`, `LimitedString<N>`, `Identifier`, and other internal types now implement `serde` and `schemars` traits, making it easier to expose ORM models directly via JSON APIs.
- **`Ord`/`PartialOrd` for `Auto<T>`**: you can now compare and sort `Auto` values, which is useful when working with primary keys.
- **Route path normalisation**: route paths without a leading `/` are automatically normalised: `"home"` and `"/home"` now define equivalent routes.
- **Non-ASCII URLs**: routing and URL reversing now correctly handle paths containing non-ASCII characters.

## What's next?

We have a bunch of ORM-related features on the roadmap, and we're starting planning a background task system for Cot. We're also looking into the idea of releasing the 1.0 version closer to the end of 2026, which would give users a promise for long-term stability. If you have any ideas for the first major release, let us know on [the discussion page](https://github.com/cot-rs/cot/issues/434)!

Ready to build? Head over to the [Cot website](https://cot.rs/guide/latest/) for the updated guide.

As always, Cot is community-driven. Whether you want to [report bugs, suggest features, or contribute code](https://github.com/cot-rs/cot), every bit of help is appreciated. Join us on [Discord](https://discord.cot.rs/) to share what you're building!

For the full list of changes, see the [official release notes](https://github.com/cot-rs/cot/releases/tag/cot-v0.7.0). Thank you to everyone who contributed to this release!
