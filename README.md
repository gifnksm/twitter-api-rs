# twitter-api-rs

[![Crates.io](https://img.shields.io/crates/v/twitter-api.svg)](https://crates.io/crates/twitter-api)
[![Docs.rs](https://docs.rs/twitter-api/badge.svg)](https://docs.rs/twitter-api)
![LICENSE](https://img.shields.io/crates/l/twitter-api.svg)
[![Build Status](https://travis-ci.org/gifnksm/twitter-api-rs.svg)](https://travis-ci.org/gifnksm/twitter-api-rs)
![Maintenance](https://img.shields.io/badge/maintenance-passively--maintained-yellowgreen.svg)

Unofficial Rust library for the Twitter API.

This library allows you to:

* get your timeline,
* update your timeline.

[Documentation](https://docs.rs/twitter-api)

## How to use?

Add this to your `Cargo.toml`:

```toml
[dependencies]
twitter-api = "0.5"
```

See [examples](./examples).

## Minimum supported Rust version (MSRV)

The minimum supported Rust version is **Rust 1.57.0**.
At least the last 3 versions of stable Rust are supported at any given time.

While a crate is pre-release status (0.x.x) it may have its MSRV bumped in a patch release.
Once a crate has reached 1.x, any MSRV bump will be accompanied with a new minor version.
