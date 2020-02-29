use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = EventTarget , extends = :: js_sys :: Object , js_name = IDBMutableFile , typescript_type = "IDBMutableFile" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `IdbMutableFile` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBMutableFile)
    ///
    ///*This API requires the following crate features to be activated: `IdbMutableFile`*
    pub type IdbMutableFile;

    # [ wasm_bindgen ( structural , method , getter , js_class = "IDBMutableFile" , js_name = name ) ]
    ///Getter for the `name` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBMutableFile/name)
    ///
    ///*This API requires the following crate features to be activated: `IdbMutableFile`*
    pub fn name(this: &IdbMutableFile) -> String;

    # [ wasm_bindgen ( structural , method , getter , js_class = "IDBMutableFile" , js_name = type ) ]
    ///Getter for the `type` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBMutableFile/type)
    ///
    ///*This API requires the following crate features to be activated: `IdbMutableFile`*
    pub fn type_(this: &IdbMutableFile) -> String;

    #[cfg(feature = "IdbDatabase")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "IDBMutableFile" , js_name = database ) ]
    ///Getter for the `database` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBMutableFile/database)
    ///
    ///*This API requires the following crate features to be activated: `IdbDatabase`, `IdbMutableFile`*
    pub fn database(this: &IdbMutableFile) -> IdbDatabase;

    # [ wasm_bindgen ( structural , method , getter , js_class = "IDBMutableFile" , js_name = onabort ) ]
    ///Getter for the `onabort` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBMutableFile/onabort)
    ///
    ///*This API requires the following crate features to be activated: `IdbMutableFile`*
    pub fn onabort(this: &IdbMutableFile) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "IDBMutableFile" , js_name = onabort ) ]
    ///Setter for the `onabort` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBMutableFile/onabort)
    ///
    ///*This API requires the following crate features to be activated: `IdbMutableFile`*
    pub fn set_onabort(this: &IdbMutableFile, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "IDBMutableFile" , js_name = onerror ) ]
    ///Getter for the `onerror` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBMutableFile/onerror)
    ///
    ///*This API requires the following crate features to be activated: `IdbMutableFile`*
    pub fn onerror(this: &IdbMutableFile) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "IDBMutableFile" , js_name = onerror ) ]
    ///Setter for the `onerror` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBMutableFile/onerror)
    ///
    ///*This API requires the following crate features to be activated: `IdbMutableFile`*
    pub fn set_onerror(this: &IdbMutableFile, value: Option<&::js_sys::Function>);

    #[cfg(feature = "DomRequest")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "IDBMutableFile" , js_name = getFile ) ]
    ///The `getFile()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBMutableFile/getFile)
    ///
    ///*This API requires the following crate features to be activated: `DomRequest`, `IdbMutableFile`*
    pub fn get_file(this: &IdbMutableFile) -> Result<DomRequest, JsValue>;

    #[cfg(feature = "IdbFileHandle")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "IDBMutableFile" , js_name = open ) ]
    ///The `open()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBMutableFile/open)
    ///
    ///*This API requires the following crate features to be activated: `IdbFileHandle`, `IdbMutableFile`*
    pub fn open(this: &IdbMutableFile) -> Result<IdbFileHandle, JsValue>;

}
