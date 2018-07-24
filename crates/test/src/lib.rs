//! Runtime support for the `#[wasm_bindgen_test]` attribute
//!
//! More documentation can be found in the README for this crate!

#![feature(use_extern_macros)]

extern crate wasm_bindgen_test_macro;
extern crate wasm_bindgen;
extern crate js_sys;
extern crate console_error_panic_hook;

pub use wasm_bindgen_test_macro::wasm_bindgen_test;

/// Helper macro which acts like `println!` only routes to `console.log`
/// instead.
#[macro_export]
macro_rules! console_log {
    ($($arg:tt)*) => (
        $crate::__rt::log(&format_args!($($arg)*))
    )
}

#[path = "rt/mod.rs"]
#[doc(hidden)]
pub mod __rt;
