use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = EventTarget , extends = :: js_sys :: Object , js_name = BroadcastChannel , typescript_name = BroadcastChannel ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `BroadcastChannel` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BroadcastChannel)\n\n*This API requires the following crate features to be activated: `BroadcastChannel`*"]
    pub type BroadcastChannel;
    # [ wasm_bindgen ( structural , method , getter , js_class = "BroadcastChannel" , js_name = name ) ]
    #[doc = "Getter for the `name` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BroadcastChannel/name)\n\n*This API requires the following crate features to be activated: `BroadcastChannel`*"]
    pub fn name(this: &BroadcastChannel) -> String;
    # [ wasm_bindgen ( structural , method , getter , js_class = "BroadcastChannel" , js_name = onmessage ) ]
    #[doc = "Getter for the `onmessage` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BroadcastChannel/onmessage)\n\n*This API requires the following crate features to be activated: `BroadcastChannel`*"]
    pub fn onmessage(this: &BroadcastChannel) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_class = "BroadcastChannel" , js_name = onmessage ) ]
    #[doc = "Setter for the `onmessage` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BroadcastChannel/onmessage)\n\n*This API requires the following crate features to be activated: `BroadcastChannel`*"]
    pub fn set_onmessage(this: &BroadcastChannel, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_class = "BroadcastChannel" , js_name = onmessageerror ) ]
    #[doc = "Getter for the `onmessageerror` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BroadcastChannel/onmessageerror)\n\n*This API requires the following crate features to be activated: `BroadcastChannel`*"]
    pub fn onmessageerror(this: &BroadcastChannel) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_class = "BroadcastChannel" , js_name = onmessageerror ) ]
    #[doc = "Setter for the `onmessageerror` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BroadcastChannel/onmessageerror)\n\n*This API requires the following crate features to be activated: `BroadcastChannel`*"]
    pub fn set_onmessageerror(this: &BroadcastChannel, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(catch, js_class = "BroadcastChannel", constructor)]
    #[doc = "The `new BroadcastChannel(..)` constructor, creating a new instance of `BroadcastChannel`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BroadcastChannel/BroadcastChannel)\n\n*This API requires the following crate features to be activated: `BroadcastChannel`*"]
    pub fn new(this: &BroadcastChannel, channel: &str) -> Result<BroadcastChannel, JsValue>;
    # [ wasm_bindgen ( method , structural , js_class = "BroadcastChannel" , js_name = close ) ]
    #[doc = "The `close()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BroadcastChannel/close)\n\n*This API requires the following crate features to be activated: `BroadcastChannel`*"]
    pub fn close(this: &BroadcastChannel);
    # [ wasm_bindgen ( catch , method , structural , js_class = "BroadcastChannel" , js_name = postMessage ) ]
    #[doc = "The `postMessage()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BroadcastChannel/postMessage)\n\n*This API requires the following crate features to be activated: `BroadcastChannel`*"]
    pub fn post_message(
        this: &BroadcastChannel,
        message: &::wasm_bindgen::JsValue,
    ) -> Result<(), JsValue>;
}
