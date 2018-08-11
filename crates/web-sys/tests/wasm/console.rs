use wasm_bindgen_test::*;
use wasm_bindgen::prelude::*;
use web_sys::console;

#[wasm_bindgen_test]
fn test_console() {
    console::time("test label");
    console::time_end("test label");
}
