#![feature(use_extern_macros, wasm_import_module)]

extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

// Strings can both be passed in and received
#[wasm_bindgen]
#[no_mangle]
pub extern "C" fn concat(a: &str, b: &str) -> String {
    let mut a = a.to_string();
    a.push_str(b);
    return a;
}

// A struct will show up as a class on the JS side of things
#[wasm_bindgen]
pub struct Foo {
    contents: u32,
}

#[wasm_bindgen]
impl Foo {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Foo {
        Foo { contents: 0 }
    }

    // Methods can be defined with `&mut self` or `&self`, and arguments you
    // can pass to a normal free function also all work in methods.
    pub fn add(&mut self, amt: u32) -> u32 {
        self.contents += amt;
        return self.contents;
    }

    // You can also take a limited set of references to other types as well.
    pub fn add_other(&mut self, bar: &Bar) {
        self.contents += bar.contents;
    }

    // Ownership can work too!
    pub fn consume_other(&mut self, bar: Bar) {
        self.contents += bar.contents;
    }
}

#[wasm_bindgen]
pub struct Bar {
    contents: u32,
    opaque: JsValue, // defined in `wasm_bindgen`, imported via prelude
}

#[wasm_bindgen(module = "./awesome")] // what ES6 module to import from
extern "C" {
    fn bar_on_reset(to: &str, opaque: &JsValue);

    // We can import classes and annotate functionality on those classes as well
    type Awesome;
    #[wasm_bindgen(constructor)]
    fn new() -> Awesome;
    #[wasm_bindgen(method)]
    fn get_internal(this: &Awesome) -> u32;
}

#[wasm_bindgen]
impl Bar {
    pub fn from_str(s: &str, opaque: JsValue) -> Bar {
        let contents = s.parse().unwrap_or_else(|_| Awesome::new().get_internal());
        Bar { contents, opaque }
    }

    pub fn reset(&mut self, s: &str) {
        if let Ok(n) = s.parse() {
            bar_on_reset(s, &self.opaque);
            self.contents = n;
        }
    }
}
