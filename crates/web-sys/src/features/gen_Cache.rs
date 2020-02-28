use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = Cache , typescript_name = Cache ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `Cache` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Cache)\n\n*This API requires the following crate features to be activated: `Cache`*"]
    pub type Cache;
    #[cfg(feature = "Request")]
    # [ wasm_bindgen ( method , structural , js_class = "Cache" , js_name = add ) ]
    #[doc = "The `add()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Cache/add)\n\n*This API requires the following crate features to be activated: `Cache`, `Request`*"]
    pub fn add_with_request(this: &Cache, request: &Request) -> ::js_sys::Promise;
    # [ wasm_bindgen ( method , structural , js_class = "Cache" , js_name = add ) ]
    #[doc = "The `add()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Cache/add)\n\n*This API requires the following crate features to be activated: `Cache`*"]
    pub fn add_with_str(this: &Cache, request: &str) -> ::js_sys::Promise;
    # [ wasm_bindgen ( method , structural , js_class = "Cache" , js_name = addAll ) ]
    #[doc = "The `addAll()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Cache/addAll)\n\n*This API requires the following crate features to be activated: `Cache`*"]
    pub fn add_all_with_request_sequence(
        this: &Cache,
        requests: &::wasm_bindgen::JsValue,
    ) -> ::js_sys::Promise;
    # [ wasm_bindgen ( method , structural , js_class = "Cache" , js_name = addAll ) ]
    #[doc = "The `addAll()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Cache/addAll)\n\n*This API requires the following crate features to be activated: `Cache`*"]
    pub fn add_all_with_str_sequence(
        this: &Cache,
        requests: &::wasm_bindgen::JsValue,
    ) -> ::js_sys::Promise;
    #[cfg(feature = "Request")]
    # [ wasm_bindgen ( method , structural , js_class = "Cache" , js_name = delete ) ]
    #[doc = "The `delete()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Cache/delete)\n\n*This API requires the following crate features to be activated: `Cache`, `Request`*"]
    pub fn delete_with_request(this: &Cache, request: &Request) -> ::js_sys::Promise;
    # [ wasm_bindgen ( method , structural , js_class = "Cache" , js_name = delete ) ]
    #[doc = "The `delete()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Cache/delete)\n\n*This API requires the following crate features to be activated: `Cache`*"]
    pub fn delete_with_str(this: &Cache, request: &str) -> ::js_sys::Promise;
    #[cfg(all(feature = "CacheQueryOptions", feature = "Request",))]
    # [ wasm_bindgen ( method , structural , js_class = "Cache" , js_name = delete ) ]
    #[doc = "The `delete()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Cache/delete)\n\n*This API requires the following crate features to be activated: `Cache`, `CacheQueryOptions`, `Request`*"]
    pub fn delete_with_request_and_options(
        this: &Cache,
        request: &Request,
        options: &CacheQueryOptions,
    ) -> ::js_sys::Promise;
    #[cfg(feature = "CacheQueryOptions")]
    # [ wasm_bindgen ( method , structural , js_class = "Cache" , js_name = delete ) ]
    #[doc = "The `delete()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Cache/delete)\n\n*This API requires the following crate features to be activated: `Cache`, `CacheQueryOptions`*"]
    pub fn delete_with_str_and_options(
        this: &Cache,
        request: &str,
        options: &CacheQueryOptions,
    ) -> ::js_sys::Promise;
    # [ wasm_bindgen ( method , structural , js_class = "Cache" , js_name = keys ) ]
    #[doc = "The `keys()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Cache/keys)\n\n*This API requires the following crate features to be activated: `Cache`*"]
    pub fn keys(this: &Cache) -> ::js_sys::Promise;
    #[cfg(feature = "Request")]
    # [ wasm_bindgen ( method , structural , js_class = "Cache" , js_name = keys ) ]
    #[doc = "The `keys()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Cache/keys)\n\n*This API requires the following crate features to be activated: `Cache`, `Request`*"]
    pub fn keys_with_request(this: &Cache, request: &Request) -> ::js_sys::Promise;
    # [ wasm_bindgen ( method , structural , js_class = "Cache" , js_name = keys ) ]
    #[doc = "The `keys()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Cache/keys)\n\n*This API requires the following crate features to be activated: `Cache`*"]
    pub fn keys_with_str(this: &Cache, request: &str) -> ::js_sys::Promise;
    #[cfg(all(feature = "CacheQueryOptions", feature = "Request",))]
    # [ wasm_bindgen ( method , structural , js_class = "Cache" , js_name = keys ) ]
    #[doc = "The `keys()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Cache/keys)\n\n*This API requires the following crate features to be activated: `Cache`, `CacheQueryOptions`, `Request`*"]
    pub fn keys_with_request_and_options(
        this: &Cache,
        request: &Request,
        options: &CacheQueryOptions,
    ) -> ::js_sys::Promise;
    #[cfg(feature = "CacheQueryOptions")]
    # [ wasm_bindgen ( method , structural , js_class = "Cache" , js_name = keys ) ]
    #[doc = "The `keys()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Cache/keys)\n\n*This API requires the following crate features to be activated: `Cache`, `CacheQueryOptions`*"]
    pub fn keys_with_str_and_options(
        this: &Cache,
        request: &str,
        options: &CacheQueryOptions,
    ) -> ::js_sys::Promise;
    #[cfg(feature = "Request")]
    # [ wasm_bindgen ( method , structural , js_class = "Cache" , js_name = match ) ]
    #[doc = "The `match()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Cache/match)\n\n*This API requires the following crate features to be activated: `Cache`, `Request`*"]
    pub fn match_with_request(this: &Cache, request: &Request) -> ::js_sys::Promise;
    # [ wasm_bindgen ( method , structural , js_class = "Cache" , js_name = match ) ]
    #[doc = "The `match()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Cache/match)\n\n*This API requires the following crate features to be activated: `Cache`*"]
    pub fn match_with_str(this: &Cache, request: &str) -> ::js_sys::Promise;
    #[cfg(all(feature = "CacheQueryOptions", feature = "Request",))]
    # [ wasm_bindgen ( method , structural , js_class = "Cache" , js_name = match ) ]
    #[doc = "The `match()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Cache/match)\n\n*This API requires the following crate features to be activated: `Cache`, `CacheQueryOptions`, `Request`*"]
    pub fn match_with_request_and_options(
        this: &Cache,
        request: &Request,
        options: &CacheQueryOptions,
    ) -> ::js_sys::Promise;
    #[cfg(feature = "CacheQueryOptions")]
    # [ wasm_bindgen ( method , structural , js_class = "Cache" , js_name = match ) ]
    #[doc = "The `match()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Cache/match)\n\n*This API requires the following crate features to be activated: `Cache`, `CacheQueryOptions`*"]
    pub fn match_with_str_and_options(
        this: &Cache,
        request: &str,
        options: &CacheQueryOptions,
    ) -> ::js_sys::Promise;
    # [ wasm_bindgen ( method , structural , js_class = "Cache" , js_name = matchAll ) ]
    #[doc = "The `matchAll()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Cache/matchAll)\n\n*This API requires the following crate features to be activated: `Cache`*"]
    pub fn match_all(this: &Cache) -> ::js_sys::Promise;
    #[cfg(feature = "Request")]
    # [ wasm_bindgen ( method , structural , js_class = "Cache" , js_name = matchAll ) ]
    #[doc = "The `matchAll()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Cache/matchAll)\n\n*This API requires the following crate features to be activated: `Cache`, `Request`*"]
    pub fn match_all_with_request(this: &Cache, request: &Request) -> ::js_sys::Promise;
    # [ wasm_bindgen ( method , structural , js_class = "Cache" , js_name = matchAll ) ]
    #[doc = "The `matchAll()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Cache/matchAll)\n\n*This API requires the following crate features to be activated: `Cache`*"]
    pub fn match_all_with_str(this: &Cache, request: &str) -> ::js_sys::Promise;
    #[cfg(all(feature = "CacheQueryOptions", feature = "Request",))]
    # [ wasm_bindgen ( method , structural , js_class = "Cache" , js_name = matchAll ) ]
    #[doc = "The `matchAll()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Cache/matchAll)\n\n*This API requires the following crate features to be activated: `Cache`, `CacheQueryOptions`, `Request`*"]
    pub fn match_all_with_request_and_options(
        this: &Cache,
        request: &Request,
        options: &CacheQueryOptions,
    ) -> ::js_sys::Promise;
    #[cfg(feature = "CacheQueryOptions")]
    # [ wasm_bindgen ( method , structural , js_class = "Cache" , js_name = matchAll ) ]
    #[doc = "The `matchAll()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Cache/matchAll)\n\n*This API requires the following crate features to be activated: `Cache`, `CacheQueryOptions`*"]
    pub fn match_all_with_str_and_options(
        this: &Cache,
        request: &str,
        options: &CacheQueryOptions,
    ) -> ::js_sys::Promise;
    #[cfg(all(feature = "Request", feature = "Response",))]
    # [ wasm_bindgen ( method , structural , js_class = "Cache" , js_name = put ) ]
    #[doc = "The `put()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Cache/put)\n\n*This API requires the following crate features to be activated: `Cache`, `Request`, `Response`*"]
    pub fn put_with_request(
        this: &Cache,
        request: &Request,
        response: &Response,
    ) -> ::js_sys::Promise;
    #[cfg(feature = "Response")]
    # [ wasm_bindgen ( method , structural , js_class = "Cache" , js_name = put ) ]
    #[doc = "The `put()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Cache/put)\n\n*This API requires the following crate features to be activated: `Cache`, `Response`*"]
    pub fn put_with_str(this: &Cache, request: &str, response: &Response) -> ::js_sys::Promise;
}
