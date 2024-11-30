use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[rustfmt::skip]
extern "C" {
    #[wasm_bindgen(thread_local_v2, static_string)]
    static FOO: JsValue = "test";
}

fn main() {}
