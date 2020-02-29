use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = Event , extends = :: js_sys :: Object , js_name = MediaEncryptedEvent , typescript_type = "MediaEncryptedEvent" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `MediaEncryptedEvent` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaEncryptedEvent)
    ///
    ///*This API requires the following crate features to be activated: `MediaEncryptedEvent`*
    pub type MediaEncryptedEvent;

    # [ wasm_bindgen ( structural , method , getter , js_class = "MediaEncryptedEvent" , js_name = initDataType ) ]
    ///Getter for the `initDataType` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaEncryptedEvent/initDataType)
    ///
    ///*This API requires the following crate features to be activated: `MediaEncryptedEvent`*
    pub fn init_data_type(this: &MediaEncryptedEvent) -> String;

    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "MediaEncryptedEvent" , js_name = initData ) ]
    ///Getter for the `initData` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaEncryptedEvent/initData)
    ///
    ///*This API requires the following crate features to be activated: `MediaEncryptedEvent`*
    pub fn init_data(this: &MediaEncryptedEvent) -> Result<Option<::js_sys::ArrayBuffer>, JsValue>;

    #[wasm_bindgen(catch, constructor, js_class = "MediaEncryptedEvent")]
    ///The `new MediaEncryptedEvent(..)` constructor, creating a new instance of `MediaEncryptedEvent`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaEncryptedEvent/MediaEncryptedEvent)
    ///
    ///*This API requires the following crate features to be activated: `MediaEncryptedEvent`*
    pub fn new(type_: &str) -> Result<MediaEncryptedEvent, JsValue>;

    #[cfg(feature = "MediaKeyNeededEventInit")]
    #[wasm_bindgen(catch, constructor, js_class = "MediaEncryptedEvent")]
    ///The `new MediaEncryptedEvent(..)` constructor, creating a new instance of `MediaEncryptedEvent`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaEncryptedEvent/MediaEncryptedEvent)
    ///
    ///*This API requires the following crate features to be activated: `MediaEncryptedEvent`, `MediaKeyNeededEventInit`*
    pub fn new_with_event_init_dict(
        type_: &str,
        event_init_dict: &MediaKeyNeededEventInit,
    ) -> Result<MediaEncryptedEvent, JsValue>;

}
