# `zepto_db` A Tiny DB built with Rust

⚠️ Work in progress 🏗️

Rust example project for educational pursuits.

## Developing with `cargo run`

Make sure to add `--` after `cargo run` followed by any commands and flags.

```rs
// example
cargo run -- --name "John McClane"
```

## Docs

Run `cargo doc --open` to generate docs and open them in the browser.

## Spec (MVP)

- define schemas
  - create table
  - specify columns
  - int32 and string support
- basic create, read (select) and delete of rows
- parser for instructions
- CLI
- select (with joins)
- joins (left join)

## Current functionality
Create a new table:
e.g.: create --table-name "buildings" --schema "address, number of floors"

Display a table:

