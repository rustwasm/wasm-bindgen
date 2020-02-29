use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = MimeTypeArray , typescript_name = MimeTypeArray ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `MimeTypeArray` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MimeTypeArray)
    ///
    ///*This API requires the following crate features to be activated: `MimeTypeArray`*
    pub type MimeTypeArray;

    # [ wasm_bindgen ( structural , method , getter , js_class = "MimeTypeArray" , js_name = length ) ]
    ///Getter for the `length` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MimeTypeArray/length)
    ///
    ///*This API requires the following crate features to be activated: `MimeTypeArray`*
    pub fn length(this: &MimeTypeArray) -> u32;

    #[cfg(feature = "MimeType")]
    # [ wasm_bindgen ( method , structural , js_class = "MimeTypeArray" , js_name = item ) ]
    ///The `item()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MimeTypeArray/item)
    ///
    ///*This API requires the following crate features to be activated: `MimeType`, `MimeTypeArray`*
    pub fn item(this: &MimeTypeArray, index: u32) -> Option<MimeType>;

    #[cfg(feature = "MimeType")]
    # [ wasm_bindgen ( method , structural , js_class = "MimeTypeArray" , js_name = namedItem ) ]
    ///The `namedItem()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MimeTypeArray/namedItem)
    ///
    ///*This API requires the following crate features to be activated: `MimeType`, `MimeTypeArray`*
    pub fn named_item(this: &MimeTypeArray, name: &str) -> Option<MimeType>;

    #[cfg(feature = "MimeType")]
    #[wasm_bindgen(method, structural, js_class = "MimeTypeArray", indexing_getter)]
    ///Indexing getter.
    ///
    ///
    ///
    ///*This API requires the following crate features to be activated: `MimeType`, `MimeTypeArray`*
    pub fn get_with_index(this: &MimeTypeArray, index: u32) -> Option<MimeType>;

    #[cfg(feature = "MimeType")]
    #[wasm_bindgen(method, structural, js_class = "MimeTypeArray", indexing_getter)]
    ///Indexing getter.
    ///
    ///
    ///
    ///*This API requires the following crate features to be activated: `MimeType`, `MimeTypeArray`*
    pub fn get_with_name(this: &MimeTypeArray, name: &str) -> Option<MimeType>;

}
