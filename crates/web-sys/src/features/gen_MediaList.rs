use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = MediaList , typescript_type = "MediaList" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `MediaList` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaList)
    ///
    ///*This API requires the following crate features to be activated: `MediaList`*
    pub type MediaList;

    # [ wasm_bindgen ( structural , method , getter , js_class = "MediaList" , js_name = mediaText ) ]
    ///Getter for the `mediaText` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaList/mediaText)
    ///
    ///*This API requires the following crate features to be activated: `MediaList`*
    pub fn media_text(this: &MediaList) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "MediaList" , js_name = mediaText ) ]
    ///Setter for the `mediaText` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaList/mediaText)
    ///
    ///*This API requires the following crate features to be activated: `MediaList`*
    pub fn set_media_text(this: &MediaList, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "MediaList" , js_name = length ) ]
    ///Getter for the `length` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaList/length)
    ///
    ///*This API requires the following crate features to be activated: `MediaList`*
    pub fn length(this: &MediaList) -> u32;

    # [ wasm_bindgen ( catch , method , structural , js_class = "MediaList" , js_name = appendMedium ) ]
    ///The `appendMedium()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaList/appendMedium)
    ///
    ///*This API requires the following crate features to be activated: `MediaList`*
    pub fn append_medium(this: &MediaList, new_medium: &str) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "MediaList" , js_name = deleteMedium ) ]
    ///The `deleteMedium()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaList/deleteMedium)
    ///
    ///*This API requires the following crate features to be activated: `MediaList`*
    pub fn delete_medium(this: &MediaList, old_medium: &str) -> Result<(), JsValue>;

    # [ wasm_bindgen ( method , structural , js_class = "MediaList" , js_name = item ) ]
    ///The `item()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaList/item)
    ///
    ///*This API requires the following crate features to be activated: `MediaList`*
    pub fn item(this: &MediaList, index: u32) -> Option<String>;

    #[wasm_bindgen(method, structural, js_class = "MediaList", indexing_getter)]
    ///Indexing getter.
    ///
    ///
    ///
    ///*This API requires the following crate features to be activated: `MediaList`*
    pub fn get(this: &MediaList, index: u32) -> Option<String>;

}
