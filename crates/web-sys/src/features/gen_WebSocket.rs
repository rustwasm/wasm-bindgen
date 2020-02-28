use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = EventTarget , extends = :: js_sys :: Object , js_name = WebSocket , typescript_name = WebSocket ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `WebSocket` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket)\n\n*This API requires the following crate features to be activated: `WebSocket`*"]
    pub type WebSocket;
    # [ wasm_bindgen ( structural , method , getter , js_name = url ) ]
    #[doc = "Getter for the `url` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/url)\n\n*This API requires the following crate features to be activated: `WebSocket`*"]
    pub fn url(this: &WebSocket) -> String;
    # [ wasm_bindgen ( structural , method , getter , js_name = readyState ) ]
    #[doc = "Getter for the `readyState` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/readyState)\n\n*This API requires the following crate features to be activated: `WebSocket`*"]
    pub fn ready_state(this: &WebSocket) -> u16;
    # [ wasm_bindgen ( structural , method , getter , js_name = bufferedAmount ) ]
    #[doc = "Getter for the `bufferedAmount` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/bufferedAmount)\n\n*This API requires the following crate features to be activated: `WebSocket`*"]
    pub fn buffered_amount(this: &WebSocket) -> u32;
    # [ wasm_bindgen ( structural , method , getter , js_name = onopen ) ]
    #[doc = "Getter for the `onopen` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/onopen)\n\n*This API requires the following crate features to be activated: `WebSocket`*"]
    pub fn onopen(this: &WebSocket) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onopen ) ]
    #[doc = "Setter for the `onopen` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/onopen)\n\n*This API requires the following crate features to be activated: `WebSocket`*"]
    pub fn set_onopen(this: &WebSocket, value: Option<::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onerror ) ]
    #[doc = "Getter for the `onerror` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/onerror)\n\n*This API requires the following crate features to be activated: `WebSocket`*"]
    pub fn onerror(this: &WebSocket) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onerror ) ]
    #[doc = "Setter for the `onerror` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/onerror)\n\n*This API requires the following crate features to be activated: `WebSocket`*"]
    pub fn set_onerror(this: &WebSocket, value: Option<::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onclose ) ]
    #[doc = "Getter for the `onclose` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/onclose)\n\n*This API requires the following crate features to be activated: `WebSocket`*"]
    pub fn onclose(this: &WebSocket) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onclose ) ]
    #[doc = "Setter for the `onclose` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/onclose)\n\n*This API requires the following crate features to be activated: `WebSocket`*"]
    pub fn set_onclose(this: &WebSocket, value: Option<::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = extensions ) ]
    #[doc = "Getter for the `extensions` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/extensions)\n\n*This API requires the following crate features to be activated: `WebSocket`*"]
    pub fn extensions(this: &WebSocket) -> String;
    # [ wasm_bindgen ( structural , method , getter , js_name = protocol ) ]
    #[doc = "Getter for the `protocol` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/protocol)\n\n*This API requires the following crate features to be activated: `WebSocket`*"]
    pub fn protocol(this: &WebSocket) -> String;
    # [ wasm_bindgen ( structural , method , getter , js_name = onmessage ) ]
    #[doc = "Getter for the `onmessage` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/onmessage)\n\n*This API requires the following crate features to be activated: `WebSocket`*"]
    pub fn onmessage(this: &WebSocket) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onmessage ) ]
    #[doc = "Setter for the `onmessage` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/onmessage)\n\n*This API requires the following crate features to be activated: `WebSocket`*"]
    pub fn set_onmessage(this: &WebSocket, value: Option<::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = binaryType ) ]
    #[cfg(feature = "BinaryType")]
    #[doc = "Getter for the `binaryType` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/binaryType)\n\n*This API requires the following crate features to be activated: `BinaryType`, `WebSocket`*"]
    pub fn binary_type(this: &WebSocket) -> BinaryType;
    # [ wasm_bindgen ( structural , method , setter , js_name = binaryType ) ]
    #[cfg(feature = "BinaryType")]
    #[doc = "Setter for the `binaryType` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/binaryType)\n\n*This API requires the following crate features to be activated: `BinaryType`, `WebSocket`*"]
    pub fn set_binary_type(this: &WebSocket, value: BinaryType);
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new WebSocket(..)` constructor, creating a new instance of `WebSocket`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/WebSocket)\n\n*This API requires the following crate features to be activated: `WebSocket`*"]
    pub fn new(this: &WebSocket, url: &str) -> Result<WebSocket, JsValue>;
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new WebSocket(..)` constructor, creating a new instance of `WebSocket`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/WebSocket)\n\n*This API requires the following crate features to be activated: `WebSocket`*"]
    pub fn new_with_str(this: &WebSocket, url: &str, protocols: &str)
        -> Result<WebSocket, JsValue>;
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new WebSocket(..)` constructor, creating a new instance of `WebSocket`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/WebSocket)\n\n*This API requires the following crate features to be activated: `WebSocket`*"]
    pub fn new_with_str_sequence(
        this: &WebSocket,
        url: &str,
        protocols: &::wasm_bindgen::JsValue,
    ) -> Result<WebSocket, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = close ) ]
    #[doc = "The `close()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/close)\n\n*This API requires the following crate features to be activated: `WebSocket`*"]
    pub fn close(this: &WebSocket) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = close ) ]
    #[doc = "The `close()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/close)\n\n*This API requires the following crate features to be activated: `WebSocket`*"]
    pub fn close_with_code(this: &WebSocket, code: u16) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = close ) ]
    #[doc = "The `close()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/close)\n\n*This API requires the following crate features to be activated: `WebSocket`*"]
    pub fn close_with_code_and_reason(
        this: &WebSocket,
        code: u16,
        reason: &str,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = send ) ]
    #[doc = "The `send()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/send)\n\n*This API requires the following crate features to be activated: `WebSocket`*"]
    pub fn send_with_str(this: &WebSocket, data: &str) -> Result<(), JsValue>;
    #[cfg(feature = "Blob")]
    # [ wasm_bindgen ( catch , method , structural , js_name = send ) ]
    #[doc = "The `send()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/send)\n\n*This API requires the following crate features to be activated: `Blob`, `WebSocket`*"]
    pub fn send_with_blob(this: &WebSocket, data: &Blob) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = send ) ]
    #[doc = "The `send()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/send)\n\n*This API requires the following crate features to be activated: `WebSocket`*"]
    pub fn send_with_array_buffer(
        this: &WebSocket,
        data: &::js_sys::ArrayBuffer,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = send ) ]
    #[doc = "The `send()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/send)\n\n*This API requires the following crate features to be activated: `WebSocket`*"]
    pub fn send_with_array_buffer_view(
        this: &WebSocket,
        data: &::js_sys::Object,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = send ) ]
    #[doc = "The `send()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/send)\n\n*This API requires the following crate features to be activated: `WebSocket`*"]
    pub fn send_with_u8_array(this: &WebSocket, data: &mut [u8]) -> Result<(), JsValue>;
}
impl WebSocket {
    pub const CONNECTING: u16 = 0i64 as u16;
    pub const OPEN: u16 = 1u64 as u16;
    pub const CLOSING: u16 = 2u64 as u16;
    pub const CLOSED: u16 = 3u64 as u16;
}
