use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = EventTarget , extends = :: js_sys :: Object , js_name = Worker , typescript_name = Worker ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `Worker` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Worker)\n\n*This API requires the following crate features to be activated: `Worker`*"]
    pub type Worker;
    # [ wasm_bindgen ( structural , method , getter , js_name = onmessage ) ]
    #[doc = "Getter for the `onmessage` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Worker/onmessage)\n\n*This API requires the following crate features to be activated: `Worker`*"]
    pub fn onmessage(this: &Worker) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onmessage ) ]
    #[doc = "Setter for the `onmessage` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Worker/onmessage)\n\n*This API requires the following crate features to be activated: `Worker`*"]
    pub fn set_onmessage(this: &Worker, value: Option<::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onmessageerror ) ]
    #[doc = "Getter for the `onmessageerror` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Worker/onmessageerror)\n\n*This API requires the following crate features to be activated: `Worker`*"]
    pub fn onmessageerror(this: &Worker) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onmessageerror ) ]
    #[doc = "Setter for the `onmessageerror` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Worker/onmessageerror)\n\n*This API requires the following crate features to be activated: `Worker`*"]
    pub fn set_onmessageerror(this: &Worker, value: Option<::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onerror ) ]
    #[doc = "Getter for the `onerror` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Worker/onerror)\n\n*This API requires the following crate features to be activated: `Worker`*"]
    pub fn onerror(this: &Worker) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onerror ) ]
    #[doc = "Setter for the `onerror` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Worker/onerror)\n\n*This API requires the following crate features to be activated: `Worker`*"]
    pub fn set_onerror(this: &Worker, value: Option<::js_sys::Function>);
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new Worker(..)` constructor, creating a new instance of `Worker`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Worker/Worker)\n\n*This API requires the following crate features to be activated: `Worker`*"]
    pub fn new(this: &Worker, script_url: &str) -> Result<Worker, JsValue>;
    #[cfg(feature = "WorkerOptions")]
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new Worker(..)` constructor, creating a new instance of `Worker`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Worker/Worker)\n\n*This API requires the following crate features to be activated: `Worker`, `WorkerOptions`*"]
    pub fn new_with_options(
        this: &Worker,
        script_url: &str,
        options: &WorkerOptions,
    ) -> Result<Worker, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = postMessage ) ]
    #[doc = "The `postMessage()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Worker/postMessage)\n\n*This API requires the following crate features to be activated: `Worker`*"]
    pub fn post_message(this: &Worker, message: &::wasm_bindgen::JsValue) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = postMessage ) ]
    #[doc = "The `postMessage()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Worker/postMessage)\n\n*This API requires the following crate features to be activated: `Worker`*"]
    pub fn post_message_with_transfer(
        this: &Worker,
        message: &::wasm_bindgen::JsValue,
        transfer: &::wasm_bindgen::JsValue,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( method , structural , js_name = terminate ) ]
    #[doc = "The `terminate()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Worker/terminate)\n\n*This API requires the following crate features to be activated: `Worker`*"]
    pub fn terminate(this: &Worker);
}
