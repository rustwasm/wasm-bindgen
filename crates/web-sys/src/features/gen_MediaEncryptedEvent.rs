use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = Event , extends = :: js_sys :: Object , js_name = MediaEncryptedEvent , typescript_name = MediaEncryptedEvent ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MediaEncryptedEvent` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaEncryptedEvent)\n\n*This API requires the following crate features to be activated: `MediaEncryptedEvent`*"]
    pub type MediaEncryptedEvent;
    # [ wasm_bindgen ( structural , method , getter , js_class = "MediaEncryptedEvent" , js_name = initDataType ) ]
    #[doc = "Getter for the `initDataType` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaEncryptedEvent/initDataType)\n\n*This API requires the following crate features to be activated: `MediaEncryptedEvent`*"]
    pub fn init_data_type(this: &MediaEncryptedEvent) -> String;
    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "MediaEncryptedEvent" , js_name = initData ) ]
    #[doc = "Getter for the `initData` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaEncryptedEvent/initData)\n\n*This API requires the following crate features to be activated: `MediaEncryptedEvent`*"]
    pub fn init_data(this: &MediaEncryptedEvent) -> Result<Option<::js_sys::ArrayBuffer>, JsValue>;
    #[wasm_bindgen(catch, js_class = "MediaEncryptedEvent", constructor)]
    #[doc = "The `new MediaEncryptedEvent(..)` constructor, creating a new instance of `MediaEncryptedEvent`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaEncryptedEvent/MediaEncryptedEvent)\n\n*This API requires the following crate features to be activated: `MediaEncryptedEvent`*"]
    pub fn new(this: &MediaEncryptedEvent, type_: &str) -> Result<MediaEncryptedEvent, JsValue>;
    #[cfg(feature = "MediaKeyNeededEventInit")]
    #[wasm_bindgen(catch, js_class = "MediaEncryptedEvent", constructor)]
    #[doc = "The `new MediaEncryptedEvent(..)` constructor, creating a new instance of `MediaEncryptedEvent`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaEncryptedEvent/MediaEncryptedEvent)\n\n*This API requires the following crate features to be activated: `MediaEncryptedEvent`, `MediaKeyNeededEventInit`*"]
    pub fn new_with_event_init_dict(
        this: &MediaEncryptedEvent,
        type_: &str,
        event_init_dict: &MediaKeyNeededEventInit,
    ) -> Result<MediaEncryptedEvent, JsValue>;
}
