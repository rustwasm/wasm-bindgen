use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = Event , extends = :: js_sys :: Object , js_name = TCPSocketErrorEvent , typescript_name = TCPSocketErrorEvent ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `TcpSocketErrorEvent` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocketErrorEvent)\n\n*This API requires the following crate features to be activated: `TcpSocketErrorEvent`*"]
    pub type TcpSocketErrorEvent;
    # [ wasm_bindgen ( structural , method , getter , js_name = name ) ]
    #[doc = "Getter for the `name` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocketErrorEvent/name)\n\n*This API requires the following crate features to be activated: `TcpSocketErrorEvent`*"]
    pub fn name(this: &TcpSocketErrorEvent) -> String;
    # [ wasm_bindgen ( structural , method , getter , js_name = message ) ]
    #[doc = "Getter for the `message` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocketErrorEvent/message)\n\n*This API requires the following crate features to be activated: `TcpSocketErrorEvent`*"]
    pub fn message(this: &TcpSocketErrorEvent) -> String;
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new TcpSocketErrorEvent(..)` constructor, creating a new instance of `TcpSocketErrorEvent`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocketErrorEvent/TCPSocketErrorEvent)\n\n*This API requires the following crate features to be activated: `TcpSocketErrorEvent`*"]
    pub fn new(this: &TcpSocketErrorEvent, type_: &str) -> Result<TcpSocketErrorEvent, JsValue>;
    #[cfg(feature = "TcpSocketErrorEventInit")]
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new TcpSocketErrorEvent(..)` constructor, creating a new instance of `TcpSocketErrorEvent`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocketErrorEvent/TCPSocketErrorEvent)\n\n*This API requires the following crate features to be activated: `TcpSocketErrorEvent`, `TcpSocketErrorEventInit`*"]
    pub fn new_with_event_init_dict(
        this: &TcpSocketErrorEvent,
        type_: &str,
        event_init_dict: &TcpSocketErrorEventInit,
    ) -> Result<TcpSocketErrorEvent, JsValue>;
}
