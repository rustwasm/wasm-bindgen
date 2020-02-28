use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = MessageChannel , typescript_name = MessageChannel ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MessageChannel` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MessageChannel)\n\n*This API requires the following crate features to be activated: `MessageChannel`*"]
    pub type MessageChannel;
    # [ wasm_bindgen ( structural , method , getter , js_name = port1 ) ]
    #[cfg(feature = "MessagePort")]
    #[doc = "Getter for the `port1` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MessageChannel/port1)\n\n*This API requires the following crate features to be activated: `MessageChannel`, `MessagePort`*"]
    pub fn port1(this: &MessageChannel) -> MessagePort;
    # [ wasm_bindgen ( structural , method , getter , js_name = port2 ) ]
    #[cfg(feature = "MessagePort")]
    #[doc = "Getter for the `port2` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MessageChannel/port2)\n\n*This API requires the following crate features to be activated: `MessageChannel`, `MessagePort`*"]
    pub fn port2(this: &MessageChannel) -> MessagePort;
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new MessageChannel(..)` constructor, creating a new instance of `MessageChannel`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MessageChannel/MessageChannel)\n\n*This API requires the following crate features to be activated: `MessageChannel`*"]
    pub fn new(this: &MessageChannel) -> Result<MessageChannel, JsValue>;
}
