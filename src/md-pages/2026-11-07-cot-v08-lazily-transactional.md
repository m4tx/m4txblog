---
title: "Cot v0.8: Lazily Transactional"
permalink: "cot-v08-lazily-transactional"
date: 2026-11-09 19:30:00+0000
reddit_link: https://www.reddit.com/r/rust/s/rPFHrq4Dch
---

Following up on [our previous announcement](/blog/cot-v07-lazily-thorough/), Cot v0.8 is here! This is one of the biggest releases so far: the ORM gets transactions, ordering, pattern matching, raw SQL, and migration rollbacks, the router has been rewritten from scratch, and the CLI got a lot smarter. Let's dig in.

## What's new?

### Transactions

Transactions were one of the most requested ORM features, and they're finally here. Call `Database::begin` to start one, then pass it anywhere you'd normally pass the database - `insert`, `save`, the `query!` macro, and so on:

```rust
async fn transfer(db: &Database) -> cot::Result<()> {
    let mut transaction = db.begin().await?;

    let mut alice = query!(Account, $id == 1).get(&mut transaction).await?.unwrap();
    let mut bob = query!(Account, $id == 2).get(&mut transaction).await?.unwrap();

    alice.balance -= 100;
    bob.balance += 100;
    alice.save(&mut transaction).await?;
    bob.save(&mut transaction).await?;

    // Both updates are applied together, or neither of them is.
    transaction.commit().await?;
    Ok(())
}
```

If a transaction is dropped without being committed (for instance, because the `?` operator returned early), it's rolled back automatically, so a failed handler never leaves a half-applied change behind. Nested transactions are supported as well - calling `begin` on a `Transaction` creates a savepoint, which can be committed or rolled back independently of the outer transaction.

Head over to the [new Transactions page in the guide](https://cot.rs/guide/v0.8/databases/transactions/) for more details.

### Ordering and pattern matching in queries

Two big gaps in the query API have been filled. First, you can now sort the results with `order_by`, which takes a list of fields or expressions, each with its own direction:

```rust
use cot::db::Model;
use cot::db::query::expr::ExprSort;

let orders = Order::objects()
    .order_by([
        <Order as Model>::Fields::is_fulfilled.asc(),
        <Order as Model>::Fields::quantity.desc(),
    ])
    .all(&db)
    .await?;
```

On top of the usual `asc`/`desc`, there's `nulls_first`/`nulls_last` to control where `NULL`s end up, and `by_values` that lets you sort by a hand-picked list of values (think `ORDER BY FIELD` in MySQL) - handy for things like showing in-stock products before preorders and backorders.

Second, the `query!` macro now supports substring, prefix, and suffix matching:

```rust
let customers = query!(Customer, $full_name.icontains("doe") && $is_verified == true)
    .all(&db)
    .await?;
```

`contains`, `starts_with`, and `ends_with` are case-sensitive, while their `i`-prefixed counterparts (`icontains`, `istarts_with`, `iends_with`) ignore case. The search strings are escaped for you, so a `%` or `_` in user input is matched literally. For anything more complex, there's `raw_like`, which accepts a glob-style pattern such as `"PKG*US"`, and Cot translates it to whatever the database backend expects.

### Raw SQL escape hatch

While the query API keeps getting more powerful, there will always be queries it can't express - complex joins, database-specific functions, aggregates. For those cases, Cot now provide a way to run raw SQL:

```rust
async fn get_customers_by_name(db: &Database, full_name: &str) -> cot::Result<()> {
    let customers = db
        .raw_as_with::<Customer>("SELECT * FROM customer WHERE full_name = ?", &[&full_name])
        .await?;
    println!("Customers: {customers:?}");
    Ok(())
}
```

`raw` and `raw_with` execute statements that don't return rows, while `raw_as` and `raw_as_with` map the returned rows onto a model. The `_with` variants bind the parameters safely, so please use these whenever the query depends on any external data - the plain variants execute the SQL string as-is.

### Rolling back migrations

Until now, migrations in Cot were a one-way street. In v0.8, you can roll them back:

```bash
cot migration rollback 0002 --dry-run
```

You can pass a full migration name (`m_0002_add_sku`), just its number (`0002`), or `zero` to undo everything for the app. Rolling back is exclusive - the migration you specify stays applied, and everything after it gets undone. Cot also follows the cross-app dependencies, so if another app has a migration that depends on the one being rolled back (e.g. through a foreign key), that one gets rolled back too. Because of that, it's a good idea to start with `--dry-run`, which prints the exact rollback plan without touching the database:

```
Rollback dry run

Target:
  app:       customers
  migration: m_0001_initial
  mode:      zero (all migrations in app rolled back)

Rollback plan:
  1. customers::m_0002_auto_20260527_004236
  2. customers::m_0001_initial

Summary:
  to roll back:     2
  skipped:          0
  database changes: none
```

Speaking of dependencies, there's also a new `cot migration graph` command that exports the migration dependency graph in GraphViz DOT or Mermaid format, so you can finally see how all of your apps' migrations fit together:

```bash
cot migration graph --format mermaid -o migrations.mmd
```

![Migration graph](/static/images/blog/2026-11-07-cot-v08-lazily-transactional-migration-graph.svg)

The migrations got [a brand-new guide page](https://cot.rs/guide/v0.8/databases/migrations/) too.

### New router

The router has been rewritten on top of a radix trie (powered by the [`matchit`](https://crates.io/crates/matchit) crate). Apart from making route matching faster, this also made the router stricter: ambiguous routes, such as `/foo/{bar}` and `/foo/{baz}`, are now rejected when the router is built, instead of silently picking one of them at runtime. This is a breaking change, so please have a look at [the upgrade guide](https://cot.rs/guide/v0.8/upgrade-guide/) if your app has a lot of routes.

The new router also brings wildcard parameters, which match the rest of the path:

```rust
async fn wildcard_path(Path(path): Path<String>) -> cot::Result<Html> {
    Ok(Html::new(format!("Passed path: {path}")))
}

Router::with_urls([
    Route::with_handler_and_name("/files/{*path}", wildcard_path, "files"),
])
```

This matches `/files/foo`, `/files/foo/bar`, and so on.

### `reverse!` with query parameters

The `reverse!` and `reverse_redirect!` macros can now append query parameters to the generated URL. The values are percent-encoded for you, so no more building query strings by hand:

```rust
let url = reverse!(request, "article", id = 5, query: { page = 2, sort = "new" })?;
// /article/5?page=2&sort=new
```

### Trailing slash redirects

The new `TrailingSlashMiddleware` redirects `GET` and `HEAD` requests to the URL with a trailing slash appended, if the original one doesn't match any route but the slashed one does. Query strings are preserved, too. The middleware is enabled by default in newly generated projects; for existing ones, just add it to your middleware stack:

```rust
fn middlewares(&self, handler: RootHandlerBuilder, context: &MiddlewareContext) -> RootHandler {
    handler
        // ...
        .middleware(TrailingSlashMiddleware::from_context(context))
        .build()
}
```

Related to this, `Redirect` can now use status codes other than `303 See Other` - there are `Redirect::permanent` (308), `Redirect::temporary` (307), and `Redirect::with_status_code` for everything else.

### Client IP address extractor

Getting the client's IP address sounds simple until there's a reverse proxy in front of your app. The new `ClientIpAddr` extractor handles that for you:

```rust
use cot::request::extractors::ClientIpAddr;

async fn handler(client_ip: ClientIpAddr) -> Html {
    Html::new(format!("Your IP is {}", client_ip.ip()))
}
```

By default, this is just the address of the peer that connected to the server. If you're running behind a proxy, configure which proxies are trusted, and Cot will read the client address from the `X-Forwarded-For` (or any other configured) header:

```toml
[client_ip]
proxies = ["10.0.0.0/8"]
headers = ["x-forwarded-for"]
```

### Smarter CLI

The `cot` CLI now discovers your project's binary, builds it when needed, and forwards any command it doesn't recognize to it. This means that the commands defined by your project (and the ones Cot ships inside every project binary, like `migration rollback` and `migration graph` above) can be run with `cot` directly, instead of going through `cargo run`. As a nice bonus, `cot --help` now lists both the built-in and project-defined commands in one place.

### Goodbye, axum

Cot used to depend on axum, but it only ever used it for `axum::serve` and its body type. In v0.8, we've replaced it with a small accept loop built directly on top of hyper. This removes axum from the critical path of a cold build - `cot-core` no longer has to wait for axum and hyper to compile - which makes the build times a bit shorter.

### Configuration reference

The guide now has [a full reference of the configuration file](https://cot.rs/guide/v0.8/configuration/), listing every table and key along with their types and default values. The page is generated automatically from the config structures, and the CI pipeline checks whether it's up to date, so - just like the guide's code snippets since v0.7 - it can never silently drift out of sync with the framework.

### Docs website improvements

We want the guide to be as useful as possible, and for that we need to know how people actually use it. To that end, [cot.rs](https://cot.rs/) now has [PostHog](https://posthog.com/) analytics. They're strictly opt-in: nothing is collected until you accept the cookie banner, and if your browser sends a Global Privacy Control or Do Not Track signal, analytics stay off regardless. You can change your choice at any time using the "Cookie settings" link in the footer.

We've also just added a "Was this helpful?" feedback widget to every guide page, so you can tell us directly which pages are great and which ones need more work, optionally leaving a comment.

### Other improvements

There are many more small fixes and improvements - check out the [GitHub release page](https://github.com/cot-rs/cot/releases/tag/cot-v0.8.0) to see all the changes!

## Project showcase: Hack the Space

As the project of this release, I've rewritten an old project of mine from Django to Cot. Hack the Space is a CTF-like web game where you need to solve various uncomplicated cyber security-related challenges (in the areas of cryptanalysis, steganography, reverse engineering, and webapp security) to progress. The game is available at [hackthespace.mackow.ski](https://hackthespace.mackow.ski/), and the source code is available on [GitHub](https://github.com/m4tx/hackthespace), if you are interested. Enjoy!

[![Hackthspace: home page](/static/images/blog/2026-11-07-cot-v08-lazily-transactional-hackthespace.png)](https://hackthespace.mackow.ski/)

## What's next?

The ORM is getting closer and closer to being feature-complete for most typical web apps, but there's still some work to be done there. We're also continuing with the plans for a background task system and the 1.0 release. If you have any ideas for the latter, let us know on [the discussion page](https://github.com/cot-rs/cot/issues/434)!

Ready to build? Head over to the [Cot website](https://cot.rs/guide/latest/) for the updated guide.

As always, Cot is community-driven. Whether you want to [report bugs, suggest features, or contribute code](https://github.com/cot-rs/cot), every bit of help is appreciated. Join us on [Discord](https://discord.cot.rs/) to share what you're building!

For the full list of changes, see the [official release notes](https://github.com/cot-rs/cot/releases/tag/cot-v0.8.0). Thank you to everyone who contributed to this release!
