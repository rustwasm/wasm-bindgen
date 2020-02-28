use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = EventTarget , extends = :: js_sys :: Object , js_name = MessagePort , typescript_name = MessagePort ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MessagePort` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MessagePort)\n\n*This API requires the following crate features to be activated: `MessagePort`*"]
    pub type MessagePort;
    # [ wasm_bindgen ( structural , method , getter , js_class = "MessagePort" , js_name = onmessage ) ]
    #[doc = "Getter for the `onmessage` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MessagePort/onmessage)\n\n*This API requires the following crate features to be activated: `MessagePort`*"]
    pub fn onmessage(this: &MessagePort) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_class = "MessagePort" , js_name = onmessage ) ]
    #[doc = "Setter for the `onmessage` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MessagePort/onmessage)\n\n*This API requires the following crate features to be activated: `MessagePort`*"]
    pub fn set_onmessage(this: &MessagePort, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_class = "MessagePort" , js_name = onmessageerror ) ]
    #[doc = "Getter for the `onmessageerror` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MessagePort/onmessageerror)\n\n*This API requires the following crate features to be activated: `MessagePort`*"]
    pub fn onmessageerror(this: &MessagePort) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_class = "MessagePort" , js_name = onmessageerror ) ]
    #[doc = "Setter for the `onmessageerror` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MessagePort/onmessageerror)\n\n*This API requires the following crate features to be activated: `MessagePort`*"]
    pub fn set_onmessageerror(this: &MessagePort, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( method , structural , js_class = "MessagePort" , js_name = close ) ]
    #[doc = "The `close()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MessagePort/close)\n\n*This API requires the following crate features to be activated: `MessagePort`*"]
    pub fn close(this: &MessagePort);
    # [ wasm_bindgen ( catch , method , structural , js_class = "MessagePort" , js_name = postMessage ) ]
    #[doc = "The `postMessage()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MessagePort/postMessage)\n\n*This API requires the following crate features to be activated: `MessagePort`*"]
    pub fn post_message(
        this: &MessagePort,
        message: &::wasm_bindgen::JsValue,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_class = "MessagePort" , js_name = postMessage ) ]
    #[doc = "The `postMessage()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MessagePort/postMessage)\n\n*This API requires the following crate features to be activated: `MessagePort`*"]
    pub fn post_message_with_transferable(
        this: &MessagePort,
        message: &::wasm_bindgen::JsValue,
        transferable: &::wasm_bindgen::JsValue,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( method , structural , js_class = "MessagePort" , js_name = start ) ]
    #[doc = "The `start()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MessagePort/start)\n\n*This API requires the following crate features to be activated: `MessagePort`*"]
    pub fn start(this: &MessagePort);
}
