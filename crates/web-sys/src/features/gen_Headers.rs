use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = Headers , typescript_type = "Headers" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `Headers` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Headers)
    ///
    ///*This API requires the following crate features to be activated: `Headers`*
    pub type Headers;

    #[wasm_bindgen(catch, constructor, js_class = "Headers")]
    ///The `new Headers(..)` constructor, creating a new instance of `Headers`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Headers/Headers)
    ///
    ///*This API requires the following crate features to be activated: `Headers`*
    pub fn new() -> Result<Headers, JsValue>;

    #[wasm_bindgen(catch, constructor, js_class = "Headers")]
    ///The `new Headers(..)` constructor, creating a new instance of `Headers`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Headers/Headers)
    ///
    ///*This API requires the following crate features to be activated: `Headers`*
    pub fn new_with_headers(init: &Headers) -> Result<Headers, JsValue>;

    #[wasm_bindgen(catch, constructor, js_class = "Headers")]
    ///The `new Headers(..)` constructor, creating a new instance of `Headers`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Headers/Headers)
    ///
    ///*This API requires the following crate features to be activated: `Headers`*
    pub fn new_with_str_sequence_sequence(
        init: &::wasm_bindgen::JsValue,
    ) -> Result<Headers, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Headers" , js_name = append ) ]
    ///The `append()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Headers/append)
    ///
    ///*This API requires the following crate features to be activated: `Headers`*
    pub fn append(this: &Headers, name: &str, value: &str) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Headers" , js_name = delete ) ]
    ///The `delete()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Headers/delete)
    ///
    ///*This API requires the following crate features to be activated: `Headers`*
    pub fn delete(this: &Headers, name: &str) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Headers" , js_name = get ) ]
    ///The `get()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Headers/get)
    ///
    ///*This API requires the following crate features to be activated: `Headers`*
    pub fn get(this: &Headers, name: &str) -> Result<Option<String>, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Headers" , js_name = has ) ]
    ///The `has()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Headers/has)
    ///
    ///*This API requires the following crate features to be activated: `Headers`*
    pub fn has(this: &Headers, name: &str) -> Result<bool, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Headers" , js_name = set ) ]
    ///The `set()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Headers/set)
    ///
    ///*This API requires the following crate features to be activated: `Headers`*
    pub fn set(this: &Headers, name: &str, value: &str) -> Result<(), JsValue>;

}
