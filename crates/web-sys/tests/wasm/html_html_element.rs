use wasm_bindgen_test::*;
use wasm_bindgen::prelude::*;
use web_sys::HtmlHtmlElement;

#[wasm_bindgen(module = "./tests/wasm/element.js")]
extern {
    #[wasm_bindgen(js_name = new_html)]
    fn make_html() -> HtmlHtmlElement;
}

#[wasm_bindgen_test]
fn test_html_html_element() {
    let element = make_html();
    assert_eq!(element.version(), "", "Shouldn't have a version");
    element.set_version("4");
    assert_eq!(element.version(), "4", "Should have a version");
}
