use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = FileList , typescript_name = FileList ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `FileList` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileList)
    ///
    ///*This API requires the following crate features to be activated: `FileList`*
    pub type FileList;

    # [ wasm_bindgen ( structural , method , getter , js_class = "FileList" , js_name = length ) ]
    ///Getter for the `length` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileList/length)
    ///
    ///*This API requires the following crate features to be activated: `FileList`*
    pub fn length(this: &FileList) -> u32;

    #[cfg(feature = "File")]
    # [ wasm_bindgen ( method , structural , js_class = "FileList" , js_name = item ) ]
    ///The `item()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileList/item)
    ///
    ///*This API requires the following crate features to be activated: `File`, `FileList`*
    pub fn item(this: &FileList, index: u32) -> Option<File>;

    #[cfg(feature = "File")]
    #[wasm_bindgen(method, structural, js_class = "FileList", indexing_getter)]
    ///Indexing getter.
    ///
    ///
    ///
    ///*This API requires the following crate features to be activated: `File`, `FileList`*
    pub fn get(this: &FileList, index: u32) -> Option<File>;

}
