use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = EventTarget , extends = :: js_sys :: Object , js_name = TCPServerSocket , typescript_name = TCPServerSocket ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `TcpServerSocket` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPServerSocket)\n\n*This API requires the following crate features to be activated: `TcpServerSocket`*"]
    pub type TcpServerSocket;
    # [ wasm_bindgen ( structural , method , getter , js_name = localPort ) ]
    #[doc = "Getter for the `localPort` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPServerSocket/localPort)\n\n*This API requires the following crate features to be activated: `TcpServerSocket`*"]
    pub fn local_port(this: &TcpServerSocket) -> u16;
    # [ wasm_bindgen ( structural , method , getter , js_name = onconnect ) ]
    #[doc = "Getter for the `onconnect` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPServerSocket/onconnect)\n\n*This API requires the following crate features to be activated: `TcpServerSocket`*"]
    pub fn onconnect(this: &TcpServerSocket) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onconnect ) ]
    #[doc = "Setter for the `onconnect` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPServerSocket/onconnect)\n\n*This API requires the following crate features to be activated: `TcpServerSocket`*"]
    pub fn set_onconnect(this: &TcpServerSocket, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onerror ) ]
    #[doc = "Getter for the `onerror` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPServerSocket/onerror)\n\n*This API requires the following crate features to be activated: `TcpServerSocket`*"]
    pub fn onerror(this: &TcpServerSocket) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onerror ) ]
    #[doc = "Setter for the `onerror` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPServerSocket/onerror)\n\n*This API requires the following crate features to be activated: `TcpServerSocket`*"]
    pub fn set_onerror(this: &TcpServerSocket, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new TcpServerSocket(..)` constructor, creating a new instance of `TcpServerSocket`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPServerSocket/TCPServerSocket)\n\n*This API requires the following crate features to be activated: `TcpServerSocket`*"]
    pub fn new(this: &TcpServerSocket, port: u16) -> Result<TcpServerSocket, JsValue>;
    #[cfg(feature = "ServerSocketOptions")]
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new TcpServerSocket(..)` constructor, creating a new instance of `TcpServerSocket`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPServerSocket/TCPServerSocket)\n\n*This API requires the following crate features to be activated: `ServerSocketOptions`, `TcpServerSocket`*"]
    pub fn new_with_options(
        this: &TcpServerSocket,
        port: u16,
        options: &ServerSocketOptions,
    ) -> Result<TcpServerSocket, JsValue>;
    #[cfg(feature = "ServerSocketOptions")]
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new TcpServerSocket(..)` constructor, creating a new instance of `TcpServerSocket`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPServerSocket/TCPServerSocket)\n\n*This API requires the following crate features to be activated: `ServerSocketOptions`, `TcpServerSocket`*"]
    pub fn new_with_options_and_backlog(
        this: &TcpServerSocket,
        port: u16,
        options: &ServerSocketOptions,
        backlog: u16,
    ) -> Result<TcpServerSocket, JsValue>;
    # [ wasm_bindgen ( method , structural , js_name = close ) ]
    #[doc = "The `close()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPServerSocket/close)\n\n*This API requires the following crate features to be activated: `TcpServerSocket`*"]
    pub fn close(this: &TcpServerSocket);
}
