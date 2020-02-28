use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = IDBFactory , typescript_name = IDBFactory ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `IdbFactory` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFactory)\n\n*This API requires the following crate features to be activated: `IdbFactory`*"]
    pub type IdbFactory;
    # [ wasm_bindgen ( catch , method , structural , js_class = "IDBFactory" , js_name = cmp ) ]
    #[doc = "The `cmp()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFactory/cmp)\n\n*This API requires the following crate features to be activated: `IdbFactory`*"]
    pub fn cmp(
        this: &IdbFactory,
        first: &::wasm_bindgen::JsValue,
        second: &::wasm_bindgen::JsValue,
    ) -> Result<i16, JsValue>;
    #[cfg(feature = "IdbOpenDbRequest")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "IDBFactory" , js_name = deleteDatabase ) ]
    #[doc = "The `deleteDatabase()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFactory/deleteDatabase)\n\n*This API requires the following crate features to be activated: `IdbFactory`, `IdbOpenDbRequest`*"]
    pub fn delete_database(this: &IdbFactory, name: &str) -> Result<IdbOpenDbRequest, JsValue>;
    #[cfg(all(feature = "IdbOpenDbOptions", feature = "IdbOpenDbRequest",))]
    # [ wasm_bindgen ( catch , method , structural , js_class = "IDBFactory" , js_name = deleteDatabase ) ]
    #[doc = "The `deleteDatabase()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFactory/deleteDatabase)\n\n*This API requires the following crate features to be activated: `IdbFactory`, `IdbOpenDbOptions`, `IdbOpenDbRequest`*"]
    pub fn delete_database_with_options(
        this: &IdbFactory,
        name: &str,
        options: &IdbOpenDbOptions,
    ) -> Result<IdbOpenDbRequest, JsValue>;
    #[cfg(feature = "IdbOpenDbRequest")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "IDBFactory" , js_name = open ) ]
    #[doc = "The `open()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFactory/open)\n\n*This API requires the following crate features to be activated: `IdbFactory`, `IdbOpenDbRequest`*"]
    pub fn open_with_u32(
        this: &IdbFactory,
        name: &str,
        version: u32,
    ) -> Result<IdbOpenDbRequest, JsValue>;
    #[cfg(feature = "IdbOpenDbRequest")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "IDBFactory" , js_name = open ) ]
    #[doc = "The `open()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFactory/open)\n\n*This API requires the following crate features to be activated: `IdbFactory`, `IdbOpenDbRequest`*"]
    pub fn open_with_f64(
        this: &IdbFactory,
        name: &str,
        version: f64,
    ) -> Result<IdbOpenDbRequest, JsValue>;
    #[cfg(feature = "IdbOpenDbRequest")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "IDBFactory" , js_name = open ) ]
    #[doc = "The `open()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFactory/open)\n\n*This API requires the following crate features to be activated: `IdbFactory`, `IdbOpenDbRequest`*"]
    pub fn open(this: &IdbFactory, name: &str) -> Result<IdbOpenDbRequest, JsValue>;
    #[cfg(all(feature = "IdbOpenDbOptions", feature = "IdbOpenDbRequest",))]
    # [ wasm_bindgen ( catch , method , structural , js_class = "IDBFactory" , js_name = open ) ]
    #[doc = "The `open()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFactory/open)\n\n*This API requires the following crate features to be activated: `IdbFactory`, `IdbOpenDbOptions`, `IdbOpenDbRequest`*"]
    pub fn open_with_idb_open_db_options(
        this: &IdbFactory,
        name: &str,
        options: &IdbOpenDbOptions,
    ) -> Result<IdbOpenDbRequest, JsValue>;
}
