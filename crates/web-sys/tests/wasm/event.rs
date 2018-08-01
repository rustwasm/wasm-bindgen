use futures::future::Future;
use js_sys::Promise;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use wasm_bindgen_test::*;
use web_sys::Event;

#[wasm_bindgen(module = "./tests/wasm/event.js")]
extern {
    fn new_event() -> Promise;
}

#[wasm_bindgen_test(async)]
fn event() -> impl Future<Item = (), Error = JsValue> {
    JsFuture::from(new_event())
        .map(Event::from)
        .map(|event| {
            // These should match `new Event`.
            assert!(event.bubbles());
            assert!(event.cancelable());
            assert!(event.composed());

            // The default behavior not initially prevented, but after
            // we call `prevent_default` it better be.
            assert!(!event.default_prevented());
            event.prevent_default();
            assert!(event.default_prevented());
        })
}
