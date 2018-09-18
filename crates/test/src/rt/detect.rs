//! Runtime detection of whether we're in node.js or a browser.

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use js_sys;

#[wasm_bindgen]
extern {
    type This;
    #[wasm_bindgen(method, getter, structural, js_name = self)]
    fn self_(me: &This) -> JsValue;
}

/// Returns whether it's likely we're executing in a browser environment, as
/// opposed to node.js.
pub fn is_browser() -> bool {
    // Test whether we're in a browser by seeing if the `self` property is
    // defined on the global object, which should in turn only be true in
    // browsers.
    js_sys::global().unchecked_into::<This>().self_() != JsValue::undefined()
}
