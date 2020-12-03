#![allow(unused_imports)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = Cache , typescript_type = "Cache")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `Cache` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Cache)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Cache`*"]
    pub type Cache;
    #[cfg(feature = "Request")]
    # [wasm_bindgen (method , structural , js_class = "Cache" , js_name = add)]
    #[doc = "The `add()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Cache/add)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Cache`, `Request`*"]
    #[doc = ""]
    #[doc = "Return value: There is additional information in the IDL file about the content of the promise, but it can not yet be explained any better."]
    pub fn add_with_request(this: &Cache, request: &Request) -> ::js_sys::Promise;
    # [wasm_bindgen (method , structural , js_class = "Cache" , js_name = add)]
    #[doc = "The `add()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Cache/add)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Cache`*"]
    #[doc = ""]
    #[doc = "Return value: There is additional information in the IDL file about the content of the promise, but it can not yet be explained any better."]
    pub fn add_with_str(this: &Cache, request: &str) -> ::js_sys::Promise;
    # [wasm_bindgen (method , structural , js_class = "Cache" , js_name = addAll)]
    #[doc = "The `addAll()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Cache/addAll)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Cache`*"]
    #[doc = ""]
    #[doc = "Argument `requests`: While the iterable or array can produce any JsValue as far as the type system is concerned, practically it is expected to contain a `& Request`."]
    #[doc = ""]
    #[doc = "Return value: There is additional information in the IDL file about the content of the promise, but it can not yet be explained any better."]
    pub fn add_all_with_request_sequence(
        this: &Cache,
        requests: &::wasm_bindgen::JsValue,
    ) -> ::js_sys::Promise;
    # [wasm_bindgen (method , structural , js_class = "Cache" , js_name = addAll)]
    #[doc = "The `addAll()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Cache/addAll)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Cache`*"]
    #[doc = ""]
    #[doc = "Argument `requests`: While the iterable or array can produce any JsValue as far as the type system is concerned, practically it is expected to contain a `& str`."]
    #[doc = ""]
    #[doc = "Return value: There is additional information in the IDL file about the content of the promise, but it can not yet be explained any better."]
    pub fn add_all_with_str_sequence(
        this: &Cache,
        requests: &::wasm_bindgen::JsValue,
    ) -> ::js_sys::Promise;
    #[cfg(feature = "Request")]
    # [wasm_bindgen (method , structural , js_class = "Cache" , js_name = delete)]
    #[doc = "The `delete()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Cache/delete)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Cache`, `Request`*"]
    #[doc = ""]
    #[doc = "Return value: While the Promise can produce any JsValue as far as the type system is concerned, practically it is expected to contain a `bool`. It can be converted like `let result: bool = result.await.into();`."]
    pub fn delete_with_request(this: &Cache, request: &Request) -> ::js_sys::Promise;
    # [wasm_bindgen (method , structural , js_class = "Cache" , js_name = delete)]
    #[doc = "The `delete()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Cache/delete)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Cache`*"]
    #[doc = ""]
    #[doc = "Return value: While the Promise can produce any JsValue as far as the type system is concerned, practically it is expected to contain a `bool`. It can be converted like `let result: bool = result.await.into();`."]
    pub fn delete_with_str(this: &Cache, request: &str) -> ::js_sys::Promise;
    #[cfg(all(feature = "CacheQueryOptions", feature = "Request",))]
    # [wasm_bindgen (method , structural , js_class = "Cache" , js_name = delete)]
    #[doc = "The `delete()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Cache/delete)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Cache`, `CacheQueryOptions`, `Request`*"]
    #[doc = ""]
    #[doc = "Return value: While the Promise can produce any JsValue as far as the type system is concerned, practically it is expected to contain a `bool`. It can be converted like `let result: bool = result.await.into();`."]
    pub fn delete_with_request_and_options(
        this: &Cache,
        request: &Request,
        options: &CacheQueryOptions,
    ) -> ::js_sys::Promise;
    #[cfg(feature = "CacheQueryOptions")]
    # [wasm_bindgen (method , structural , js_class = "Cache" , js_name = delete)]
    #[doc = "The `delete()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Cache/delete)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Cache`, `CacheQueryOptions`*"]
    #[doc = ""]
    #[doc = "Return value: While the Promise can produce any JsValue as far as the type system is concerned, practically it is expected to contain a `bool`. It can be converted like `let result: bool = result.await.into();`."]
    pub fn delete_with_str_and_options(
        this: &Cache,
        request: &str,
        options: &CacheQueryOptions,
    ) -> ::js_sys::Promise;
    # [wasm_bindgen (method , structural , js_class = "Cache" , js_name = keys)]
    #[doc = "The `keys()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Cache/keys)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Cache`*"]
    #[doc = ""]
    #[doc = "Return value: While the Promise can produce any JsValue as far as the type system is concerned, practically it is expected to contain a `:: js_sys :: Array`. It can be converted like `let result: :: js_sys :: Array = result.await.into();`. More information is available in the source IDL file."]
    pub fn keys(this: &Cache) -> ::js_sys::Promise;
    #[cfg(feature = "Request")]
    # [wasm_bindgen (method , structural , js_class = "Cache" , js_name = keys)]
    #[doc = "The `keys()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Cache/keys)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Cache`, `Request`*"]
    #[doc = ""]
    #[doc = "Return value: While the Promise can produce any JsValue as far as the type system is concerned, practically it is expected to contain a `:: js_sys :: Array`. It can be converted like `let result: :: js_sys :: Array = result.await.into();`. More information is available in the source IDL file."]
    pub fn keys_with_request(this: &Cache, request: &Request) -> ::js_sys::Promise;
    # [wasm_bindgen (method , structural , js_class = "Cache" , js_name = keys)]
    #[doc = "The `keys()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Cache/keys)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Cache`*"]
    #[doc = ""]
    #[doc = "Return value: While the Promise can produce any JsValue as far as the type system is concerned, practically it is expected to contain a `:: js_sys :: Array`. It can be converted like `let result: :: js_sys :: Array = result.await.into();`. More information is available in the source IDL file."]
    pub fn keys_with_str(this: &Cache, request: &str) -> ::js_sys::Promise;
    #[cfg(all(feature = "CacheQueryOptions", feature = "Request",))]
    # [wasm_bindgen (method , structural , js_class = "Cache" , js_name = keys)]
    #[doc = "The `keys()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Cache/keys)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Cache`, `CacheQueryOptions`, `Request`*"]
    #[doc = ""]
    #[doc = "Return value: While the Promise can produce any JsValue as far as the type system is concerned, practically it is expected to contain a `:: js_sys :: Array`. It can be converted like `let result: :: js_sys :: Array = result.await.into();`. More information is available in the source IDL file."]
    pub fn keys_with_request_and_options(
        this: &Cache,
        request: &Request,
        options: &CacheQueryOptions,
    ) -> ::js_sys::Promise;
    #[cfg(feature = "CacheQueryOptions")]
    # [wasm_bindgen (method , structural , js_class = "Cache" , js_name = keys)]
    #[doc = "The `keys()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Cache/keys)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Cache`, `CacheQueryOptions`*"]
    #[doc = ""]
    #[doc = "Return value: While the Promise can produce any JsValue as far as the type system is concerned, practically it is expected to contain a `:: js_sys :: Array`. It can be converted like `let result: :: js_sys :: Array = result.await.into();`. More information is available in the source IDL file."]
    pub fn keys_with_str_and_options(
        this: &Cache,
        request: &str,
        options: &CacheQueryOptions,
    ) -> ::js_sys::Promise;
    #[cfg(feature = "Request")]
    # [wasm_bindgen (method , structural , js_class = "Cache" , js_name = match)]
    #[doc = "The `match()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Cache/match)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Cache`, `Request`*"]
    #[doc = ""]
    #[doc = "Return value: While the Promise can produce any JsValue as far as the type system is concerned, practically it is expected to contain a `Response`. It can be converted like `let result: Response = result.await.into();`."]
    pub fn match_with_request(this: &Cache, request: &Request) -> ::js_sys::Promise;
    # [wasm_bindgen (method , structural , js_class = "Cache" , js_name = match)]
    #[doc = "The `match()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Cache/match)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Cache`*"]
    #[doc = ""]
    #[doc = "Return value: While the Promise can produce any JsValue as far as the type system is concerned, practically it is expected to contain a `Response`. It can be converted like `let result: Response = result.await.into();`."]
    pub fn match_with_str(this: &Cache, request: &str) -> ::js_sys::Promise;
    #[cfg(all(feature = "CacheQueryOptions", feature = "Request",))]
    # [wasm_bindgen (method , structural , js_class = "Cache" , js_name = match)]
    #[doc = "The `match()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Cache/match)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Cache`, `CacheQueryOptions`, `Request`*"]
    #[doc = ""]
    #[doc = "Return value: While the Promise can produce any JsValue as far as the type system is concerned, practically it is expected to contain a `Response`. It can be converted like `let result: Response = result.await.into();`."]
    pub fn match_with_request_and_options(
        this: &Cache,
        request: &Request,
        options: &CacheQueryOptions,
    ) -> ::js_sys::Promise;
    #[cfg(feature = "CacheQueryOptions")]
    # [wasm_bindgen (method , structural , js_class = "Cache" , js_name = match)]
    #[doc = "The `match()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Cache/match)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Cache`, `CacheQueryOptions`*"]
    #[doc = ""]
    #[doc = "Return value: While the Promise can produce any JsValue as far as the type system is concerned, practically it is expected to contain a `Response`. It can be converted like `let result: Response = result.await.into();`."]
    pub fn match_with_str_and_options(
        this: &Cache,
        request: &str,
        options: &CacheQueryOptions,
    ) -> ::js_sys::Promise;
    # [wasm_bindgen (method , structural , js_class = "Cache" , js_name = matchAll)]
    #[doc = "The `matchAll()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Cache/matchAll)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Cache`*"]
    #[doc = ""]
    #[doc = "Return value: While the Promise can produce any JsValue as far as the type system is concerned, practically it is expected to contain a `:: js_sys :: Array`. It can be converted like `let result: :: js_sys :: Array = result.await.into();`. More information is available in the source IDL file."]
    pub fn match_all(this: &Cache) -> ::js_sys::Promise;
    #[cfg(feature = "Request")]
    # [wasm_bindgen (method , structural , js_class = "Cache" , js_name = matchAll)]
    #[doc = "The `matchAll()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Cache/matchAll)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Cache`, `Request`*"]
    #[doc = ""]
    #[doc = "Return value: While the Promise can produce any JsValue as far as the type system is concerned, practically it is expected to contain a `:: js_sys :: Array`. It can be converted like `let result: :: js_sys :: Array = result.await.into();`. More information is available in the source IDL file."]
    pub fn match_all_with_request(this: &Cache, request: &Request) -> ::js_sys::Promise;
    # [wasm_bindgen (method , structural , js_class = "Cache" , js_name = matchAll)]
    #[doc = "The `matchAll()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Cache/matchAll)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Cache`*"]
    #[doc = ""]
    #[doc = "Return value: While the Promise can produce any JsValue as far as the type system is concerned, practically it is expected to contain a `:: js_sys :: Array`. It can be converted like `let result: :: js_sys :: Array = result.await.into();`. More information is available in the source IDL file."]
    pub fn match_all_with_str(this: &Cache, request: &str) -> ::js_sys::Promise;
    #[cfg(all(feature = "CacheQueryOptions", feature = "Request",))]
    # [wasm_bindgen (method , structural , js_class = "Cache" , js_name = matchAll)]
    #[doc = "The `matchAll()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Cache/matchAll)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Cache`, `CacheQueryOptions`, `Request`*"]
    #[doc = ""]
    #[doc = "Return value: While the Promise can produce any JsValue as far as the type system is concerned, practically it is expected to contain a `:: js_sys :: Array`. It can be converted like `let result: :: js_sys :: Array = result.await.into();`. More information is available in the source IDL file."]
    pub fn match_all_with_request_and_options(
        this: &Cache,
        request: &Request,
        options: &CacheQueryOptions,
    ) -> ::js_sys::Promise;
    #[cfg(feature = "CacheQueryOptions")]
    # [wasm_bindgen (method , structural , js_class = "Cache" , js_name = matchAll)]
    #[doc = "The `matchAll()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Cache/matchAll)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Cache`, `CacheQueryOptions`*"]
    #[doc = ""]
    #[doc = "Return value: While the Promise can produce any JsValue as far as the type system is concerned, practically it is expected to contain a `:: js_sys :: Array`. It can be converted like `let result: :: js_sys :: Array = result.await.into();`. More information is available in the source IDL file."]
    pub fn match_all_with_str_and_options(
        this: &Cache,
        request: &str,
        options: &CacheQueryOptions,
    ) -> ::js_sys::Promise;
    #[cfg(all(feature = "Request", feature = "Response",))]
    # [wasm_bindgen (method , structural , js_class = "Cache" , js_name = put)]
    #[doc = "The `put()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Cache/put)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Cache`, `Request`, `Response`*"]
    #[doc = ""]
    #[doc = "Return value: There is additional information in the IDL file about the content of the promise, but it can not yet be explained any better."]
    pub fn put_with_request(
        this: &Cache,
        request: &Request,
        response: &Response,
    ) -> ::js_sys::Promise;
    #[cfg(feature = "Response")]
    # [wasm_bindgen (method , structural , js_class = "Cache" , js_name = put)]
    #[doc = "The `put()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Cache/put)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Cache`, `Response`*"]
    #[doc = ""]
    #[doc = "Return value: There is additional information in the IDL file about the content of the promise, but it can not yet be explained any better."]
    pub fn put_with_str(this: &Cache, request: &str, response: &Response) -> ::js_sys::Promise;
}
