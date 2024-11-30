use wasm_bindgen::prelude::*;

#[wasm_bindgen]
fn foo() {}

#[wasm_bindgen]
pub const fn foo2() {}

#[wasm_bindgen]
struct Foo<T>(T);

#[wasm_bindgen]
#[rustfmt::skip]
extern "C" {
    static mut FOO: u32;

    #[wasm_bindgen(static_string)]
    static FOO2: JsString;

    #[wasm_bindgen(thread_local_v2, static_string)]
    static FOO3: JsString;

    static FOO4: JsString = "test";

    #[wasm_bindgen(static_string)]
    static FOO5: JsString = "test";

    pub fn foo3(x: i32, ...);
}

#[wasm_bindgen]
extern "system" {}

#[wasm_bindgen]
pub fn foo4<T>() {}
#[wasm_bindgen]
pub fn foo5<'a>() {}
#[wasm_bindgen]
pub fn foo6<'a, T>() {}

#[wasm_bindgen]
trait X {}

fn main() {}
