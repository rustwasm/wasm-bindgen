use js_sys::{Object, Promise};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use wasm_bindgen_test::*;
use web_sys::CustomEvent;

#[wasm_bindgen(module = "/tests/wasm/custom_event.js")]
extern "C" {
    fn new_custom_event() -> Promise;
}

#[wasm_bindgen_test]
async fn custom_event() {
    let result = JsFuture::from(new_custom_event()).await.unwrap();
    let event = CustomEvent::from(result);
    // All DOM interfaces should inherit from `Object`.
    assert!(event.is_instance_of::<Object>());
    let _: &Object = event.as_ref();

    // These should match `new Event`.
    assert!(event.bubbles());
    assert!(event.cancelable());
    assert!(event.composed());
    assert_eq!(event.detail().as_string().unwrap(), "detail");

    // The default behavior not initially prevented, but after
    // we call `prevent_default` it better be.
    assert!(!event.default_prevented());
    event.prevent_default();
    assert!(event.default_prevented());
}
