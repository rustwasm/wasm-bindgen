use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = EventTarget , extends = :: js_sys :: Object , js_name = WebSocket , typescript_type = "WebSocket" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `WebSocket` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket)
    ///
    ///*This API requires the following crate features to be activated: `WebSocket`*
    pub type WebSocket;

    # [ wasm_bindgen ( structural , method , getter , js_class = "WebSocket" , js_name = url ) ]
    ///Getter for the `url` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/url)
    ///
    ///*This API requires the following crate features to be activated: `WebSocket`*
    pub fn url(this: &WebSocket) -> String;

    # [ wasm_bindgen ( structural , method , getter , js_class = "WebSocket" , js_name = readyState ) ]
    ///Getter for the `readyState` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/readyState)
    ///
    ///*This API requires the following crate features to be activated: `WebSocket`*
    pub fn ready_state(this: &WebSocket) -> u16;

    # [ wasm_bindgen ( structural , method , getter , js_class = "WebSocket" , js_name = bufferedAmount ) ]
    ///Getter for the `bufferedAmount` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/bufferedAmount)
    ///
    ///*This API requires the following crate features to be activated: `WebSocket`*
    pub fn buffered_amount(this: &WebSocket) -> u32;

    # [ wasm_bindgen ( structural , method , getter , js_class = "WebSocket" , js_name = onopen ) ]
    ///Getter for the `onopen` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/onopen)
    ///
    ///*This API requires the following crate features to be activated: `WebSocket`*
    pub fn onopen(this: &WebSocket) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "WebSocket" , js_name = onopen ) ]
    ///Setter for the `onopen` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/onopen)
    ///
    ///*This API requires the following crate features to be activated: `WebSocket`*
    pub fn set_onopen(this: &WebSocket, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "WebSocket" , js_name = onerror ) ]
    ///Getter for the `onerror` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/onerror)
    ///
    ///*This API requires the following crate features to be activated: `WebSocket`*
    pub fn onerror(this: &WebSocket) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "WebSocket" , js_name = onerror ) ]
    ///Setter for the `onerror` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/onerror)
    ///
    ///*This API requires the following crate features to be activated: `WebSocket`*
    pub fn set_onerror(this: &WebSocket, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "WebSocket" , js_name = onclose ) ]
    ///Getter for the `onclose` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/onclose)
    ///
    ///*This API requires the following crate features to be activated: `WebSocket`*
    pub fn onclose(this: &WebSocket) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "WebSocket" , js_name = onclose ) ]
    ///Setter for the `onclose` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/onclose)
    ///
    ///*This API requires the following crate features to be activated: `WebSocket`*
    pub fn set_onclose(this: &WebSocket, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "WebSocket" , js_name = extensions ) ]
    ///Getter for the `extensions` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/extensions)
    ///
    ///*This API requires the following crate features to be activated: `WebSocket`*
    pub fn extensions(this: &WebSocket) -> String;

    # [ wasm_bindgen ( structural , method , getter , js_class = "WebSocket" , js_name = protocol ) ]
    ///Getter for the `protocol` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/protocol)
    ///
    ///*This API requires the following crate features to be activated: `WebSocket`*
    pub fn protocol(this: &WebSocket) -> String;

    # [ wasm_bindgen ( structural , method , getter , js_class = "WebSocket" , js_name = onmessage ) ]
    ///Getter for the `onmessage` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/onmessage)
    ///
    ///*This API requires the following crate features to be activated: `WebSocket`*
    pub fn onmessage(this: &WebSocket) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "WebSocket" , js_name = onmessage ) ]
    ///Setter for the `onmessage` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/onmessage)
    ///
    ///*This API requires the following crate features to be activated: `WebSocket`*
    pub fn set_onmessage(this: &WebSocket, value: Option<&::js_sys::Function>);

    #[cfg(feature = "BinaryType")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "WebSocket" , js_name = binaryType ) ]
    ///Getter for the `binaryType` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/binaryType)
    ///
    ///*This API requires the following crate features to be activated: `BinaryType`, `WebSocket`*
    pub fn binary_type(this: &WebSocket) -> BinaryType;

    #[cfg(feature = "BinaryType")]
    # [ wasm_bindgen ( structural , method , setter , js_class = "WebSocket" , js_name = binaryType ) ]
    ///Setter for the `binaryType` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/binaryType)
    ///
    ///*This API requires the following crate features to be activated: `BinaryType`, `WebSocket`*
    pub fn set_binary_type(this: &WebSocket, value: BinaryType);

    #[wasm_bindgen(catch, constructor, js_class = "WebSocket")]
    ///The `new WebSocket(..)` constructor, creating a new instance of `WebSocket`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/WebSocket)
    ///
    ///*This API requires the following crate features to be activated: `WebSocket`*
    pub fn new(url: &str) -> Result<WebSocket, JsValue>;

    #[wasm_bindgen(catch, constructor, js_class = "WebSocket")]
    ///The `new WebSocket(..)` constructor, creating a new instance of `WebSocket`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/WebSocket)
    ///
    ///*This API requires the following crate features to be activated: `WebSocket`*
    pub fn new_with_str(url: &str, protocols: &str) -> Result<WebSocket, JsValue>;

    #[wasm_bindgen(catch, constructor, js_class = "WebSocket")]
    ///The `new WebSocket(..)` constructor, creating a new instance of `WebSocket`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/WebSocket)
    ///
    ///*This API requires the following crate features to be activated: `WebSocket`*
    pub fn new_with_str_sequence(
        url: &str,
        protocols: &::wasm_bindgen::JsValue,
    ) -> Result<WebSocket, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "WebSocket" , js_name = close ) ]
    ///The `close()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/close)
    ///
    ///*This API requires the following crate features to be activated: `WebSocket`*
    pub fn close(this: &WebSocket) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "WebSocket" , js_name = close ) ]
    ///The `close()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/close)
    ///
    ///*This API requires the following crate features to be activated: `WebSocket`*
    pub fn close_with_code(this: &WebSocket, code: u16) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "WebSocket" , js_name = close ) ]
    ///The `close()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/close)
    ///
    ///*This API requires the following crate features to be activated: `WebSocket`*
    pub fn close_with_code_and_reason(
        this: &WebSocket,
        code: u16,
        reason: &str,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "WebSocket" , js_name = send ) ]
    ///The `send()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/send)
    ///
    ///*This API requires the following crate features to be activated: `WebSocket`*
    pub fn send_with_str(this: &WebSocket, data: &str) -> Result<(), JsValue>;

    #[cfg(feature = "Blob")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "WebSocket" , js_name = send ) ]
    ///The `send()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/send)
    ///
    ///*This API requires the following crate features to be activated: `Blob`, `WebSocket`*
    pub fn send_with_blob(this: &WebSocket, data: &Blob) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "WebSocket" , js_name = send ) ]
    ///The `send()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/send)
    ///
    ///*This API requires the following crate features to be activated: `WebSocket`*
    pub fn send_with_array_buffer(
        this: &WebSocket,
        data: &::js_sys::ArrayBuffer,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "WebSocket" , js_name = send ) ]
    ///The `send()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/send)
    ///
    ///*This API requires the following crate features to be activated: `WebSocket`*
    pub fn send_with_array_buffer_view(
        this: &WebSocket,
        data: &::js_sys::Object,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "WebSocket" , js_name = send ) ]
    ///The `send()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/send)
    ///
    ///*This API requires the following crate features to be activated: `WebSocket`*
    pub fn send_with_u8_array(this: &WebSocket, data: &mut [u8]) -> Result<(), JsValue>;

}

impl WebSocket {
    ///The `WebSocket.CONNECTING` const.
    ///
    ///*This API requires the following crate features to be activated: `WebSocket`*

    pub const CONNECTING: u16 = 0i64 as u16;

    ///The `WebSocket.OPEN` const.
    ///
    ///*This API requires the following crate features to be activated: `WebSocket`*

    pub const OPEN: u16 = 1u64 as u16;

    ///The `WebSocket.CLOSING` const.
    ///
    ///*This API requires the following crate features to be activated: `WebSocket`*

    pub const CLOSING: u16 = 2u64 as u16;

    ///The `WebSocket.CLOSED` const.
    ///
    ///*This API requires the following crate features to be activated: `WebSocket`*

    pub const CLOSED: u16 = 3u64 as u16;
}
