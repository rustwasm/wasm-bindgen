# `web-sys`

Raw bindings to Web APIs for projects using `wasm-bindgen`.

* [The `web-sys` section of the `wasm-bindgen`
  guide](https://rustwasm.github.io/wasm-bindgen/web-sys/index.html)
* [API Documentation](https://rustwasm.github.io/wasm-bindgen/api/web_sys/)

## Crate features

This crate by default contains very little when compiled as almost all of its
exposed APIs are gated by Cargo features. The exhaustive list of features can be
found in `crates/web-sys/Cargo.toml`, but the rule of thumb for `web-sys` is
that each type has its own cargo feature (named after the type). Using an API
requires enabling the features for all types used in the API, and APIs should
mention in the documentation what features they require.

## How to add an interface

If you don't see a particular web API in `web-sys`, here is how to add it.

1. Copy the WebIDL specification of the API and place it in a new file in the
   `webidls/unstable` folder. You can often find the IDL by going to the MDN
   docs page for the API, scrolling to the bottom, clicking the
   "Specifications" link, and scrolling to the bottom of the specification
   page. For example, the bottom of the [MDN
   docs](https://developer.mozilla.org/en-US/docs/Web/API/MediaSession) on the
   MediaSession API takes you to the
   [spec](https://w3c.github.io/mediasession/#the-mediasession-interface). The
   [very bottom](https://w3c.github.io/mediasession/#idl-index) of _that_ page
   is the IDL.
2. Annotate the functions that can throw with `[Throws]`
3. Run `cargo run --release --package wasm-bindgen-webidl -- webidls src/features`
4. Copy the contents of `features` into the `[features]` section of `Cargo.toml`
5. Run `git add .` to add all the generated files into git.
