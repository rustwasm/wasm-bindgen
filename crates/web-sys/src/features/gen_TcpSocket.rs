use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = EventTarget , extends = :: js_sys :: Object , js_name = TCPSocket , typescript_name = TCPSocket ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `TcpSocket` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocket)\n\n*This API requires the following crate features to be activated: `TcpSocket`*"]
    pub type TcpSocket;
    # [ wasm_bindgen ( structural , method , getter , js_name = host ) ]
    #[doc = "Getter for the `host` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocket/host)\n\n*This API requires the following crate features to be activated: `TcpSocket`*"]
    pub fn host(this: &TcpSocket) -> String;
    # [ wasm_bindgen ( structural , method , getter , js_name = port ) ]
    #[doc = "Getter for the `port` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocket/port)\n\n*This API requires the following crate features to be activated: `TcpSocket`*"]
    pub fn port(this: &TcpSocket) -> u16;
    # [ wasm_bindgen ( structural , method , getter , js_name = ssl ) ]
    #[doc = "Getter for the `ssl` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocket/ssl)\n\n*This API requires the following crate features to be activated: `TcpSocket`*"]
    pub fn ssl(this: &TcpSocket) -> bool;
    # [ wasm_bindgen ( structural , method , getter , js_name = bufferedAmount ) ]
    #[doc = "Getter for the `bufferedAmount` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocket/bufferedAmount)\n\n*This API requires the following crate features to be activated: `TcpSocket`*"]
    pub fn buffered_amount(this: &TcpSocket) -> f64;
    # [ wasm_bindgen ( structural , method , getter , js_name = readyState ) ]
    #[cfg(feature = "TcpReadyState")]
    #[doc = "Getter for the `readyState` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocket/readyState)\n\n*This API requires the following crate features to be activated: `TcpReadyState`, `TcpSocket`*"]
    pub fn ready_state(this: &TcpSocket) -> TcpReadyState;
    # [ wasm_bindgen ( structural , method , getter , js_name = binaryType ) ]
    #[cfg(feature = "TcpSocketBinaryType")]
    #[doc = "Getter for the `binaryType` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocket/binaryType)\n\n*This API requires the following crate features to be activated: `TcpSocket`, `TcpSocketBinaryType`*"]
    pub fn binary_type(this: &TcpSocket) -> TcpSocketBinaryType;
    # [ wasm_bindgen ( structural , method , getter , js_name = onopen ) ]
    #[doc = "Getter for the `onopen` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocket/onopen)\n\n*This API requires the following crate features to be activated: `TcpSocket`*"]
    pub fn onopen(this: &TcpSocket) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onopen ) ]
    #[doc = "Setter for the `onopen` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocket/onopen)\n\n*This API requires the following crate features to be activated: `TcpSocket`*"]
    pub fn set_onopen(this: &TcpSocket, value: Option<::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = ondrain ) ]
    #[doc = "Getter for the `ondrain` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocket/ondrain)\n\n*This API requires the following crate features to be activated: `TcpSocket`*"]
    pub fn ondrain(this: &TcpSocket) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = ondrain ) ]
    #[doc = "Setter for the `ondrain` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocket/ondrain)\n\n*This API requires the following crate features to be activated: `TcpSocket`*"]
    pub fn set_ondrain(this: &TcpSocket, value: Option<::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = ondata ) ]
    #[doc = "Getter for the `ondata` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocket/ondata)\n\n*This API requires the following crate features to be activated: `TcpSocket`*"]
    pub fn ondata(this: &TcpSocket) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = ondata ) ]
    #[doc = "Setter for the `ondata` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocket/ondata)\n\n*This API requires the following crate features to be activated: `TcpSocket`*"]
    pub fn set_ondata(this: &TcpSocket, value: Option<::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onerror ) ]
    #[doc = "Getter for the `onerror` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocket/onerror)\n\n*This API requires the following crate features to be activated: `TcpSocket`*"]
    pub fn onerror(this: &TcpSocket) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onerror ) ]
    #[doc = "Setter for the `onerror` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocket/onerror)\n\n*This API requires the following crate features to be activated: `TcpSocket`*"]
    pub fn set_onerror(this: &TcpSocket, value: Option<::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onclose ) ]
    #[doc = "Getter for the `onclose` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocket/onclose)\n\n*This API requires the following crate features to be activated: `TcpSocket`*"]
    pub fn onclose(this: &TcpSocket) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onclose ) ]
    #[doc = "Setter for the `onclose` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocket/onclose)\n\n*This API requires the following crate features to be activated: `TcpSocket`*"]
    pub fn set_onclose(this: &TcpSocket, value: Option<::js_sys::Function>);
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new TcpSocket(..)` constructor, creating a new instance of `TcpSocket`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocket/TCPSocket)\n\n*This API requires the following crate features to be activated: `TcpSocket`*"]
    pub fn new(this: &TcpSocket, host: &str, port: u16) -> Result<TcpSocket, JsValue>;
    #[cfg(feature = "SocketOptions")]
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new TcpSocket(..)` constructor, creating a new instance of `TcpSocket`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocket/TCPSocket)\n\n*This API requires the following crate features to be activated: `SocketOptions`, `TcpSocket`*"]
    pub fn new_with_options(
        this: &TcpSocket,
        host: &str,
        port: u16,
        options: &SocketOptions,
    ) -> Result<TcpSocket, JsValue>;
    # [ wasm_bindgen ( method , structural , js_name = close ) ]
    #[doc = "The `close()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocket/close)\n\n*This API requires the following crate features to be activated: `TcpSocket`*"]
    pub fn close(this: &TcpSocket);
    # [ wasm_bindgen ( catch , method , structural , js_name = resume ) ]
    #[doc = "The `resume()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocket/resume)\n\n*This API requires the following crate features to be activated: `TcpSocket`*"]
    pub fn resume(this: &TcpSocket) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = send ) ]
    #[doc = "The `send()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocket/send)\n\n*This API requires the following crate features to be activated: `TcpSocket`*"]
    pub fn send_with_str(this: &TcpSocket, data: &str) -> Result<bool, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = send ) ]
    #[doc = "The `send()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocket/send)\n\n*This API requires the following crate features to be activated: `TcpSocket`*"]
    pub fn send_with_array_buffer(
        this: &TcpSocket,
        data: &::js_sys::ArrayBuffer,
    ) -> Result<bool, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = send ) ]
    #[doc = "The `send()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocket/send)\n\n*This API requires the following crate features to be activated: `TcpSocket`*"]
    pub fn send_with_array_buffer_and_byte_offset(
        this: &TcpSocket,
        data: &::js_sys::ArrayBuffer,
        byte_offset: u32,
    ) -> Result<bool, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = send ) ]
    #[doc = "The `send()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocket/send)\n\n*This API requires the following crate features to be activated: `TcpSocket`*"]
    pub fn send_with_array_buffer_and_byte_offset_and_byte_length(
        this: &TcpSocket,
        data: &::js_sys::ArrayBuffer,
        byte_offset: u32,
        byte_length: u32,
    ) -> Result<bool, JsValue>;
    # [ wasm_bindgen ( method , structural , js_name = suspend ) ]
    #[doc = "The `suspend()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocket/suspend)\n\n*This API requires the following crate features to be activated: `TcpSocket`*"]
    pub fn suspend(this: &TcpSocket);
    # [ wasm_bindgen ( catch , method , structural , js_name = upgradeToSecure ) ]
    #[doc = "The `upgradeToSecure()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocket/upgradeToSecure)\n\n*This API requires the following crate features to be activated: `TcpSocket`*"]
    pub fn upgrade_to_secure(this: &TcpSocket) -> Result<(), JsValue>;
}
