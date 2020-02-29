use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = WorkerGlobalScope , extends = EventTarget , extends = :: js_sys :: Object , js_name = DedicatedWorkerGlobalScope , typescript_type = "DedicatedWorkerGlobalScope" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `DedicatedWorkerGlobalScope` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DedicatedWorkerGlobalScope)
    ///
    ///*This API requires the following crate features to be activated: `DedicatedWorkerGlobalScope`*
    pub type DedicatedWorkerGlobalScope;

    # [ wasm_bindgen ( structural , method , getter , js_class = "DedicatedWorkerGlobalScope" , js_name = name ) ]
    ///Getter for the `name` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DedicatedWorkerGlobalScope/name)
    ///
    ///*This API requires the following crate features to be activated: `DedicatedWorkerGlobalScope`*
    pub fn name(this: &DedicatedWorkerGlobalScope) -> String;

    # [ wasm_bindgen ( structural , method , getter , js_class = "DedicatedWorkerGlobalScope" , js_name = onmessage ) ]
    ///Getter for the `onmessage` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DedicatedWorkerGlobalScope/onmessage)
    ///
    ///*This API requires the following crate features to be activated: `DedicatedWorkerGlobalScope`*
    pub fn onmessage(this: &DedicatedWorkerGlobalScope) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "DedicatedWorkerGlobalScope" , js_name = onmessage ) ]
    ///Setter for the `onmessage` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DedicatedWorkerGlobalScope/onmessage)
    ///
    ///*This API requires the following crate features to be activated: `DedicatedWorkerGlobalScope`*
    pub fn set_onmessage(this: &DedicatedWorkerGlobalScope, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "DedicatedWorkerGlobalScope" , js_name = onmessageerror ) ]
    ///Getter for the `onmessageerror` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DedicatedWorkerGlobalScope/onmessageerror)
    ///
    ///*This API requires the following crate features to be activated: `DedicatedWorkerGlobalScope`*
    pub fn onmessageerror(this: &DedicatedWorkerGlobalScope) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "DedicatedWorkerGlobalScope" , js_name = onmessageerror ) ]
    ///Setter for the `onmessageerror` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DedicatedWorkerGlobalScope/onmessageerror)
    ///
    ///*This API requires the following crate features to be activated: `DedicatedWorkerGlobalScope`*
    pub fn set_onmessageerror(
        this: &DedicatedWorkerGlobalScope,
        value: Option<&::js_sys::Function>,
    );

    # [ wasm_bindgen ( method , structural , js_class = "DedicatedWorkerGlobalScope" , js_name = close ) ]
    ///The `close()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DedicatedWorkerGlobalScope/close)
    ///
    ///*This API requires the following crate features to be activated: `DedicatedWorkerGlobalScope`*
    pub fn close(this: &DedicatedWorkerGlobalScope);

    # [ wasm_bindgen ( catch , method , structural , js_class = "DedicatedWorkerGlobalScope" , js_name = postMessage ) ]
    ///The `postMessage()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DedicatedWorkerGlobalScope/postMessage)
    ///
    ///*This API requires the following crate features to be activated: `DedicatedWorkerGlobalScope`*
    pub fn post_message(
        this: &DedicatedWorkerGlobalScope,
        message: &::wasm_bindgen::JsValue,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "DedicatedWorkerGlobalScope" , js_name = postMessage ) ]
    ///The `postMessage()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DedicatedWorkerGlobalScope/postMessage)
    ///
    ///*This API requires the following crate features to be activated: `DedicatedWorkerGlobalScope`*
    pub fn post_message_with_transfer(
        this: &DedicatedWorkerGlobalScope,
        message: &::wasm_bindgen::JsValue,
        transfer: &::wasm_bindgen::JsValue,
    ) -> Result<(), JsValue>;

}
