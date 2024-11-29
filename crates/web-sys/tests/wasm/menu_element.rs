use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;
use web_sys::HtmlMenuElement;

#[wasm_bindgen(module = "/tests/wasm/element.js")]
extern "C" {
    fn new_menu() -> HtmlMenuElement;
}

#[wasm_bindgen_test]
fn test_menu_element() {
    let menu = new_menu();

    menu.set_compact(true);
    assert!(
        menu.compact(),
        "Menu should be compact after we set it to be compact."
    );

    menu.set_compact(false);
    assert!(
        !menu.compact(),
        "Menu should not be compact after we set it to be not-compact."
    );
}
