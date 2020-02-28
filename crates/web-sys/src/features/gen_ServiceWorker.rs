use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = EventTarget , extends = :: js_sys :: Object , js_name = ServiceWorker , typescript_name = ServiceWorker ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ServiceWorker` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorker)\n\n*This API requires the following crate features to be activated: `ServiceWorker`*"]
    pub type ServiceWorker;
    # [ wasm_bindgen ( structural , method , getter , js_class = "ServiceWorker" , js_name = scriptURL ) ]
    #[doc = "Getter for the `scriptURL` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorker/scriptURL)\n\n*This API requires the following crate features to be activated: `ServiceWorker`*"]
    pub fn script_url(this: &ServiceWorker) -> String;
    # [ wasm_bindgen ( structural , method , getter , js_class = "ServiceWorker" , js_name = state ) ]
    #[cfg(feature = "ServiceWorkerState")]
    #[doc = "Getter for the `state` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorker/state)\n\n*This API requires the following crate features to be activated: `ServiceWorker`, `ServiceWorkerState`*"]
    pub fn state(this: &ServiceWorker) -> ServiceWorkerState;
    # [ wasm_bindgen ( structural , method , getter , js_class = "ServiceWorker" , js_name = onstatechange ) ]
    #[doc = "Getter for the `onstatechange` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorker/onstatechange)\n\n*This API requires the following crate features to be activated: `ServiceWorker`*"]
    pub fn onstatechange(this: &ServiceWorker) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_class = "ServiceWorker" , js_name = onstatechange ) ]
    #[doc = "Setter for the `onstatechange` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorker/onstatechange)\n\n*This API requires the following crate features to be activated: `ServiceWorker`*"]
    pub fn set_onstatechange(this: &ServiceWorker, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_class = "ServiceWorker" , js_name = onerror ) ]
    #[doc = "Getter for the `onerror` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorker/onerror)\n\n*This API requires the following crate features to be activated: `ServiceWorker`*"]
    pub fn onerror(this: &ServiceWorker) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_class = "ServiceWorker" , js_name = onerror ) ]
    #[doc = "Setter for the `onerror` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorker/onerror)\n\n*This API requires the following crate features to be activated: `ServiceWorker`*"]
    pub fn set_onerror(this: &ServiceWorker, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( catch , method , structural , js_class = "ServiceWorker" , js_name = postMessage ) ]
    #[doc = "The `postMessage()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorker/postMessage)\n\n*This API requires the following crate features to be activated: `ServiceWorker`*"]
    pub fn post_message(
        this: &ServiceWorker,
        message: &::wasm_bindgen::JsValue,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_class = "ServiceWorker" , js_name = postMessage ) ]
    #[doc = "The `postMessage()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorker/postMessage)\n\n*This API requires the following crate features to be activated: `ServiceWorker`*"]
    pub fn post_message_with_transferable(
        this: &ServiceWorker,
        message: &::wasm_bindgen::JsValue,
        transferable: &::wasm_bindgen::JsValue,
    ) -> Result<(), JsValue>;
}
