use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = PushMessageData , typescript_name = PushMessageData ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `PushMessageData` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PushMessageData)
    ///
    ///*This API requires the following crate features to be activated: `PushMessageData`*
    pub type PushMessageData;

    # [ wasm_bindgen ( catch , method , structural , js_class = "PushMessageData" , js_name = arrayBuffer ) ]
    ///The `arrayBuffer()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PushMessageData/arrayBuffer)
    ///
    ///*This API requires the following crate features to be activated: `PushMessageData`*
    pub fn array_buffer(this: &PushMessageData) -> Result<::js_sys::ArrayBuffer, JsValue>;

    #[cfg(feature = "Blob")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "PushMessageData" , js_name = blob ) ]
    ///The `blob()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PushMessageData/blob)
    ///
    ///*This API requires the following crate features to be activated: `Blob`, `PushMessageData`*
    pub fn blob(this: &PushMessageData) -> Result<Blob, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "PushMessageData" , js_name = json ) ]
    ///The `json()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PushMessageData/json)
    ///
    ///*This API requires the following crate features to be activated: `PushMessageData`*
    pub fn json(this: &PushMessageData) -> Result<::wasm_bindgen::JsValue, JsValue>;

    # [ wasm_bindgen ( method , structural , js_class = "PushMessageData" , js_name = text ) ]
    ///The `text()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PushMessageData/text)
    ///
    ///*This API requires the following crate features to be activated: `PushMessageData`*
    pub fn text(this: &PushMessageData) -> String;

}
