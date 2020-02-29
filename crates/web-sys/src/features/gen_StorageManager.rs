use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = StorageManager , typescript_type = "StorageManager" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `StorageManager` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StorageManager)
    ///
    ///*This API requires the following crate features to be activated: `StorageManager`*
    pub type StorageManager;

    # [ wasm_bindgen ( catch , method , structural , js_class = "StorageManager" , js_name = estimate ) ]
    ///The `estimate()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StorageManager/estimate)
    ///
    ///*This API requires the following crate features to be activated: `StorageManager`*
    pub fn estimate(this: &StorageManager) -> Result<::js_sys::Promise, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "StorageManager" , js_name = persist ) ]
    ///The `persist()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StorageManager/persist)
    ///
    ///*This API requires the following crate features to be activated: `StorageManager`*
    pub fn persist(this: &StorageManager) -> Result<::js_sys::Promise, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "StorageManager" , js_name = persisted ) ]
    ///The `persisted()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StorageManager/persisted)
    ///
    ///*This API requires the following crate features to be activated: `StorageManager`*
    pub fn persisted(this: &StorageManager) -> Result<::js_sys::Promise, JsValue>;

}
