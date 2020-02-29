use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = Event , extends = :: js_sys :: Object , js_name = TCPSocketEvent , typescript_name = TCPSocketEvent ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `TcpSocketEvent` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocketEvent)
    ///
    ///*This API requires the following crate features to be activated: `TcpSocketEvent`*
    pub type TcpSocketEvent;

    # [ wasm_bindgen ( structural , method , getter , js_class = "TCPSocketEvent" , js_name = data ) ]
    ///Getter for the `data` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocketEvent/data)
    ///
    ///*This API requires the following crate features to be activated: `TcpSocketEvent`*
    pub fn data(this: &TcpSocketEvent) -> ::wasm_bindgen::JsValue;

    #[wasm_bindgen(catch, constructor, js_class = "TCPSocketEvent")]
    ///The `new TcpSocketEvent(..)` constructor, creating a new instance of `TcpSocketEvent`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocketEvent/TCPSocketEvent)
    ///
    ///*This API requires the following crate features to be activated: `TcpSocketEvent`*
    pub fn new(type_: &str) -> Result<TcpSocketEvent, JsValue>;

    #[cfg(feature = "TcpSocketEventInit")]
    #[wasm_bindgen(catch, constructor, js_class = "TCPSocketEvent")]
    ///The `new TcpSocketEvent(..)` constructor, creating a new instance of `TcpSocketEvent`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocketEvent/TCPSocketEvent)
    ///
    ///*This API requires the following crate features to be activated: `TcpSocketEvent`, `TcpSocketEventInit`*
    pub fn new_with_event_init_dict(
        type_: &str,
        event_init_dict: &TcpSocketEventInit,
    ) -> Result<TcpSocketEvent, JsValue>;

}
