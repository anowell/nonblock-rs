# nonblock-rs
Read available data from file descriptors without blocking

[Documentation](http://anowell.github.io/nonblock-rs/nonblock/)

[![Crates.io](https://img.shields.io/crates/v/nonblock.svg?maxAge=2592000)](https://crates.io/crates/nonblock)
[![Build Status](https://travis-ci.org/anowell/nonblock-rs.svg)](https://travis-ci.org/anowell/nonblock-rs)

## Examples

See [structure-stdio.rs](examples/structure-stdio.rs/) for an example usage.

## Build & Test

This project is built and tested with cargo:

```bash
cargo build
cargo test
cargo doc --no-deps
```

Pro-tip: before building docs, clone existing docs to track changes
```bash
git clone -b gh-pages git@github.com:anowell/nonblock-rs.git target/doc
```

