use wasm_bindgen::JsValue;
use wasm_bindgen_test::*;
use js_sys::*;

// TODO: not much you can do with `MapIterator` types yet :(

#[wasm_bindgen_test]
fn entries() {
    let map = Map::new();
    assert!(JsValue::from(map.entries()).is_object());
}

#[wasm_bindgen_test]
fn keys() {
    let map = Map::new();
    assert!(JsValue::from(map.keys()).is_object());
}

#[wasm_bindgen_test]
fn values() {
    let map = Map::new();
    assert!(JsValue::from(map.values()).is_object());
}
