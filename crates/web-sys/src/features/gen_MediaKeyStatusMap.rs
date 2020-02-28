use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = MediaKeyStatusMap , typescript_name = MediaKeyStatusMap ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MediaKeyStatusMap` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeyStatusMap)\n\n*This API requires the following crate features to be activated: `MediaKeyStatusMap`*"]
    pub type MediaKeyStatusMap;
    # [ wasm_bindgen ( structural , method , getter , js_name = size ) ]
    #[doc = "Getter for the `size` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeyStatusMap/size)\n\n*This API requires the following crate features to be activated: `MediaKeyStatusMap`*"]
    pub fn size(this: &MediaKeyStatusMap) -> u32;
    # [ wasm_bindgen ( catch , method , structural , js_name = get ) ]
    #[doc = "The `get()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeyStatusMap/get)\n\n*This API requires the following crate features to be activated: `MediaKeyStatusMap`*"]
    pub fn get_with_buffer_source(
        this: &MediaKeyStatusMap,
        key_id: &::js_sys::Object,
    ) -> Result<::wasm_bindgen::JsValue, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = get ) ]
    #[doc = "The `get()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeyStatusMap/get)\n\n*This API requires the following crate features to be activated: `MediaKeyStatusMap`*"]
    pub fn get_with_u8_array(
        this: &MediaKeyStatusMap,
        key_id: &mut [u8],
    ) -> Result<::wasm_bindgen::JsValue, JsValue>;
    # [ wasm_bindgen ( method , structural , js_name = has ) ]
    #[doc = "The `has()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeyStatusMap/has)\n\n*This API requires the following crate features to be activated: `MediaKeyStatusMap`*"]
    pub fn has_with_buffer_source(this: &MediaKeyStatusMap, key_id: &::js_sys::Object) -> bool;
    # [ wasm_bindgen ( method , structural , js_name = has ) ]
    #[doc = "The `has()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeyStatusMap/has)\n\n*This API requires the following crate features to be activated: `MediaKeyStatusMap`*"]
    pub fn has_with_u8_array(this: &MediaKeyStatusMap, key_id: &mut [u8]) -> bool;
}
