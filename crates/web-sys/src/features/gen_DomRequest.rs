use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = EventTarget , extends = :: js_sys :: Object , js_name = DOMRequest , typescript_name = DOMRequest ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DomRequest` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRequest)\n\n*This API requires the following crate features to be activated: `DomRequest`*"]
    pub type DomRequest;
    # [ wasm_bindgen ( structural , method , getter , js_name = readyState ) ]
    #[cfg(feature = "DomRequestReadyState")]
    #[doc = "Getter for the `readyState` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRequest/readyState)\n\n*This API requires the following crate features to be activated: `DomRequest`, `DomRequestReadyState`*"]
    pub fn ready_state(this: &DomRequest) -> DomRequestReadyState;
    # [ wasm_bindgen ( structural , method , getter , js_name = result ) ]
    #[doc = "Getter for the `result` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRequest/result)\n\n*This API requires the following crate features to be activated: `DomRequest`*"]
    pub fn result(this: &DomRequest) -> ::wasm_bindgen::JsValue;
    # [ wasm_bindgen ( structural , method , getter , js_name = error ) ]
    #[cfg(feature = "DomException")]
    #[doc = "Getter for the `error` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRequest/error)\n\n*This API requires the following crate features to be activated: `DomException`, `DomRequest`*"]
    pub fn error(this: &DomRequest) -> Option<DomException>;
    # [ wasm_bindgen ( structural , method , getter , js_name = onsuccess ) ]
    #[doc = "Getter for the `onsuccess` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRequest/onsuccess)\n\n*This API requires the following crate features to be activated: `DomRequest`*"]
    pub fn onsuccess(this: &DomRequest) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onsuccess ) ]
    #[doc = "Setter for the `onsuccess` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRequest/onsuccess)\n\n*This API requires the following crate features to be activated: `DomRequest`*"]
    pub fn set_onsuccess(this: &DomRequest, value: Option<::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onerror ) ]
    #[doc = "Getter for the `onerror` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRequest/onerror)\n\n*This API requires the following crate features to be activated: `DomRequest`*"]
    pub fn onerror(this: &DomRequest) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onerror ) ]
    #[doc = "Setter for the `onerror` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRequest/onerror)\n\n*This API requires the following crate features to be activated: `DomRequest`*"]
    pub fn set_onerror(this: &DomRequest, value: Option<::js_sys::Function>);
    # [ wasm_bindgen ( catch , method , structural , js_name = then ) ]
    #[doc = "The `then()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRequest/then)\n\n*This API requires the following crate features to be activated: `DomRequest`*"]
    pub fn then(this: &DomRequest) -> Result<::wasm_bindgen::JsValue, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = then ) ]
    #[doc = "The `then()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRequest/then)\n\n*This API requires the following crate features to be activated: `DomRequest`*"]
    pub fn then_with_fulfill_callback(
        this: &DomRequest,
        fulfill_callback: Option<&::js_sys::Function>,
    ) -> Result<::wasm_bindgen::JsValue, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = then ) ]
    #[doc = "The `then()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRequest/then)\n\n*This API requires the following crate features to be activated: `DomRequest`*"]
    pub fn then_with_fulfill_callback_and_reject_callback(
        this: &DomRequest,
        fulfill_callback: Option<&::js_sys::Function>,
        reject_callback: Option<&::js_sys::Function>,
    ) -> Result<::wasm_bindgen::JsValue, JsValue>;
}
