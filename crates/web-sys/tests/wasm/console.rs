use wasm_bindgen_test::*;
use web_sys::console;

#[wasm_bindgen_test]
fn test_console() {
    console::time_using_label("test label");
    console::time_end_using_label("test label");
}
