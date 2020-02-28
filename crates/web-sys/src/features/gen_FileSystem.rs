use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = FileSystem , typescript_name = FileSystem ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `FileSystem` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystem)\n\n*This API requires the following crate features to be activated: `FileSystem`*"]
    pub type FileSystem;
    # [ wasm_bindgen ( structural , method , getter , js_name = name ) ]
    #[doc = "Getter for the `name` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystem/name)\n\n*This API requires the following crate features to be activated: `FileSystem`*"]
    pub fn name(this: &FileSystem) -> String;
    # [ wasm_bindgen ( structural , method , getter , js_name = root ) ]
    #[cfg(feature = "FileSystemDirectoryEntry")]
    #[doc = "Getter for the `root` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystem/root)\n\n*This API requires the following crate features to be activated: `FileSystem`, `FileSystemDirectoryEntry`*"]
    pub fn root(this: &FileSystem) -> FileSystemDirectoryEntry;
}
