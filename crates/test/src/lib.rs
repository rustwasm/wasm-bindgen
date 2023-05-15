//! Runtime support for the `#[wasm_bindgen_test]` attribute
//!
//! More documentation can be found in the README for this crate!

#![deny(missing_docs)]

pub use wasm_bindgen_test_macro::wasm_bindgen_test;

// Custom allocator that only returns pointers in the 2GB-4GB range
// To ensure we actually support more than 2GB of memory
#[cfg(all(test, feature = "gg-alloc"))]
#[global_allocator]
static A: gg_alloc::GgAlloc<std::alloc::System> = gg_alloc::GgAlloc::new(std::alloc::System);

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
/// ```ignore
/// wasm_bindgen_test_configure!(foo bar baz);
/// ```
///
/// where all of `foo`, `bar`, and `baz`, would be recognized options to this
/// macro. The currently known options to this macro are:
///
/// * `run_in_browser` - requires that this test is run in a browser rather than
///   node.js, which is the default for executing tests.
/// * `run_in_worker` - requires that this test is run in a web worker rather than
///   node.js, which is the default for executing tests.
///
/// This macro may be invoked at most one time per test suite (an entire binary
/// like `tests/foo.rs`, not per module)
#[macro_export]
macro_rules! wasm_bindgen_test_configure {
    (run_in_browser $($others:tt)*) => (
        #[link_section = "__wasm_bindgen_test_unstable"]
        #[cfg(target_arch = "wasm32")]
        pub static __WBG_TEST_RUN_IN_BROWSER: [u8; 1] = [0x01];
        $crate::wasm_bindgen_test_configure!($($others)*);
    );
    (run_in_worker $($others:tt)*) => (
        #[link_section = "__wasm_bindgen_test_unstable"]
        #[cfg(target_arch = "wasm32")]
        pub static __WBG_TEST_RUN_IN_WORKER: [u8; 1] = [0x10];
        $crate::wasm_bindgen_test_configure!($($others)*);
    );
    () => ()
}

#[path = "rt/mod.rs"]
pub mod __rt;
