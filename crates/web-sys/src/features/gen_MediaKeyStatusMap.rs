use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = MediaKeyStatusMap , typescript_name = MediaKeyStatusMap ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `MediaKeyStatusMap` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeyStatusMap)
    ///
    ///*This API requires the following crate features to be activated: `MediaKeyStatusMap`*
    pub type MediaKeyStatusMap;

    # [ wasm_bindgen ( structural , method , getter , js_class = "MediaKeyStatusMap" , js_name = size ) ]
    ///Getter for the `size` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeyStatusMap/size)
    ///
    ///*This API requires the following crate features to be activated: `MediaKeyStatusMap`*
    pub fn size(this: &MediaKeyStatusMap) -> u32;

    # [ wasm_bindgen ( catch , method , structural , js_class = "MediaKeyStatusMap" , js_name = get ) ]
    ///The `get()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeyStatusMap/get)
    ///
    ///*This API requires the following crate features to be activated: `MediaKeyStatusMap`*
    pub fn get_with_buffer_source(
        this: &MediaKeyStatusMap,
        key_id: &::js_sys::Object,
    ) -> Result<::wasm_bindgen::JsValue, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "MediaKeyStatusMap" , js_name = get ) ]
    ///The `get()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeyStatusMap/get)
    ///
    ///*This API requires the following crate features to be activated: `MediaKeyStatusMap`*
    pub fn get_with_u8_array(
        this: &MediaKeyStatusMap,
        key_id: &mut [u8],
    ) -> Result<::wasm_bindgen::JsValue, JsValue>;

    # [ wasm_bindgen ( method , structural , js_class = "MediaKeyStatusMap" , js_name = has ) ]
    ///The `has()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeyStatusMap/has)
    ///
    ///*This API requires the following crate features to be activated: `MediaKeyStatusMap`*
    pub fn has_with_buffer_source(this: &MediaKeyStatusMap, key_id: &::js_sys::Object) -> bool;

    # [ wasm_bindgen ( method , structural , js_class = "MediaKeyStatusMap" , js_name = has ) ]
    ///The `has()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeyStatusMap/has)
    ///
    ///*This API requires the following crate features to be activated: `MediaKeyStatusMap`*
    pub fn has_with_u8_array(this: &MediaKeyStatusMap, key_id: &mut [u8]) -> bool;

}
