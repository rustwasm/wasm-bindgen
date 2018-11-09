extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    type Foo;

    #[wasm_bindgen(method, structural, host_binding)]
    fn bar(this: &Foo);
}
