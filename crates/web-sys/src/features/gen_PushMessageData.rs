use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = PushMessageData , typescript_name = PushMessageData ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PushMessageData` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PushMessageData)\n\n*This API requires the following crate features to be activated: `PushMessageData`*"]
    pub type PushMessageData;
    # [ wasm_bindgen ( catch , method , structural , js_class = "PushMessageData" , js_name = arrayBuffer ) ]
    #[doc = "The `arrayBuffer()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PushMessageData/arrayBuffer)\n\n*This API requires the following crate features to be activated: `PushMessageData`*"]
    pub fn array_buffer(this: &PushMessageData) -> Result<::js_sys::ArrayBuffer, JsValue>;
    #[cfg(feature = "Blob")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "PushMessageData" , js_name = blob ) ]
    #[doc = "The `blob()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PushMessageData/blob)\n\n*This API requires the following crate features to be activated: `Blob`, `PushMessageData`*"]
    pub fn blob(this: &PushMessageData) -> Result<Blob, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_class = "PushMessageData" , js_name = json ) ]
    #[doc = "The `json()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PushMessageData/json)\n\n*This API requires the following crate features to be activated: `PushMessageData`*"]
    pub fn json(this: &PushMessageData) -> Result<::wasm_bindgen::JsValue, JsValue>;
    # [ wasm_bindgen ( method , structural , js_class = "PushMessageData" , js_name = text ) ]
    #[doc = "The `text()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PushMessageData/text)\n\n*This API requires the following crate features to be activated: `PushMessageData`*"]
    pub fn text(this: &PushMessageData) -> String;
}
