use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = EventTarget , extends = :: js_sys :: Object , js_name = Worker , typescript_type = "Worker" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `Worker` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Worker)
    ///
    ///*This API requires the following crate features to be activated: `Worker`*
    pub type Worker;

    # [ wasm_bindgen ( structural , method , getter , js_class = "Worker" , js_name = onmessage ) ]
    ///Getter for the `onmessage` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Worker/onmessage)
    ///
    ///*This API requires the following crate features to be activated: `Worker`*
    pub fn onmessage(this: &Worker) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Worker" , js_name = onmessage ) ]
    ///Setter for the `onmessage` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Worker/onmessage)
    ///
    ///*This API requires the following crate features to be activated: `Worker`*
    pub fn set_onmessage(this: &Worker, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Worker" , js_name = onmessageerror ) ]
    ///Getter for the `onmessageerror` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Worker/onmessageerror)
    ///
    ///*This API requires the following crate features to be activated: `Worker`*
    pub fn onmessageerror(this: &Worker) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Worker" , js_name = onmessageerror ) ]
    ///Setter for the `onmessageerror` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Worker/onmessageerror)
    ///
    ///*This API requires the following crate features to be activated: `Worker`*
    pub fn set_onmessageerror(this: &Worker, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Worker" , js_name = onerror ) ]
    ///Getter for the `onerror` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Worker/onerror)
    ///
    ///*This API requires the following crate features to be activated: `Worker`*
    pub fn onerror(this: &Worker) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Worker" , js_name = onerror ) ]
    ///Setter for the `onerror` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Worker/onerror)
    ///
    ///*This API requires the following crate features to be activated: `Worker`*
    pub fn set_onerror(this: &Worker, value: Option<&::js_sys::Function>);

    #[wasm_bindgen(catch, constructor, js_class = "Worker")]
    ///The `new Worker(..)` constructor, creating a new instance of `Worker`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Worker/Worker)
    ///
    ///*This API requires the following crate features to be activated: `Worker`*
    pub fn new(script_url: &str) -> Result<Worker, JsValue>;

    #[cfg(feature = "WorkerOptions")]
    #[wasm_bindgen(catch, constructor, js_class = "Worker")]
    ///The `new Worker(..)` constructor, creating a new instance of `Worker`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Worker/Worker)
    ///
    ///*This API requires the following crate features to be activated: `Worker`, `WorkerOptions`*
    pub fn new_with_options(script_url: &str, options: &WorkerOptions) -> Result<Worker, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Worker" , js_name = postMessage ) ]
    ///The `postMessage()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Worker/postMessage)
    ///
    ///*This API requires the following crate features to be activated: `Worker`*
    pub fn post_message(this: &Worker, message: &::wasm_bindgen::JsValue) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Worker" , js_name = postMessage ) ]
    ///The `postMessage()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Worker/postMessage)
    ///
    ///*This API requires the following crate features to be activated: `Worker`*
    pub fn post_message_with_transfer(
        this: &Worker,
        message: &::wasm_bindgen::JsValue,
        transfer: &::wasm_bindgen::JsValue,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( method , structural , js_class = "Worker" , js_name = terminate ) ]
    ///The `terminate()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Worker/terminate)
    ///
    ///*This API requires the following crate features to be activated: `Worker`*
    pub fn terminate(this: &Worker);

}
