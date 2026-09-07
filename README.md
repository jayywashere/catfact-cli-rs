# catfact-cli-rs

> a small Rust CLI for fetching random cat facts because apparently i needed another cat project (copied C# but written in Rust, wow?)

## About

basically, i made this as a Rust learning project to practice working with REST APIs, asynchronous programming, JSON deserialization, and CLI structure

it uses the Cat Facts API to fetch random cat facts, with an optional maximum fact length and the ability to keep fetching facts in a loop

i also used my own Rust utility crate, `jstd`, for console input and output

## Features

- fetch random cat facts
- optional maximum fact length
- fetch multiple facts in a loop
- async HTTP requests with `reqwest` and `tokio`
- JSON deserialization with `serde`
- console I/O with `jstd` (i LOVE using this crate of mine in my Rust projects)

## Requirements

- Rust
- Cargo

## Running

run the program with:

```bash
cargo run
```

for a release build:

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

## What I Learned

- Rust modules and structs
- `Option<T>` and `Result<T, E>`
- error propagation with `?`
- `async` / `await`
- HTTP requests and REST APIs
- JSON deserialization with Serde
- `FromStr`
- using a personal Rust utility crate
- separating application, API, and model logic

## License

See [LICENSE](LICENSE).
