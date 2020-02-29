use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = Client , typescript_name = Client ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `Client` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Client)
    ///
    ///*This API requires the following crate features to be activated: `Client`*
    pub type Client;

    # [ wasm_bindgen ( structural , method , getter , js_class = "Client" , js_name = url ) ]
    ///Getter for the `url` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Client/url)
    ///
    ///*This API requires the following crate features to be activated: `Client`*
    pub fn url(this: &Client) -> String;

    #[cfg(feature = "FrameType")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "Client" , js_name = frameType ) ]
    ///Getter for the `frameType` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Client/frameType)
    ///
    ///*This API requires the following crate features to be activated: `Client`, `FrameType`*
    pub fn frame_type(this: &Client) -> FrameType;

    #[cfg(feature = "ClientType")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "Client" , js_name = type ) ]
    ///Getter for the `type` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Client/type)
    ///
    ///*This API requires the following crate features to be activated: `Client`, `ClientType`*
    pub fn type_(this: &Client) -> ClientType;

    # [ wasm_bindgen ( structural , method , getter , js_class = "Client" , js_name = id ) ]
    ///Getter for the `id` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Client/id)
    ///
    ///*This API requires the following crate features to be activated: `Client`*
    pub fn id(this: &Client) -> String;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Client" , js_name = postMessage ) ]
    ///The `postMessage()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Client/postMessage)
    ///
    ///*This API requires the following crate features to be activated: `Client`*
    pub fn post_message(this: &Client, message: &::wasm_bindgen::JsValue) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Client" , js_name = postMessage ) ]
    ///The `postMessage()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Client/postMessage)
    ///
    ///*This API requires the following crate features to be activated: `Client`*
    pub fn post_message_with_transfer(
        this: &Client,
        message: &::wasm_bindgen::JsValue,
        transfer: &::wasm_bindgen::JsValue,
    ) -> Result<(), JsValue>;

}
