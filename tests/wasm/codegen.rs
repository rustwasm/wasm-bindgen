//! This tests that the `wasm_bindgen` macro produces code that compiles for these use cases.
//! `cargo test --target wasm32-unknown-unknown` will not run if any of these tests breaks.
use wasm_bindgen::prelude::*;

macro_rules! my_export {
    ($i: ident, $s: ty) => {
        #[wasm_bindgen]
        pub fn $i(_: $s) {}
    };
}

my_export!(should_compile, &[i32]);

#[wasm_bindgen]
pub struct A;

#[wasm_bindgen]
impl A {
    pub fn test_only_self() -> Self {
        A
    }
    pub fn test_option_self() -> Option<Self> {
        None
    }
    pub fn test_nested_self() -> Result<Option<Self>, String> {
        Ok(None)
    }
}
