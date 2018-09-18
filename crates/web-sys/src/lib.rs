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
    use std::sync::atomic::{AtomicUsize, ATOMIC_USIZE_INIT, Ordering::SeqCst};
    use wasm_bindgen::{JsValue, JsCast};

    // Cached `Box<JsValue>`, if we've already executed this.
    //
    // 0 = not calculated
    // 1 = `None`
    // n = Some(n) == Some(Box<JsValue>)
    static WINDOW: AtomicUsize = ATOMIC_USIZE_INIT;

    match WINDOW.load(SeqCst) {
        0 => {}
        1 => return None,
        n => return unsafe { Some((*(n as *const JsValue)).clone().unchecked_into()) },
    }

    // Ok we don't have a cached value, let's load one! Manufacture a function
    // to get access to the `this` context and see if it's an instance of
    // `Window`.
    //
    // Note that we avoid `unwrap()` on `call0` to avoid code size bloat, we
    // just handle the `Err` case as returning `None`.
    let window = js_sys::Function::new_no_args("return this")
        .call0(&JsValue::undefined())
        .ok()
        .and_then(|w| w.dyn_into::<Window>().ok());

    match &window {
        None => WINDOW.store(1, SeqCst),
        Some(window) => {
            let window: &JsValue = window.as_ref();
            let ptr: *mut JsValue = Box::into_raw(Box::new(window.clone()));
            match WINDOW.compare_exchange(0, ptr as usize, SeqCst, SeqCst) {
                // We stored out value, relinquishing ownership of `ptr`
                Ok(_) => {}
                // Another thread one, drop our value
                Err(_) => unsafe { drop(Box::from_raw(ptr)) },
            }
        }
    }

    window
}

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
