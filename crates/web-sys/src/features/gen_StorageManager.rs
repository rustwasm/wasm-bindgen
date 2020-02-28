use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = StorageManager , typescript_name = StorageManager ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `StorageManager` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StorageManager)\n\n*This API requires the following crate features to be activated: `StorageManager`*"]
    pub type StorageManager;
    # [ wasm_bindgen ( catch , method , structural , js_name = estimate ) ]
    #[doc = "The `estimate()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StorageManager/estimate)\n\n*This API requires the following crate features to be activated: `StorageManager`*"]
    pub fn estimate(this: &StorageManager) -> Result<::js_sys::Promise, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = persist ) ]
    #[doc = "The `persist()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StorageManager/persist)\n\n*This API requires the following crate features to be activated: `StorageManager`*"]
    pub fn persist(this: &StorageManager) -> Result<::js_sys::Promise, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = persisted ) ]
    #[doc = "The `persisted()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StorageManager/persisted)\n\n*This API requires the following crate features to be activated: `StorageManager`*"]
    pub fn persisted(this: &StorageManager) -> Result<::js_sys::Promise, JsValue>;
}
