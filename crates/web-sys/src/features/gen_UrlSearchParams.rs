use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = URLSearchParams , typescript_name = URLSearchParams ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `UrlSearchParams` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URLSearchParams)\n\n*This API requires the following crate features to be activated: `UrlSearchParams`*"]
    pub type UrlSearchParams;
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new UrlSearchParams(..)` constructor, creating a new instance of `UrlSearchParams`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URLSearchParams/URLSearchParams)\n\n*This API requires the following crate features to be activated: `UrlSearchParams`*"]
    pub fn new(this: &UrlSearchParams) -> Result<UrlSearchParams, JsValue>;
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new UrlSearchParams(..)` constructor, creating a new instance of `UrlSearchParams`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URLSearchParams/URLSearchParams)\n\n*This API requires the following crate features to be activated: `UrlSearchParams`*"]
    pub fn new_with_str_sequence_sequence(
        this: &UrlSearchParams,
        init: &::wasm_bindgen::JsValue,
    ) -> Result<UrlSearchParams, JsValue>;
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new UrlSearchParams(..)` constructor, creating a new instance of `UrlSearchParams`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URLSearchParams/URLSearchParams)\n\n*This API requires the following crate features to be activated: `UrlSearchParams`*"]
    pub fn new_with_str(this: &UrlSearchParams, init: &str) -> Result<UrlSearchParams, JsValue>;
    # [ wasm_bindgen ( method , structural , js_name = append ) ]
    #[doc = "The `append()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URLSearchParams/append)\n\n*This API requires the following crate features to be activated: `UrlSearchParams`*"]
    pub fn append(this: &UrlSearchParams, name: &str, value: &str);
    # [ wasm_bindgen ( method , structural , js_name = delete ) ]
    #[doc = "The `delete()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URLSearchParams/delete)\n\n*This API requires the following crate features to be activated: `UrlSearchParams`*"]
    pub fn delete(this: &UrlSearchParams, name: &str);
    # [ wasm_bindgen ( method , structural , js_name = get ) ]
    #[doc = "The `get()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URLSearchParams/get)\n\n*This API requires the following crate features to be activated: `UrlSearchParams`*"]
    pub fn get(this: &UrlSearchParams, name: &str) -> Option<String>;
    # [ wasm_bindgen ( method , structural , js_name = getAll ) ]
    #[doc = "The `getAll()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URLSearchParams/getAll)\n\n*This API requires the following crate features to be activated: `UrlSearchParams`*"]
    pub fn get_all(this: &UrlSearchParams, name: &str) -> ::js_sys::Array;
    # [ wasm_bindgen ( method , structural , js_name = has ) ]
    #[doc = "The `has()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URLSearchParams/has)\n\n*This API requires the following crate features to be activated: `UrlSearchParams`*"]
    pub fn has(this: &UrlSearchParams, name: &str) -> bool;
    # [ wasm_bindgen ( method , structural , js_name = set ) ]
    #[doc = "The `set()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URLSearchParams/set)\n\n*This API requires the following crate features to be activated: `UrlSearchParams`*"]
    pub fn set(this: &UrlSearchParams, name: &str, value: &str);
    # [ wasm_bindgen ( catch , method , structural , js_name = sort ) ]
    #[doc = "The `sort()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URLSearchParams/sort)\n\n*This API requires the following crate features to be activated: `UrlSearchParams`*"]
    pub fn sort(this: &UrlSearchParams) -> Result<(), JsValue>;
}
