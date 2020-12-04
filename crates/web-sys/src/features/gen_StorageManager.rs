#![allow(unused_imports)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = StorageManager , typescript_type = "StorageManager")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `StorageManager` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StorageManager)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StorageManager`*"]
    pub type StorageManager;
    # [wasm_bindgen (catch , method , structural , js_class = "StorageManager" , js_name = estimate)]
    #[doc = "The `estimate()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StorageManager/estimate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StorageManager`*"]
    #[doc = ""]
    #[doc = "Return value: While the Promise of the successful result can produce any JsValue as far as the type system is concerned, practically it is expected to contain a <code>[StorageEstimate]</code>. It can be converted like `<code>let result: [StorageEstimate] = result?.await.into();</code>."]
    pub fn estimate(this: &StorageManager) -> Result<::js_sys::Promise, JsValue>;
    # [wasm_bindgen (catch , method , structural , js_class = "StorageManager" , js_name = persist)]
    #[doc = "The `persist()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StorageManager/persist)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StorageManager`*"]
    #[doc = ""]
    #[doc = "Return value: While the Promise of the successful result can produce any JsValue as far as the type system is concerned, practically it is expected to contain a <code>[bool]</code>. It can be converted like `<code>let result: [bool] = result?.await.into();</code>."]
    pub fn persist(this: &StorageManager) -> Result<::js_sys::Promise, JsValue>;
    # [wasm_bindgen (catch , method , structural , js_class = "StorageManager" , js_name = persisted)]
    #[doc = "The `persisted()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StorageManager/persisted)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StorageManager`*"]
    #[doc = ""]
    #[doc = "Return value: While the Promise of the successful result can produce any JsValue as far as the type system is concerned, practically it is expected to contain a <code>[bool]</code>. It can be converted like `<code>let result: [bool] = result?.await.into();</code>."]
    pub fn persisted(this: &StorageManager) -> Result<::js_sys::Promise, JsValue>;
}
