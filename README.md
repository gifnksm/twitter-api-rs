# twitter-api-rs [![Build Status](https://travis-ci.org/gifnksm/twitter-api-rs.svg)](https://travis-ci.org/gifnksm/twitter-api-rs) [![Coverage Status](https://coveralls.io/repos/gifnksm/twitter-api-rs/badge.svg?branch=master&service=github)](https://coveralls.io/github/gifnksm/twitter-api-rs?branch=master)

Unofficial Rust library for the Twitter API.

This library allows you to:

*   get your timeline,
*   update your timeline.

This library uses the rust-oauth library (please to see ```oauth-client```).

[Documentation](https://gifnksm.github.io/twitter-api-rs)

## How to use?

Add this to your `Cargo.toml`:

```toml
[dependencies]
twitter-api = "*"
```

and this to your crate root:

```rust
extern crate twitter_api;
```

See [examples](./examples).
