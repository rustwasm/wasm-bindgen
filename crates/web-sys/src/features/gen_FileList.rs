use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = FileList , typescript_name = FileList ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `FileList` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileList)\n\n*This API requires the following crate features to be activated: `FileList`*"]
    pub type FileList;
    # [ wasm_bindgen ( structural , method , getter , js_class = "FileList" , js_name = length ) ]
    #[doc = "Getter for the `length` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileList/length)\n\n*This API requires the following crate features to be activated: `FileList`*"]
    pub fn length(this: &FileList) -> u32;
    #[cfg(feature = "File")]
    # [ wasm_bindgen ( method , structural , js_class = "FileList" , js_name = item ) ]
    #[doc = "The `item()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileList/item)\n\n*This API requires the following crate features to be activated: `File`, `FileList`*"]
    pub fn item(this: &FileList, index: u32) -> Option<File>;
    #[cfg(feature = "File")]
    #[wasm_bindgen(method, structural, js_class = "FileList", indexing_getter)]
    #[doc = "Indexing getter.\n\n\n\n*This API requires the following crate features to be activated: `File`, `FileList`*"]
    pub fn get(this: &FileList, index: u32) -> Option<File>;
}
