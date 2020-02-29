use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = FileSystem , typescript_type = "FileSystem" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `FileSystem` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystem)
    ///
    ///*This API requires the following crate features to be activated: `FileSystem`*
    pub type FileSystem;

    # [ wasm_bindgen ( structural , method , getter , js_class = "FileSystem" , js_name = name ) ]
    ///Getter for the `name` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystem/name)
    ///
    ///*This API requires the following crate features to be activated: `FileSystem`*
    pub fn name(this: &FileSystem) -> String;

    #[cfg(feature = "FileSystemDirectoryEntry")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "FileSystem" , js_name = root ) ]
    ///Getter for the `root` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystem/root)
    ///
    ///*This API requires the following crate features to be activated: `FileSystem`, `FileSystemDirectoryEntry`*
    pub fn root(this: &FileSystem) -> FileSystemDirectoryEntry;

}
