use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct A {
    data: u32,
}

// oh no, I forgot to add `#[wasm_bindgen]` to the impl block
impl A {
    #[wasm_bindgen(js_name = bar)]
    pub fn foo(&self) {}
}

#[wasm_bindgen]
extern "C" {
    type MyClass;

    // oops, I mixed up `self` and `this`
    #[wasm_bindgen(method)]
    fn render(self: &MyClass) -> String;
}

fn main() {}
