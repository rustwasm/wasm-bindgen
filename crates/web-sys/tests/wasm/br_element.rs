use wasm_bindgen_test::*;
use wasm_bindgen::prelude::*;
use web_sys::HtmlBrElement;

#[wasm_bindgen(module = "./tests/wasm/element.js")]
extern {
    fn new_br() -> HtmlBrElement;
}

#[wasm_bindgen_test]
fn test_br_element() {
    let element = new_br();
    // Legacy clear method
    assert_eq!(element.clear(), "", "Shouldn't have a clear");
    element.set_clear("boop");
    assert_eq!(element.clear(), "boop", "Should have a clear");
}
