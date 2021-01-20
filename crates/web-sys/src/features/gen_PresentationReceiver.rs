#![allow(unused_imports)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = PresentationReceiver , typescript_type = "PresentationReceiver")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PresentationReceiver` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationReceiver)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationReceiver`*"]
    pub type PresentationReceiver;
    # [wasm_bindgen (structural , catch , method , getter , js_class = "PresentationReceiver" , js_name = connectionList)]
    #[doc = "Getter for the `connectionList` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationReceiver/connectionList)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationReceiver`*"]
    #[doc = ""]
    #[doc = "Return value: While the Promise can produce any JsValue as far as the type system is concerned, practically it is expected to contain a <code>[PresentationConnectionList]</code>."]
    pub fn connection_list(this: &PresentationReceiver) -> Result<::js_sys::Promise, JsValue>;
}
