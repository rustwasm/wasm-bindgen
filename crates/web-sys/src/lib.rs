//! Raw API bindings for Web APIs
//!
//! This is a procedurally generated crate from browser WebIDL which provides a
//! binding to all APIs that browser provide on the web.
//!
//! This crate by default contains very little when compiled as almost all of
//! its exposed APIs are gated by Cargo features. The exhaustive list of
//! features can be found in `crates/web-sys/Cargo.toml`, but the rule of thumb
//! for `web-sys` is that each type has its own cargo feature (named after the
//! type). Using an API requires enabling the features for all types used in the
//! API, and APIs should mention in the documentation what features they
//! require.

#![doc(html_root_url = "https://docs.rs/web-sys/0.2")]

extern crate wasm_bindgen;
extern crate js_sys;
use js_sys::Object;

#[cfg(feature = "Window")]
pub fn window() -> Option<Window> {
    use wasm_bindgen::{JsValue, JsCast};

    js_sys::Function::new_no_args("return this")
        .call0(&JsValue::undefined())
        .ok()?
        .dyn_into::<Window>()
        .ok()
}

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
