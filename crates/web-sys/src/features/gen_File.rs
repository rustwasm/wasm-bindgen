use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = Blob , extends = :: js_sys :: Object , js_name = File , typescript_name = File ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `File` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/File)\n\n*This API requires the following crate features to be activated: `File`*"]
    pub type File;
    # [ wasm_bindgen ( structural , method , getter , js_name = name ) ]
    #[doc = "Getter for the `name` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/File/name)\n\n*This API requires the following crate features to be activated: `File`*"]
    pub fn name(this: &File) -> String;
    # [ wasm_bindgen ( structural , method , getter , js_name = lastModified ) ]
    #[doc = "Getter for the `lastModified` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/File/lastModified)\n\n*This API requires the following crate features to be activated: `File`*"]
    pub fn last_modified(this: &File) -> f64;
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new File(..)` constructor, creating a new instance of `File`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/File/File)\n\n*This API requires the following crate features to be activated: `File`*"]
    pub fn new_with_buffer_source_sequence(
        this: &File,
        file_bits: &::wasm_bindgen::JsValue,
        file_name: &str,
    ) -> Result<File, JsValue>;
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new File(..)` constructor, creating a new instance of `File`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/File/File)\n\n*This API requires the following crate features to be activated: `File`*"]
    pub fn new_with_u8_array_sequence(
        this: &File,
        file_bits: &::wasm_bindgen::JsValue,
        file_name: &str,
    ) -> Result<File, JsValue>;
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new File(..)` constructor, creating a new instance of `File`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/File/File)\n\n*This API requires the following crate features to be activated: `File`*"]
    pub fn new_with_blob_sequence(
        this: &File,
        file_bits: &::wasm_bindgen::JsValue,
        file_name: &str,
    ) -> Result<File, JsValue>;
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new File(..)` constructor, creating a new instance of `File`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/File/File)\n\n*This API requires the following crate features to be activated: `File`*"]
    pub fn new_with_str_sequence(
        this: &File,
        file_bits: &::wasm_bindgen::JsValue,
        file_name: &str,
    ) -> Result<File, JsValue>;
    #[cfg(feature = "FilePropertyBag")]
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new File(..)` constructor, creating a new instance of `File`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/File/File)\n\n*This API requires the following crate features to be activated: `File`, `FilePropertyBag`*"]
    pub fn new_with_buffer_source_sequence_and_options(
        this: &File,
        file_bits: &::wasm_bindgen::JsValue,
        file_name: &str,
        options: &FilePropertyBag,
    ) -> Result<File, JsValue>;
    #[cfg(feature = "FilePropertyBag")]
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new File(..)` constructor, creating a new instance of `File`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/File/File)\n\n*This API requires the following crate features to be activated: `File`, `FilePropertyBag`*"]
    pub fn new_with_u8_array_sequence_and_options(
        this: &File,
        file_bits: &::wasm_bindgen::JsValue,
        file_name: &str,
        options: &FilePropertyBag,
    ) -> Result<File, JsValue>;
    #[cfg(feature = "FilePropertyBag")]
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new File(..)` constructor, creating a new instance of `File`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/File/File)\n\n*This API requires the following crate features to be activated: `File`, `FilePropertyBag`*"]
    pub fn new_with_blob_sequence_and_options(
        this: &File,
        file_bits: &::wasm_bindgen::JsValue,
        file_name: &str,
        options: &FilePropertyBag,
    ) -> Result<File, JsValue>;
    #[cfg(feature = "FilePropertyBag")]
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new File(..)` constructor, creating a new instance of `File`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/File/File)\n\n*This API requires the following crate features to be activated: `File`, `FilePropertyBag`*"]
    pub fn new_with_str_sequence_and_options(
        this: &File,
        file_bits: &::wasm_bindgen::JsValue,
        file_name: &str,
        options: &FilePropertyBag,
    ) -> Result<File, JsValue>;
}
