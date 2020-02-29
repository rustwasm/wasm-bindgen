use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = CacheStorage , typescript_name = CacheStorage ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `CacheStorage` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CacheStorage)
    ///
    ///*This API requires the following crate features to be activated: `CacheStorage`*
    pub type CacheStorage;

    # [ wasm_bindgen ( method , structural , js_class = "CacheStorage" , js_name = delete ) ]
    ///The `delete()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CacheStorage/delete)
    ///
    ///*This API requires the following crate features to be activated: `CacheStorage`*
    pub fn delete(this: &CacheStorage, cache_name: &str) -> ::js_sys::Promise;

    # [ wasm_bindgen ( method , structural , js_class = "CacheStorage" , js_name = has ) ]
    ///The `has()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CacheStorage/has)
    ///
    ///*This API requires the following crate features to be activated: `CacheStorage`*
    pub fn has(this: &CacheStorage, cache_name: &str) -> ::js_sys::Promise;

    # [ wasm_bindgen ( method , structural , js_class = "CacheStorage" , js_name = keys ) ]
    ///The `keys()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CacheStorage/keys)
    ///
    ///*This API requires the following crate features to be activated: `CacheStorage`*
    pub fn keys(this: &CacheStorage) -> ::js_sys::Promise;

    #[cfg(feature = "Request")]
    # [ wasm_bindgen ( method , structural , js_class = "CacheStorage" , js_name = match ) ]
    ///The `match()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CacheStorage/match)
    ///
    ///*This API requires the following crate features to be activated: `CacheStorage`, `Request`*
    pub fn match_with_request(this: &CacheStorage, request: &Request) -> ::js_sys::Promise;

    # [ wasm_bindgen ( method , structural , js_class = "CacheStorage" , js_name = match ) ]
    ///The `match()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CacheStorage/match)
    ///
    ///*This API requires the following crate features to be activated: `CacheStorage`*
    pub fn match_with_str(this: &CacheStorage, request: &str) -> ::js_sys::Promise;

    #[cfg(all(feature = "CacheQueryOptions", feature = "Request",))]
    # [ wasm_bindgen ( method , structural , js_class = "CacheStorage" , js_name = match ) ]
    ///The `match()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CacheStorage/match)
    ///
    ///*This API requires the following crate features to be activated: `CacheQueryOptions`, `CacheStorage`, `Request`*
    pub fn match_with_request_and_options(
        this: &CacheStorage,
        request: &Request,
        options: &CacheQueryOptions,
    ) -> ::js_sys::Promise;

    #[cfg(feature = "CacheQueryOptions")]
    # [ wasm_bindgen ( method , structural , js_class = "CacheStorage" , js_name = match ) ]
    ///The `match()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CacheStorage/match)
    ///
    ///*This API requires the following crate features to be activated: `CacheQueryOptions`, `CacheStorage`*
    pub fn match_with_str_and_options(
        this: &CacheStorage,
        request: &str,
        options: &CacheQueryOptions,
    ) -> ::js_sys::Promise;

    # [ wasm_bindgen ( method , structural , js_class = "CacheStorage" , js_name = open ) ]
    ///The `open()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CacheStorage/open)
    ///
    ///*This API requires the following crate features to be activated: `CacheStorage`*
    pub fn open(this: &CacheStorage, cache_name: &str) -> ::js_sys::Promise;

}
