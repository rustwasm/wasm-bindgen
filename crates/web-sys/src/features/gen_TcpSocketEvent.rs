use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = Event , extends = :: js_sys :: Object , js_name = TCPSocketEvent , typescript_name = TCPSocketEvent ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `TcpSocketEvent` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocketEvent)\n\n*This API requires the following crate features to be activated: `TcpSocketEvent`*"]
    pub type TcpSocketEvent;
    # [ wasm_bindgen ( structural , method , getter , js_name = data ) ]
    #[doc = "Getter for the `data` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocketEvent/data)\n\n*This API requires the following crate features to be activated: `TcpSocketEvent`*"]
    pub fn data(this: &TcpSocketEvent) -> ::wasm_bindgen::JsValue;
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new TcpSocketEvent(..)` constructor, creating a new instance of `TcpSocketEvent`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocketEvent/TCPSocketEvent)\n\n*This API requires the following crate features to be activated: `TcpSocketEvent`*"]
    pub fn new(this: &TcpSocketEvent, type_: &str) -> Result<TcpSocketEvent, JsValue>;
    #[cfg(feature = "TcpSocketEventInit")]
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new TcpSocketEvent(..)` constructor, creating a new instance of `TcpSocketEvent`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocketEvent/TCPSocketEvent)\n\n*This API requires the following crate features to be activated: `TcpSocketEvent`, `TcpSocketEventInit`*"]
    pub fn new_with_event_init_dict(
        this: &TcpSocketEvent,
        type_: &str,
        event_init_dict: &TcpSocketEventInit,
    ) -> Result<TcpSocketEvent, JsValue>;
}
