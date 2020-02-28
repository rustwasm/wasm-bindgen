use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = EventTarget , extends = :: js_sys :: Object , js_name = PresentationConnection , typescript_name = PresentationConnection ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PresentationConnection` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnection)\n\n*This API requires the following crate features to be activated: `PresentationConnection`*"]
    pub type PresentationConnection;
    # [ wasm_bindgen ( structural , method , getter , js_name = id ) ]
    #[doc = "Getter for the `id` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnection/id)\n\n*This API requires the following crate features to be activated: `PresentationConnection`*"]
    pub fn id(this: &PresentationConnection) -> String;
    # [ wasm_bindgen ( structural , method , getter , js_name = url ) ]
    #[doc = "Getter for the `url` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnection/url)\n\n*This API requires the following crate features to be activated: `PresentationConnection`*"]
    pub fn url(this: &PresentationConnection) -> String;
    # [ wasm_bindgen ( structural , method , getter , js_name = state ) ]
    #[cfg(feature = "PresentationConnectionState")]
    #[doc = "Getter for the `state` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnection/state)\n\n*This API requires the following crate features to be activated: `PresentationConnection`, `PresentationConnectionState`*"]
    pub fn state(this: &PresentationConnection) -> PresentationConnectionState;
    # [ wasm_bindgen ( structural , method , getter , js_name = onconnect ) ]
    #[doc = "Getter for the `onconnect` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnection/onconnect)\n\n*This API requires the following crate features to be activated: `PresentationConnection`*"]
    pub fn onconnect(this: &PresentationConnection) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onconnect ) ]
    #[doc = "Setter for the `onconnect` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnection/onconnect)\n\n*This API requires the following crate features to be activated: `PresentationConnection`*"]
    pub fn set_onconnect(this: &PresentationConnection, value: Option<::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onclose ) ]
    #[doc = "Getter for the `onclose` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnection/onclose)\n\n*This API requires the following crate features to be activated: `PresentationConnection`*"]
    pub fn onclose(this: &PresentationConnection) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onclose ) ]
    #[doc = "Setter for the `onclose` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnection/onclose)\n\n*This API requires the following crate features to be activated: `PresentationConnection`*"]
    pub fn set_onclose(this: &PresentationConnection, value: Option<::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onterminate ) ]
    #[doc = "Getter for the `onterminate` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnection/onterminate)\n\n*This API requires the following crate features to be activated: `PresentationConnection`*"]
    pub fn onterminate(this: &PresentationConnection) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onterminate ) ]
    #[doc = "Setter for the `onterminate` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnection/onterminate)\n\n*This API requires the following crate features to be activated: `PresentationConnection`*"]
    pub fn set_onterminate(this: &PresentationConnection, value: Option<::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = binaryType ) ]
    #[cfg(feature = "PresentationConnectionBinaryType")]
    #[doc = "Getter for the `binaryType` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnection/binaryType)\n\n*This API requires the following crate features to be activated: `PresentationConnection`, `PresentationConnectionBinaryType`*"]
    pub fn binary_type(this: &PresentationConnection) -> PresentationConnectionBinaryType;
    # [ wasm_bindgen ( structural , method , setter , js_name = binaryType ) ]
    #[cfg(feature = "PresentationConnectionBinaryType")]
    #[doc = "Setter for the `binaryType` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnection/binaryType)\n\n*This API requires the following crate features to be activated: `PresentationConnection`, `PresentationConnectionBinaryType`*"]
    pub fn set_binary_type(this: &PresentationConnection, value: PresentationConnectionBinaryType);
    # [ wasm_bindgen ( structural , method , getter , js_name = onmessage ) ]
    #[doc = "Getter for the `onmessage` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnection/onmessage)\n\n*This API requires the following crate features to be activated: `PresentationConnection`*"]
    pub fn onmessage(this: &PresentationConnection) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onmessage ) ]
    #[doc = "Setter for the `onmessage` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnection/onmessage)\n\n*This API requires the following crate features to be activated: `PresentationConnection`*"]
    pub fn set_onmessage(this: &PresentationConnection, value: Option<::js_sys::Function>);
    # [ wasm_bindgen ( catch , method , structural , js_name = close ) ]
    #[doc = "The `close()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnection/close)\n\n*This API requires the following crate features to be activated: `PresentationConnection`*"]
    pub fn close(this: &PresentationConnection) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = send ) ]
    #[doc = "The `send()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnection/send)\n\n*This API requires the following crate features to be activated: `PresentationConnection`*"]
    pub fn send_with_str(this: &PresentationConnection, data: &str) -> Result<(), JsValue>;
    #[cfg(feature = "Blob")]
    # [ wasm_bindgen ( catch , method , structural , js_name = send ) ]
    #[doc = "The `send()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnection/send)\n\n*This API requires the following crate features to be activated: `Blob`, `PresentationConnection`*"]
    pub fn send_with_blob(this: &PresentationConnection, data: &Blob) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = send ) ]
    #[doc = "The `send()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnection/send)\n\n*This API requires the following crate features to be activated: `PresentationConnection`*"]
    pub fn send_with_array_buffer(
        this: &PresentationConnection,
        data: &::js_sys::ArrayBuffer,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = send ) ]
    #[doc = "The `send()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnection/send)\n\n*This API requires the following crate features to be activated: `PresentationConnection`*"]
    pub fn send_with_array_buffer_view(
        this: &PresentationConnection,
        data: &::js_sys::Object,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = send ) ]
    #[doc = "The `send()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnection/send)\n\n*This API requires the following crate features to be activated: `PresentationConnection`*"]
    pub fn send_with_u8_array(
        this: &PresentationConnection,
        data: &mut [u8],
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = terminate ) ]
    #[doc = "The `terminate()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnection/terminate)\n\n*This API requires the following crate features to be activated: `PresentationConnection`*"]
    pub fn terminate(this: &PresentationConnection) -> Result<(), JsValue>;
}
