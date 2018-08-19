extern crate wasm_bindgen;
extern crate web_sys;

use wasm_bindgen::prelude::*;

// You can use the console bindings from web-sys...
use web_sys::console;

// ... or you can manually write the bindings yourself
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

#[wasm_bindgen]
pub fn run() {
    log("Hello from Rust!");
    log_u32(42);
    log_many("Logging", "many values!");

    console::log(JsValue::from("Another message from rust!"));
    console::log(JsValue::from(56u32));
}
