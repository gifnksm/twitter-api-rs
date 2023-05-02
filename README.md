# twitter-api-rs

[![maintenance-status](https://img.shields.io/badge/maintenance-deprecated-red.svg)](https://doc.rust-lang.org/cargo/reference/manifest.html#the-badges-section)
[![license](https://img.shields.io/crates/l/twitter-api.svg)](LICENSE)
[![crates.io](https://img.shields.io/crates/v/twitter-api.svg)](https://crates.io/crates/twitter-api)
[![docs.rs](https://img.shields.io/docsrs/twitter-api/latest)](https://docs.rs/twitter-api/latest/)
[![rust 1.57.0+ badge](https://img.shields.io/badge/rust-1.57.0+-93450a.svg)](https://doc.rust-lang.org/cargo/reference/manifest.html#the-rust-version-field)
[![Rust CI](https://github.com/gifnksm/twitter-api-rs/actions/workflows/rust-ci.yml/badge.svg)](https://github.com/gifnksm/twitter-api-rs/actions/workflows/rust-ci.yml)
[![codecov](https://codecov.io/gh/gifnksm/twitter-api-rs/branch/master/graph/badge.svg?token=0NGaJWNYLq)](https://codecov.io/gh/gifnksm/twitter-api-rs)

**This crate is deprecated due to Twitter API change**

Unofficial Rust library for the Twitter API.

This library allows you to:

* get your timeline,
* update your timeline.

[Documentation](https://docs.rs/twitter-api)

## How to use?

Add this to your `Cargo.toml`:

```toml
[dependencies]
twitter-api = "0.6.0"
```

See [examples](./examples).

## Minimum supported Rust version (MSRV)

The minimum supported Rust version is **Rust 1.57.0**.
At least the last 3 versions of stable Rust are supported at any given time.

While a crate is pre-release status (0.x.x) it may have its MSRV bumped in a patch release.
Once a crate has reached 1.x, any MSRV bump will be accompanied with a new minor version.
