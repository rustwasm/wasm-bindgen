#![allow(unused_imports)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = Event , extends = :: js_sys :: Object , js_name = ExtendableEvent , typescript_type = "ExtendableEvent")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ExtendableEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ExtendableEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ExtendableEvent`*"]
    pub type ExtendableEvent;
    #[wasm_bindgen(catch, constructor, js_class = "ExtendableEvent")]
    #[doc = "The `new ExtendableEvent(..)` constructor, creating a new instance of `ExtendableEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ExtendableEvent/ExtendableEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ExtendableEvent`*"]
    pub fn new(type_: &str) -> Result<ExtendableEvent, JsValue>;
    #[cfg(feature = "ExtendableEventInit")]
    #[wasm_bindgen(catch, constructor, js_class = "ExtendableEvent")]
    #[doc = "The `new ExtendableEvent(..)` constructor, creating a new instance of `ExtendableEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ExtendableEvent/ExtendableEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ExtendableEvent`, `ExtendableEventInit`*"]
    pub fn new_with_event_init_dict(
        type_: &str,
        event_init_dict: &ExtendableEventInit,
    ) -> Result<ExtendableEvent, JsValue>;
    # [wasm_bindgen (catch , method , structural , js_class = "ExtendableEvent" , js_name = waitUntil)]
    #[doc = "The `waitUntil()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ExtendableEvent/waitUntil)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ExtendableEvent`*"]
    #[doc = ""]
    #[doc = "Argument `p`: While the Promise can produce any JsValue as far as the type system is concerned, practically it is expected to contain a <code>&[::wasm_bindgen::JsValue]</code>."]
    pub fn wait_until(this: &ExtendableEvent, p: &::js_sys::Promise) -> Result<(), JsValue>;
}
