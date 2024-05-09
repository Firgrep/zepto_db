# `zepto_db` A Tiny DB built with Rust

⚠️ Work in progress 🏗️

Rust example project for educational pursuits.

## Developing with `cargo run`

Make sure to add `--` after `cargo run` followed by any commands and flags.

```rs
// example
cargo run -- --name "John McClane"
```

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
