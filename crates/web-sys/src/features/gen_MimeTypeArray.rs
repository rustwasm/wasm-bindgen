use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = MimeTypeArray , typescript_name = MimeTypeArray ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MimeTypeArray` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MimeTypeArray)\n\n*This API requires the following crate features to be activated: `MimeTypeArray`*"]
    pub type MimeTypeArray;
    # [ wasm_bindgen ( structural , method , getter , js_class = "MimeTypeArray" , js_name = length ) ]
    #[doc = "Getter for the `length` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MimeTypeArray/length)\n\n*This API requires the following crate features to be activated: `MimeTypeArray`*"]
    pub fn length(this: &MimeTypeArray) -> u32;
    #[cfg(feature = "MimeType")]
    # [ wasm_bindgen ( method , structural , js_class = "MimeTypeArray" , js_name = item ) ]
    #[doc = "The `item()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MimeTypeArray/item)\n\n*This API requires the following crate features to be activated: `MimeType`, `MimeTypeArray`*"]
    pub fn item(this: &MimeTypeArray, index: u32) -> Option<MimeType>;
    #[cfg(feature = "MimeType")]
    # [ wasm_bindgen ( method , structural , js_class = "MimeTypeArray" , js_name = namedItem ) ]
    #[doc = "The `namedItem()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MimeTypeArray/namedItem)\n\n*This API requires the following crate features to be activated: `MimeType`, `MimeTypeArray`*"]
    pub fn named_item(this: &MimeTypeArray, name: &str) -> Option<MimeType>;
    #[cfg(feature = "MimeType")]
    #[wasm_bindgen(method, structural, js_class = "MimeTypeArray", indexing_getter)]
    #[doc = "Indexing getter.\n\n\n\n*This API requires the following crate features to be activated: `MimeType`, `MimeTypeArray`*"]
    pub fn get_with_index(this: &MimeTypeArray, index: u32) -> Option<MimeType>;
    #[cfg(feature = "MimeType")]
    #[wasm_bindgen(method, structural, js_class = "MimeTypeArray", indexing_getter)]
    #[doc = "Indexing getter.\n\n\n\n*This API requires the following crate features to be activated: `MimeType`, `MimeTypeArray`*"]
    pub fn get_with_name(this: &MimeTypeArray, name: &str) -> Option<MimeType>;
}
