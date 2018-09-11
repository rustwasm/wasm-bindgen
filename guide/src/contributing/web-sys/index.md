# `web-sys`

The `web-sys` crate provides raw bindings to all of the Web's APIs, and its
source lives at `wasm-bindgen/crates/web-sys`.

The `web-sys` crate is **entirely** mechanically generated inside `build.rs`
using `wasm-bindgen`'s WebIDL frontend and the WebIDL interface definitions for
Web APIs. This means that `web-sys` isn't always the most ergonomic crate to
use, but it's intended to provide verified and correct bindings to the web
platform, and then better interfaces can be iterated on crates.io!
