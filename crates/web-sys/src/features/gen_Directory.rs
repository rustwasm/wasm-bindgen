use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = Directory , typescript_type = "Directory" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `Directory` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Directory)
    ///
    ///*This API requires the following crate features to be activated: `Directory`*
    pub type Directory;

    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "Directory" , js_name = name ) ]
    ///Getter for the `name` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Directory/name)
    ///
    ///*This API requires the following crate features to be activated: `Directory`*
    pub fn name(this: &Directory) -> Result<String, JsValue>;

    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "Directory" , js_name = path ) ]
    ///Getter for the `path` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Directory/path)
    ///
    ///*This API requires the following crate features to be activated: `Directory`*
    pub fn path(this: &Directory) -> Result<String, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Directory" , js_name = getFiles ) ]
    ///The `getFiles()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Directory/getFiles)
    ///
    ///*This API requires the following crate features to be activated: `Directory`*
    pub fn get_files(this: &Directory) -> Result<::js_sys::Promise, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Directory" , js_name = getFiles ) ]
    ///The `getFiles()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Directory/getFiles)
    ///
    ///*This API requires the following crate features to be activated: `Directory`*
    pub fn get_files_with_recursive_flag(
        this: &Directory,
        recursive_flag: bool,
    ) -> Result<::js_sys::Promise, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Directory" , js_name = getFilesAndDirectories ) ]
    ///The `getFilesAndDirectories()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Directory/getFilesAndDirectories)
    ///
    ///*This API requires the following crate features to be activated: `Directory`*
    pub fn get_files_and_directories(this: &Directory) -> Result<::js_sys::Promise, JsValue>;

}
