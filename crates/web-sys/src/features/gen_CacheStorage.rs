#![allow(unused_imports)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = CacheStorage , typescript_type = "CacheStorage")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `CacheStorage` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CacheStorage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CacheStorage`*"]
    pub type CacheStorage;
    # [wasm_bindgen (method , structural , js_class = "CacheStorage" , js_name = delete)]
    #[doc = "The `delete()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CacheStorage/delete)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CacheStorage`*"]
    #[doc = ""]
    #[doc = "Return value: While the Promise can produce any JsValue as far as the type system is concerned, practically it is expected to contain a `bool`. It can be converted like `let result: bool = result.await.into();`."]
    pub fn delete(this: &CacheStorage, cache_name: &str) -> ::js_sys::Promise;
    # [wasm_bindgen (method , structural , js_class = "CacheStorage" , js_name = has)]
    #[doc = "The `has()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CacheStorage/has)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CacheStorage`*"]
    #[doc = ""]
    #[doc = "Return value: While the Promise can produce any JsValue as far as the type system is concerned, practically it is expected to contain a `bool`. It can be converted like `let result: bool = result.await.into();`."]
    pub fn has(this: &CacheStorage, cache_name: &str) -> ::js_sys::Promise;
    # [wasm_bindgen (method , structural , js_class = "CacheStorage" , js_name = keys)]
    #[doc = "The `keys()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CacheStorage/keys)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CacheStorage`*"]
    #[doc = ""]
    #[doc = "Return value: While the Promise can produce any JsValue as far as the type system is concerned, practically it is expected to contain a `:: js_sys :: Array`. It can be converted like `let result: :: js_sys :: Array = result.await.into();`. More information is available in the source IDL file."]
    pub fn keys(this: &CacheStorage) -> ::js_sys::Promise;
    #[cfg(feature = "Request")]
    # [wasm_bindgen (method , structural , js_class = "CacheStorage" , js_name = match)]
    #[doc = "The `match()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CacheStorage/match)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CacheStorage`, `Request`*"]
    #[doc = ""]
    #[doc = "Return value: While the Promise can produce any JsValue as far as the type system is concerned, practically it is expected to contain a `Response`. It can be converted like `let result: Response = result.await.into();`."]
    pub fn match_with_request(this: &CacheStorage, request: &Request) -> ::js_sys::Promise;
    # [wasm_bindgen (method , structural , js_class = "CacheStorage" , js_name = match)]
    #[doc = "The `match()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CacheStorage/match)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CacheStorage`*"]
    #[doc = ""]
    #[doc = "Return value: While the Promise can produce any JsValue as far as the type system is concerned, practically it is expected to contain a `Response`. It can be converted like `let result: Response = result.await.into();`."]
    pub fn match_with_str(this: &CacheStorage, request: &str) -> ::js_sys::Promise;
    #[cfg(all(feature = "CacheQueryOptions", feature = "Request",))]
    # [wasm_bindgen (method , structural , js_class = "CacheStorage" , js_name = match)]
    #[doc = "The `match()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CacheStorage/match)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CacheQueryOptions`, `CacheStorage`, `Request`*"]
    #[doc = ""]
    #[doc = "Return value: While the Promise can produce any JsValue as far as the type system is concerned, practically it is expected to contain a `Response`. It can be converted like `let result: Response = result.await.into();`."]
    pub fn match_with_request_and_options(
        this: &CacheStorage,
        request: &Request,
        options: &CacheQueryOptions,
    ) -> ::js_sys::Promise;
    #[cfg(feature = "CacheQueryOptions")]
    # [wasm_bindgen (method , structural , js_class = "CacheStorage" , js_name = match)]
    #[doc = "The `match()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CacheStorage/match)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CacheQueryOptions`, `CacheStorage`*"]
    #[doc = ""]
    #[doc = "Return value: While the Promise can produce any JsValue as far as the type system is concerned, practically it is expected to contain a `Response`. It can be converted like `let result: Response = result.await.into();`."]
    pub fn match_with_str_and_options(
        this: &CacheStorage,
        request: &str,
        options: &CacheQueryOptions,
    ) -> ::js_sys::Promise;
    # [wasm_bindgen (method , structural , js_class = "CacheStorage" , js_name = open)]
    #[doc = "The `open()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CacheStorage/open)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CacheStorage`*"]
    #[doc = ""]
    #[doc = "Return value: While the Promise can produce any JsValue as far as the type system is concerned, practically it is expected to contain a `Cache`. It can be converted like `let result: Cache = result.await.into();`."]
    pub fn open(this: &CacheStorage, cache_name: &str) -> ::js_sys::Promise;
}
