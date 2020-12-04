#![allow(unused_imports)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = EventTarget , extends = :: js_sys :: Object , js_name = PresentationConnectionList , typescript_type = "PresentationConnectionList")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PresentationConnectionList` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnectionList)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationConnectionList`*"]
    pub type PresentationConnectionList;
    # [wasm_bindgen (structural , method , getter , js_class = "PresentationConnectionList" , js_name = connections)]
    #[doc = "Getter for the `connections` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnectionList/connections)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationConnectionList`*"]
    #[doc = ""]
    #[doc = "Return value: While the iterable or array can produce any JsValue as far as the type system is concerned, practically it is expected to contain a <code>[PresentationConnection]</code>."]
    pub fn connections(this: &PresentationConnectionList) -> ::js_sys::Array;
    # [wasm_bindgen (structural , method , getter , js_class = "PresentationConnectionList" , js_name = onconnectionavailable)]
    #[doc = "Getter for the `onconnectionavailable` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnectionList/onconnectionavailable)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationConnectionList`*"]
    #[doc = ""]
    #[doc = "Return value: See the referenced MDN documentation or the IDL files for the signature of the callback inside the option."]
    pub fn onconnectionavailable(this: &PresentationConnectionList) -> Option<::js_sys::Function>;
    # [wasm_bindgen (structural , method , setter , js_class = "PresentationConnectionList" , js_name = onconnectionavailable)]
    #[doc = "Setter for the `onconnectionavailable` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnectionList/onconnectionavailable)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationConnectionList`*"]
    #[doc = ""]
    #[doc = "Argument: See the referenced MDN documentation or the IDL files for the signature of the callback inside the option."]
    pub fn set_onconnectionavailable(
        this: &PresentationConnectionList,
        value: Option<&::js_sys::Function>,
    );
}
