use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = EventTarget , extends = :: js_sys :: Object , js_name = PresentationConnection , typescript_type = "PresentationConnection" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `PresentationConnection` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnection)
    ///
    ///*This API requires the following crate features to be activated: `PresentationConnection`*
    pub type PresentationConnection;

    # [ wasm_bindgen ( structural , method , getter , js_class = "PresentationConnection" , js_name = id ) ]
    ///Getter for the `id` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnection/id)
    ///
    ///*This API requires the following crate features to be activated: `PresentationConnection`*
    pub fn id(this: &PresentationConnection) -> String;

    # [ wasm_bindgen ( structural , method , getter , js_class = "PresentationConnection" , js_name = url ) ]
    ///Getter for the `url` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnection/url)
    ///
    ///*This API requires the following crate features to be activated: `PresentationConnection`*
    pub fn url(this: &PresentationConnection) -> String;

    #[cfg(feature = "PresentationConnectionState")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "PresentationConnection" , js_name = state ) ]
    ///Getter for the `state` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnection/state)
    ///
    ///*This API requires the following crate features to be activated: `PresentationConnection`, `PresentationConnectionState`*
    pub fn state(this: &PresentationConnection) -> PresentationConnectionState;

    # [ wasm_bindgen ( structural , method , getter , js_class = "PresentationConnection" , js_name = onconnect ) ]
    ///Getter for the `onconnect` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnection/onconnect)
    ///
    ///*This API requires the following crate features to be activated: `PresentationConnection`*
    pub fn onconnect(this: &PresentationConnection) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "PresentationConnection" , js_name = onconnect ) ]
    ///Setter for the `onconnect` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnection/onconnect)
    ///
    ///*This API requires the following crate features to be activated: `PresentationConnection`*
    pub fn set_onconnect(this: &PresentationConnection, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "PresentationConnection" , js_name = onclose ) ]
    ///Getter for the `onclose` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnection/onclose)
    ///
    ///*This API requires the following crate features to be activated: `PresentationConnection`*
    pub fn onclose(this: &PresentationConnection) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "PresentationConnection" , js_name = onclose ) ]
    ///Setter for the `onclose` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnection/onclose)
    ///
    ///*This API requires the following crate features to be activated: `PresentationConnection`*
    pub fn set_onclose(this: &PresentationConnection, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "PresentationConnection" , js_name = onterminate ) ]
    ///Getter for the `onterminate` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnection/onterminate)
    ///
    ///*This API requires the following crate features to be activated: `PresentationConnection`*
    pub fn onterminate(this: &PresentationConnection) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "PresentationConnection" , js_name = onterminate ) ]
    ///Setter for the `onterminate` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnection/onterminate)
    ///
    ///*This API requires the following crate features to be activated: `PresentationConnection`*
    pub fn set_onterminate(this: &PresentationConnection, value: Option<&::js_sys::Function>);

    #[cfg(feature = "PresentationConnectionBinaryType")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "PresentationConnection" , js_name = binaryType ) ]
    ///Getter for the `binaryType` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnection/binaryType)
    ///
    ///*This API requires the following crate features to be activated: `PresentationConnection`, `PresentationConnectionBinaryType`*
    pub fn binary_type(this: &PresentationConnection) -> PresentationConnectionBinaryType;

    #[cfg(feature = "PresentationConnectionBinaryType")]
    # [ wasm_bindgen ( structural , method , setter , js_class = "PresentationConnection" , js_name = binaryType ) ]
    ///Setter for the `binaryType` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnection/binaryType)
    ///
    ///*This API requires the following crate features to be activated: `PresentationConnection`, `PresentationConnectionBinaryType`*
    pub fn set_binary_type(this: &PresentationConnection, value: PresentationConnectionBinaryType);

    # [ wasm_bindgen ( structural , method , getter , js_class = "PresentationConnection" , js_name = onmessage ) ]
    ///Getter for the `onmessage` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnection/onmessage)
    ///
    ///*This API requires the following crate features to be activated: `PresentationConnection`*
    pub fn onmessage(this: &PresentationConnection) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "PresentationConnection" , js_name = onmessage ) ]
    ///Setter for the `onmessage` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnection/onmessage)
    ///
    ///*This API requires the following crate features to be activated: `PresentationConnection`*
    pub fn set_onmessage(this: &PresentationConnection, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( catch , method , structural , js_class = "PresentationConnection" , js_name = close ) ]
    ///The `close()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnection/close)
    ///
    ///*This API requires the following crate features to be activated: `PresentationConnection`*
    pub fn close(this: &PresentationConnection) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "PresentationConnection" , js_name = send ) ]
    ///The `send()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnection/send)
    ///
    ///*This API requires the following crate features to be activated: `PresentationConnection`*
    pub fn send_with_str(this: &PresentationConnection, data: &str) -> Result<(), JsValue>;

    #[cfg(feature = "Blob")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "PresentationConnection" , js_name = send ) ]
    ///The `send()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnection/send)
    ///
    ///*This API requires the following crate features to be activated: `Blob`, `PresentationConnection`*
    pub fn send_with_blob(this: &PresentationConnection, data: &Blob) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "PresentationConnection" , js_name = send ) ]
    ///The `send()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnection/send)
    ///
    ///*This API requires the following crate features to be activated: `PresentationConnection`*
    pub fn send_with_array_buffer(
        this: &PresentationConnection,
        data: &::js_sys::ArrayBuffer,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "PresentationConnection" , js_name = send ) ]
    ///The `send()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnection/send)
    ///
    ///*This API requires the following crate features to be activated: `PresentationConnection`*
    pub fn send_with_array_buffer_view(
        this: &PresentationConnection,
        data: &::js_sys::Object,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "PresentationConnection" , js_name = send ) ]
    ///The `send()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnection/send)
    ///
    ///*This API requires the following crate features to be activated: `PresentationConnection`*
    pub fn send_with_u8_array(
        this: &PresentationConnection,
        data: &mut [u8],
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "PresentationConnection" , js_name = terminate ) ]
    ///The `terminate()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnection/terminate)
    ///
    ///*This API requires the following crate features to be activated: `PresentationConnection`*
    pub fn terminate(this: &PresentationConnection) -> Result<(), JsValue>;

}
