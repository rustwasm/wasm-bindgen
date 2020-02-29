use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = MessageChannel , typescript_type = "MessageChannel" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `MessageChannel` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MessageChannel)
    ///
    ///*This API requires the following crate features to be activated: `MessageChannel`*
    pub type MessageChannel;

    #[cfg(feature = "MessagePort")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "MessageChannel" , js_name = port1 ) ]
    ///Getter for the `port1` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MessageChannel/port1)
    ///
    ///*This API requires the following crate features to be activated: `MessageChannel`, `MessagePort`*
    pub fn port1(this: &MessageChannel) -> MessagePort;

    #[cfg(feature = "MessagePort")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "MessageChannel" , js_name = port2 ) ]
    ///Getter for the `port2` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MessageChannel/port2)
    ///
    ///*This API requires the following crate features to be activated: `MessageChannel`, `MessagePort`*
    pub fn port2(this: &MessageChannel) -> MessagePort;

    #[wasm_bindgen(catch, constructor, js_class = "MessageChannel")]
    ///The `new MessageChannel(..)` constructor, creating a new instance of `MessageChannel`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MessageChannel/MessageChannel)
    ///
    ///*This API requires the following crate features to be activated: `MessageChannel`*
    pub fn new() -> Result<MessageChannel, JsValue>;

}
