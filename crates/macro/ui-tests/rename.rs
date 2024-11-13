use wasm_bindgen::prelude::*;

#[wasm_bindgen(auto_rename)]
fn foo_bar() {}

#[wasm_bindgen(auto_rename)]
extern "C" {
    #[wasm_bindgen(auto_rename)]
    fn foo_bar_again();

    #[wasm_bindgen(auto_rename)]
    type A;
}

#[wasm_bindgen(auto_rename)]
pub struct Foo {
    #[wasm_bindgen(auto_rename)]
    pub foo: u32,
}

#[wasm_bindgen(auto_rename)]
impl Foo {
    #[wasm_bindgen(auto_rename)]
    pub fn foo(&self) {}

    pub fn isShouldCauseAWarning(&self) {}
}

fn main() {}
