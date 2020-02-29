use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = EventTarget , extends = :: js_sys :: Object , js_name = TCPSocket , typescript_type = "TCPSocket" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `TcpSocket` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocket)
    ///
    ///*This API requires the following crate features to be activated: `TcpSocket`*
    pub type TcpSocket;

    # [ wasm_bindgen ( structural , method , getter , js_class = "TCPSocket" , js_name = host ) ]
    ///Getter for the `host` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocket/host)
    ///
    ///*This API requires the following crate features to be activated: `TcpSocket`*
    pub fn host(this: &TcpSocket) -> String;

    # [ wasm_bindgen ( structural , method , getter , js_class = "TCPSocket" , js_name = port ) ]
    ///Getter for the `port` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocket/port)
    ///
    ///*This API requires the following crate features to be activated: `TcpSocket`*
    pub fn port(this: &TcpSocket) -> u16;

    # [ wasm_bindgen ( structural , method , getter , js_class = "TCPSocket" , js_name = ssl ) ]
    ///Getter for the `ssl` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocket/ssl)
    ///
    ///*This API requires the following crate features to be activated: `TcpSocket`*
    pub fn ssl(this: &TcpSocket) -> bool;

    # [ wasm_bindgen ( structural , method , getter , js_class = "TCPSocket" , js_name = bufferedAmount ) ]
    ///Getter for the `bufferedAmount` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocket/bufferedAmount)
    ///
    ///*This API requires the following crate features to be activated: `TcpSocket`*
    pub fn buffered_amount(this: &TcpSocket) -> f64;

    #[cfg(feature = "TcpReadyState")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "TCPSocket" , js_name = readyState ) ]
    ///Getter for the `readyState` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocket/readyState)
    ///
    ///*This API requires the following crate features to be activated: `TcpReadyState`, `TcpSocket`*
    pub fn ready_state(this: &TcpSocket) -> TcpReadyState;

    #[cfg(feature = "TcpSocketBinaryType")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "TCPSocket" , js_name = binaryType ) ]
    ///Getter for the `binaryType` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocket/binaryType)
    ///
    ///*This API requires the following crate features to be activated: `TcpSocket`, `TcpSocketBinaryType`*
    pub fn binary_type(this: &TcpSocket) -> TcpSocketBinaryType;

    # [ wasm_bindgen ( structural , method , getter , js_class = "TCPSocket" , js_name = onopen ) ]
    ///Getter for the `onopen` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocket/onopen)
    ///
    ///*This API requires the following crate features to be activated: `TcpSocket`*
    pub fn onopen(this: &TcpSocket) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "TCPSocket" , js_name = onopen ) ]
    ///Setter for the `onopen` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocket/onopen)
    ///
    ///*This API requires the following crate features to be activated: `TcpSocket`*
    pub fn set_onopen(this: &TcpSocket, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "TCPSocket" , js_name = ondrain ) ]
    ///Getter for the `ondrain` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocket/ondrain)
    ///
    ///*This API requires the following crate features to be activated: `TcpSocket`*
    pub fn ondrain(this: &TcpSocket) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "TCPSocket" , js_name = ondrain ) ]
    ///Setter for the `ondrain` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocket/ondrain)
    ///
    ///*This API requires the following crate features to be activated: `TcpSocket`*
    pub fn set_ondrain(this: &TcpSocket, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "TCPSocket" , js_name = ondata ) ]
    ///Getter for the `ondata` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocket/ondata)
    ///
    ///*This API requires the following crate features to be activated: `TcpSocket`*
    pub fn ondata(this: &TcpSocket) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "TCPSocket" , js_name = ondata ) ]
    ///Setter for the `ondata` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocket/ondata)
    ///
    ///*This API requires the following crate features to be activated: `TcpSocket`*
    pub fn set_ondata(this: &TcpSocket, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "TCPSocket" , js_name = onerror ) ]
    ///Getter for the `onerror` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocket/onerror)
    ///
    ///*This API requires the following crate features to be activated: `TcpSocket`*
    pub fn onerror(this: &TcpSocket) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "TCPSocket" , js_name = onerror ) ]
    ///Setter for the `onerror` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocket/onerror)
    ///
    ///*This API requires the following crate features to be activated: `TcpSocket`*
    pub fn set_onerror(this: &TcpSocket, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "TCPSocket" , js_name = onclose ) ]
    ///Getter for the `onclose` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocket/onclose)
    ///
    ///*This API requires the following crate features to be activated: `TcpSocket`*
    pub fn onclose(this: &TcpSocket) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "TCPSocket" , js_name = onclose ) ]
    ///Setter for the `onclose` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocket/onclose)
    ///
    ///*This API requires the following crate features to be activated: `TcpSocket`*
    pub fn set_onclose(this: &TcpSocket, value: Option<&::js_sys::Function>);

    #[wasm_bindgen(catch, constructor, js_class = "TCPSocket")]
    ///The `new TcpSocket(..)` constructor, creating a new instance of `TcpSocket`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocket/TCPSocket)
    ///
    ///*This API requires the following crate features to be activated: `TcpSocket`*
    pub fn new(host: &str, port: u16) -> Result<TcpSocket, JsValue>;

    #[cfg(feature = "SocketOptions")]
    #[wasm_bindgen(catch, constructor, js_class = "TCPSocket")]
    ///The `new TcpSocket(..)` constructor, creating a new instance of `TcpSocket`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocket/TCPSocket)
    ///
    ///*This API requires the following crate features to be activated: `SocketOptions`, `TcpSocket`*
    pub fn new_with_options(
        host: &str,
        port: u16,
        options: &SocketOptions,
    ) -> Result<TcpSocket, JsValue>;

    # [ wasm_bindgen ( method , structural , js_class = "TCPSocket" , js_name = close ) ]
    ///The `close()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocket/close)
    ///
    ///*This API requires the following crate features to be activated: `TcpSocket`*
    pub fn close(this: &TcpSocket);

    # [ wasm_bindgen ( catch , method , structural , js_class = "TCPSocket" , js_name = resume ) ]
    ///The `resume()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocket/resume)
    ///
    ///*This API requires the following crate features to be activated: `TcpSocket`*
    pub fn resume(this: &TcpSocket) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "TCPSocket" , js_name = send ) ]
    ///The `send()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocket/send)
    ///
    ///*This API requires the following crate features to be activated: `TcpSocket`*
    pub fn send_with_str(this: &TcpSocket, data: &str) -> Result<bool, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "TCPSocket" , js_name = send ) ]
    ///The `send()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocket/send)
    ///
    ///*This API requires the following crate features to be activated: `TcpSocket`*
    pub fn send_with_array_buffer(
        this: &TcpSocket,
        data: &::js_sys::ArrayBuffer,
    ) -> Result<bool, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "TCPSocket" , js_name = send ) ]
    ///The `send()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocket/send)
    ///
    ///*This API requires the following crate features to be activated: `TcpSocket`*
    pub fn send_with_array_buffer_and_byte_offset(
        this: &TcpSocket,
        data: &::js_sys::ArrayBuffer,
        byte_offset: u32,
    ) -> Result<bool, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "TCPSocket" , js_name = send ) ]
    ///The `send()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocket/send)
    ///
    ///*This API requires the following crate features to be activated: `TcpSocket`*
    pub fn send_with_array_buffer_and_byte_offset_and_byte_length(
        this: &TcpSocket,
        data: &::js_sys::ArrayBuffer,
        byte_offset: u32,
        byte_length: u32,
    ) -> Result<bool, JsValue>;

    # [ wasm_bindgen ( method , structural , js_class = "TCPSocket" , js_name = suspend ) ]
    ///The `suspend()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocket/suspend)
    ///
    ///*This API requires the following crate features to be activated: `TcpSocket`*
    pub fn suspend(this: &TcpSocket);

    # [ wasm_bindgen ( catch , method , structural , js_class = "TCPSocket" , js_name = upgradeToSecure ) ]
    ///The `upgradeToSecure()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocket/upgradeToSecure)
    ///
    ///*This API requires the following crate features to be activated: `TcpSocket`*
    pub fn upgrade_to_secure(this: &TcpSocket) -> Result<(), JsValue>;

}
