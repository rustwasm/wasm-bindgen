use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = Storage , typescript_name = Storage ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `Storage` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Storage)\n\n*This API requires the following crate features to be activated: `Storage`*"]
    pub type Storage;
    # [ wasm_bindgen ( structural , catch , method , getter , js_name = length ) ]
    #[doc = "Getter for the `length` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Storage/length)\n\n*This API requires the following crate features to be activated: `Storage`*"]
    pub fn length(this: &Storage) -> Result<u32, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = clear ) ]
    #[doc = "The `clear()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Storage/clear)\n\n*This API requires the following crate features to be activated: `Storage`*"]
    pub fn clear(this: &Storage) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = getItem ) ]
    #[doc = "The `getItem()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Storage/getItem)\n\n*This API requires the following crate features to be activated: `Storage`*"]
    pub fn get_item(this: &Storage, key: &str) -> Result<Option<String>, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = key ) ]
    #[doc = "The `key()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Storage/key)\n\n*This API requires the following crate features to be activated: `Storage`*"]
    pub fn key(this: &Storage, index: u32) -> Result<Option<String>, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = removeItem ) ]
    #[doc = "The `removeItem()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Storage/removeItem)\n\n*This API requires the following crate features to be activated: `Storage`*"]
    pub fn remove_item(this: &Storage, key: &str) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = setItem ) ]
    #[doc = "The `setItem()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Storage/setItem)\n\n*This API requires the following crate features to be activated: `Storage`*"]
    pub fn set_item(this: &Storage, key: &str, value: &str) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, structural, indexing_getter)]
    #[doc = "Indexing getter.\n\n\n\n*This API requires the following crate features to be activated: `Storage`*"]
    pub fn get(this: &Storage, key: &str) -> Result<Option<String>, JsValue>;
    #[wasm_bindgen(catch, method, structural, indexing_setter)]
    #[doc = "Indexing setter.\n\n\n\n*This API requires the following crate features to be activated: `Storage`*"]
    pub fn set(this: &Storage, key: &str, value: &str) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, structural, indexing_deleter)]
    #[doc = "Indexing deleter.\n\n\n\n*This API requires the following crate features to be activated: `Storage`*"]
    pub fn delete(this: &Storage, key: &str) -> Result<(), JsValue>;
}
