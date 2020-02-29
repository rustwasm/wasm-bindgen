use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = Event , extends = :: js_sys :: Object , js_name = TCPSocketErrorEvent , typescript_type = "TCPSocketErrorEvent" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `TcpSocketErrorEvent` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocketErrorEvent)
    ///
    ///*This API requires the following crate features to be activated: `TcpSocketErrorEvent`*
    pub type TcpSocketErrorEvent;

    # [ wasm_bindgen ( structural , method , getter , js_class = "TCPSocketErrorEvent" , js_name = name ) ]
    ///Getter for the `name` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocketErrorEvent/name)
    ///
    ///*This API requires the following crate features to be activated: `TcpSocketErrorEvent`*
    pub fn name(this: &TcpSocketErrorEvent) -> String;

    # [ wasm_bindgen ( structural , method , getter , js_class = "TCPSocketErrorEvent" , js_name = message ) ]
    ///Getter for the `message` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocketErrorEvent/message)
    ///
    ///*This API requires the following crate features to be activated: `TcpSocketErrorEvent`*
    pub fn message(this: &TcpSocketErrorEvent) -> String;

    #[wasm_bindgen(catch, constructor, js_class = "TCPSocketErrorEvent")]
    ///The `new TcpSocketErrorEvent(..)` constructor, creating a new instance of `TcpSocketErrorEvent`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocketErrorEvent/TCPSocketErrorEvent)
    ///
    ///*This API requires the following crate features to be activated: `TcpSocketErrorEvent`*
    pub fn new(type_: &str) -> Result<TcpSocketErrorEvent, JsValue>;

    #[cfg(feature = "TcpSocketErrorEventInit")]
    #[wasm_bindgen(catch, constructor, js_class = "TCPSocketErrorEvent")]
    ///The `new TcpSocketErrorEvent(..)` constructor, creating a new instance of `TcpSocketErrorEvent`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocketErrorEvent/TCPSocketErrorEvent)
    ///
    ///*This API requires the following crate features to be activated: `TcpSocketErrorEvent`, `TcpSocketErrorEventInit`*
    pub fn new_with_event_init_dict(
        type_: &str,
        event_init_dict: &TcpSocketErrorEventInit,
    ) -> Result<TcpSocketErrorEvent, JsValue>;

}
