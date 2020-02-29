use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = URLSearchParams , typescript_name = URLSearchParams ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `UrlSearchParams` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URLSearchParams)
    ///
    ///*This API requires the following crate features to be activated: `UrlSearchParams`*
    pub type UrlSearchParams;

    #[wasm_bindgen(catch, constructor, js_class = "URLSearchParams")]
    ///The `new UrlSearchParams(..)` constructor, creating a new instance of `UrlSearchParams`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URLSearchParams/URLSearchParams)
    ///
    ///*This API requires the following crate features to be activated: `UrlSearchParams`*
    pub fn new() -> Result<UrlSearchParams, JsValue>;

    #[wasm_bindgen(catch, constructor, js_class = "URLSearchParams")]
    ///The `new UrlSearchParams(..)` constructor, creating a new instance of `UrlSearchParams`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URLSearchParams/URLSearchParams)
    ///
    ///*This API requires the following crate features to be activated: `UrlSearchParams`*
    pub fn new_with_str_sequence_sequence(
        init: &::wasm_bindgen::JsValue,
    ) -> Result<UrlSearchParams, JsValue>;

    #[wasm_bindgen(catch, constructor, js_class = "URLSearchParams")]
    ///The `new UrlSearchParams(..)` constructor, creating a new instance of `UrlSearchParams`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URLSearchParams/URLSearchParams)
    ///
    ///*This API requires the following crate features to be activated: `UrlSearchParams`*
    pub fn new_with_str(init: &str) -> Result<UrlSearchParams, JsValue>;

    # [ wasm_bindgen ( method , structural , js_class = "URLSearchParams" , js_name = append ) ]
    ///The `append()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URLSearchParams/append)
    ///
    ///*This API requires the following crate features to be activated: `UrlSearchParams`*
    pub fn append(this: &UrlSearchParams, name: &str, value: &str);

    # [ wasm_bindgen ( method , structural , js_class = "URLSearchParams" , js_name = delete ) ]
    ///The `delete()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URLSearchParams/delete)
    ///
    ///*This API requires the following crate features to be activated: `UrlSearchParams`*
    pub fn delete(this: &UrlSearchParams, name: &str);

    # [ wasm_bindgen ( method , structural , js_class = "URLSearchParams" , js_name = get ) ]
    ///The `get()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URLSearchParams/get)
    ///
    ///*This API requires the following crate features to be activated: `UrlSearchParams`*
    pub fn get(this: &UrlSearchParams, name: &str) -> Option<String>;

    # [ wasm_bindgen ( method , structural , js_class = "URLSearchParams" , js_name = getAll ) ]
    ///The `getAll()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URLSearchParams/getAll)
    ///
    ///*This API requires the following crate features to be activated: `UrlSearchParams`*
    pub fn get_all(this: &UrlSearchParams, name: &str) -> ::js_sys::Array;

    # [ wasm_bindgen ( method , structural , js_class = "URLSearchParams" , js_name = has ) ]
    ///The `has()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URLSearchParams/has)
    ///
    ///*This API requires the following crate features to be activated: `UrlSearchParams`*
    pub fn has(this: &UrlSearchParams, name: &str) -> bool;

    # [ wasm_bindgen ( method , structural , js_class = "URLSearchParams" , js_name = set ) ]
    ///The `set()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URLSearchParams/set)
    ///
    ///*This API requires the following crate features to be activated: `UrlSearchParams`*
    pub fn set(this: &UrlSearchParams, name: &str, value: &str);

    # [ wasm_bindgen ( catch , method , structural , js_class = "URLSearchParams" , js_name = sort ) ]
    ///The `sort()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URLSearchParams/sort)
    ///
    ///*This API requires the following crate features to be activated: `UrlSearchParams`*
    pub fn sort(this: &UrlSearchParams) -> Result<(), JsValue>;

}
