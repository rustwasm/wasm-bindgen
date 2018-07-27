//! Runtime support for the `#[wasm_bindgen_test]` attribute
//!
//! More documentation can be found in the README for this crate!

#![feature(use_extern_macros)]
#![deny(missing_docs)]

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

/// A macro used to configured how this test is executed by the
/// `wasm-bindgen-test-runner` harness.
///
/// This macro is invoked as:
///
///     wasm_bindgen_test_configure!(foo bar baz);
///
/// where all of `foo`, `bar`, and `baz`, would be recognized options to this
/// macro. The currently known options to this macro are:
///
/// * `run_in_browser` - requires that this test is run in a browser rather than
///   node.js, which is the default for executing tests.
///
/// This macro may be invoked at most one time per test suite.
#[macro_export]
macro_rules! wasm_bindgen_test_configure {
    (run_in_browser $($others:tt)*) => (
        #[link_section = "__wasm_bindgen_test_unstable"]
        #[cfg(target_arch = "wasm32")]
        pub static __WBG_TEST_RUN_IN_BROWSER: [u8; 1] = [0x01];
        wasm_bindgen_test_configure!($($others)*);
    );
    () => ()
}

#[path = "rt/mod.rs"]
pub mod __rt;
