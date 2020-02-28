use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = FormData , typescript_name = FormData ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `FormData` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FormData)\n\n*This API requires the following crate features to be activated: `FormData`*"]
    pub type FormData;
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new FormData(..)` constructor, creating a new instance of `FormData`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FormData/FormData)\n\n*This API requires the following crate features to be activated: `FormData`*"]
    pub fn new(this: &FormData) -> Result<FormData, JsValue>;
    #[cfg(feature = "HtmlFormElement")]
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new FormData(..)` constructor, creating a new instance of `FormData`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FormData/FormData)\n\n*This API requires the following crate features to be activated: `FormData`, `HtmlFormElement`*"]
    pub fn new_with_form(this: &FormData, form: &HtmlFormElement) -> Result<FormData, JsValue>;
    #[cfg(feature = "Blob")]
    # [ wasm_bindgen ( catch , method , structural , js_name = append ) ]
    #[doc = "The `append()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FormData/append)\n\n*This API requires the following crate features to be activated: `Blob`, `FormData`*"]
    pub fn append_with_blob(this: &FormData, name: &str, value: &Blob) -> Result<(), JsValue>;
    #[cfg(feature = "Blob")]
    # [ wasm_bindgen ( catch , method , structural , js_name = append ) ]
    #[doc = "The `append()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FormData/append)\n\n*This API requires the following crate features to be activated: `Blob`, `FormData`*"]
    pub fn append_with_blob_and_filename(
        this: &FormData,
        name: &str,
        value: &Blob,
        filename: &str,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = append ) ]
    #[doc = "The `append()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FormData/append)\n\n*This API requires the following crate features to be activated: `FormData`*"]
    pub fn append_with_str(this: &FormData, name: &str, value: &str) -> Result<(), JsValue>;
    # [ wasm_bindgen ( method , structural , js_name = delete ) ]
    #[doc = "The `delete()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FormData/delete)\n\n*This API requires the following crate features to be activated: `FormData`*"]
    pub fn delete(this: &FormData, name: &str);
    # [ wasm_bindgen ( method , structural , js_name = get ) ]
    #[doc = "The `get()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FormData/get)\n\n*This API requires the following crate features to be activated: `FormData`*"]
    pub fn get(this: &FormData, name: &str) -> ::wasm_bindgen::JsValue;
    # [ wasm_bindgen ( method , structural , js_name = getAll ) ]
    #[doc = "The `getAll()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FormData/getAll)\n\n*This API requires the following crate features to be activated: `FormData`*"]
    pub fn get_all(this: &FormData, name: &str) -> ::js_sys::Array;
    # [ wasm_bindgen ( method , structural , js_name = has ) ]
    #[doc = "The `has()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FormData/has)\n\n*This API requires the following crate features to be activated: `FormData`*"]
    pub fn has(this: &FormData, name: &str) -> bool;
    #[cfg(feature = "Blob")]
    # [ wasm_bindgen ( catch , method , structural , js_name = set ) ]
    #[doc = "The `set()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FormData/set)\n\n*This API requires the following crate features to be activated: `Blob`, `FormData`*"]
    pub fn set_with_blob(this: &FormData, name: &str, value: &Blob) -> Result<(), JsValue>;
    #[cfg(feature = "Blob")]
    # [ wasm_bindgen ( catch , method , structural , js_name = set ) ]
    #[doc = "The `set()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FormData/set)\n\n*This API requires the following crate features to be activated: `Blob`, `FormData`*"]
    pub fn set_with_blob_and_filename(
        this: &FormData,
        name: &str,
        value: &Blob,
        filename: &str,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = set ) ]
    #[doc = "The `set()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FormData/set)\n\n*This API requires the following crate features to be activated: `FormData`*"]
    pub fn set_with_str(this: &FormData, name: &str, value: &str) -> Result<(), JsValue>;
}
