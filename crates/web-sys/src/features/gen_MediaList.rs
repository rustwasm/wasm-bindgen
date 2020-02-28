use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = MediaList , typescript_name = MediaList ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MediaList` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaList)\n\n*This API requires the following crate features to be activated: `MediaList`*"]
    pub type MediaList;
    # [ wasm_bindgen ( structural , method , getter , js_name = mediaText ) ]
    #[doc = "Getter for the `mediaText` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaList/mediaText)\n\n*This API requires the following crate features to be activated: `MediaList`*"]
    pub fn media_text(this: &MediaList) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = mediaText ) ]
    #[doc = "Setter for the `mediaText` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaList/mediaText)\n\n*This API requires the following crate features to be activated: `MediaList`*"]
    pub fn set_media_text(this: &MediaList, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_name = length ) ]
    #[doc = "Getter for the `length` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaList/length)\n\n*This API requires the following crate features to be activated: `MediaList`*"]
    pub fn length(this: &MediaList) -> u32;
    # [ wasm_bindgen ( catch , method , structural , js_name = appendMedium ) ]
    #[doc = "The `appendMedium()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaList/appendMedium)\n\n*This API requires the following crate features to be activated: `MediaList`*"]
    pub fn append_medium(this: &MediaList, new_medium: &str) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = deleteMedium ) ]
    #[doc = "The `deleteMedium()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaList/deleteMedium)\n\n*This API requires the following crate features to be activated: `MediaList`*"]
    pub fn delete_medium(this: &MediaList, old_medium: &str) -> Result<(), JsValue>;
    # [ wasm_bindgen ( method , structural , js_name = item ) ]
    #[doc = "The `item()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaList/item)\n\n*This API requires the following crate features to be activated: `MediaList`*"]
    pub fn item(this: &MediaList, index: u32) -> Option<String>;
    #[wasm_bindgen(method, structural, indexing_getter)]
    #[doc = "Indexing getter.\n\n\n\n*This API requires the following crate features to be activated: `MediaList`*"]
    pub fn get(this: &MediaList, index: u32) -> Option<String>;
}
