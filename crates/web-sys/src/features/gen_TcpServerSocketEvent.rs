use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = Event , extends = :: js_sys :: Object , js_name = TCPServerSocketEvent , typescript_name = TCPServerSocketEvent ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `TcpServerSocketEvent` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPServerSocketEvent)\n\n*This API requires the following crate features to be activated: `TcpServerSocketEvent`*"]
    pub type TcpServerSocketEvent;
    # [ wasm_bindgen ( structural , method , getter , js_name = socket ) ]
    #[cfg(feature = "TcpSocket")]
    #[doc = "Getter for the `socket` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPServerSocketEvent/socket)\n\n*This API requires the following crate features to be activated: `TcpServerSocketEvent`, `TcpSocket`*"]
    pub fn socket(this: &TcpServerSocketEvent) -> TcpSocket;
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new TcpServerSocketEvent(..)` constructor, creating a new instance of `TcpServerSocketEvent`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPServerSocketEvent/TCPServerSocketEvent)\n\n*This API requires the following crate features to be activated: `TcpServerSocketEvent`*"]
    pub fn new(this: &TcpServerSocketEvent, type_: &str) -> Result<TcpServerSocketEvent, JsValue>;
    #[cfg(feature = "TcpServerSocketEventInit")]
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new TcpServerSocketEvent(..)` constructor, creating a new instance of `TcpServerSocketEvent`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPServerSocketEvent/TCPServerSocketEvent)\n\n*This API requires the following crate features to be activated: `TcpServerSocketEvent`, `TcpServerSocketEventInit`*"]
    pub fn new_with_event_init_dict(
        this: &TcpServerSocketEvent,
        type_: &str,
        event_init_dict: &TcpServerSocketEventInit,
    ) -> Result<TcpServerSocketEvent, JsValue>;
}
