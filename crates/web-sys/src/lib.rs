//! Raw API bindings for Web APIs
//!
//! This is a procedurally generated crate from browser WebIDL which provides a
//! binding to all APIs that browsers provide on the web. The source files are maintained [in the
//! crate's webidl directory][1]. The generated API documentation captures most of the information
//! contained in there – if not, it refers back there.
//!
//! [1]: https://github.com/rustwasm/wasm-bindgen/tree/master/crates/web-sys/webidls
//!
//! This crate by default contains very little when compiled as almost all of
//! its exposed APIs are gated by Cargo features. The exhaustive list of
//! features can be found in `crates/web-sys/Cargo.toml`, but the rule of thumb
//! for `web-sys` is that each type has its own cargo feature (named after the
//! type). Using an API requires enabling the features for all types used in the
//! API, and APIs should mention in the documentation what features they
//! require.

#![doc(html_root_url = "https://docs.rs/web-sys/0.3")]
#![allow(deprecated)]

mod features;
pub use features::*;

/// Getter for the `Window` object
///
/// [MDN Documentation]
///
/// *This API requires the following crate features to be activated: `Window`*
///
/// [MDN Documentation]: https://developer.mozilla.org/en-US/docs/Web/API/Window
#[cfg(feature = "Window")]
pub fn window() -> Option<Window> {
    use wasm_bindgen::JsCast;

    js_sys::global().dyn_into::<Window>().ok()
}
