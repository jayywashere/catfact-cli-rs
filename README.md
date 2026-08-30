# catfact-cli-rs

A small Rust CLI that fetches random cat facts from the [Cat Facts API](https://catfact.ninja/).

Built as a Rust learning project focused on REST APIs, asynchronous programming, JSON deserialization, and CLI structure.

## Features

* Fetch random cat facts
* Optional maximum fact length
* Fetch multiple facts in a loop
* Async HTTP requests with `reqwest` and `tokio`
* JSON deserialization with `serde`
* Console I/O with [`jstd`](https://github.com/jayywashere/jstd)

## Stack Used

* Rust
* Reqwest
* Serde
* Tokio
* jstd

## Structure

```text
src/
├── main.rs
├── app.rs
├── api/
│   ├── mod.rs
│   └── cat_fact_client.rs
└── models/
    ├── mod.rs
    └── cat_fact.rs
```

## Running

```bash
cargo run
```

For a release build:

```bash
cargo build --release
```

## Example

```text
Maximum fact length (leave blank for none): 20
Cats have 3 eyelids.
(20 characters)

Get another fact? (y/n): n
```

## What I Learned (And Will Forget Tomorrow)

* Rust modules and structs
* `Option<T>` and `Result<T, E>`
* Error propagation with `?`
* `async` / `await`
* HTTP requests and REST APIs
* JSON deserialization with Serde
* `FromStr`
* Using a personal Rust utility crate
* Separating application, API, and model logic