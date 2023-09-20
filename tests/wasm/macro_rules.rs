//! This tests that the `wasm_bindgen` macro produces code that compiles for this use case.
//! `cargo test --target wasm32-unknown-unknown` will not run if this test breaks.
use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

macro_rules! my_export {
    ($i: ident, $s: ty) => {
        #[wasm_bindgen]
        pub fn $i(s: $s) {
        }
    }
}

my_export!(should_compile, &[i32]);
