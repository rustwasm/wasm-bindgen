#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = CacheQueryOptions)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `CacheQueryOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CacheQueryOptions`*"]
    pub type CacheQueryOptions;
    #[wasm_bindgen(method, setter = "cacheName")]
    fn cache_name_shim(this: &CacheQueryOptions, val: &str);
    #[wasm_bindgen(method, setter = "ignoreMethod")]
    fn ignore_method_shim(this: &CacheQueryOptions, val: bool);
    #[wasm_bindgen(method, setter = "ignoreSearch")]
    fn ignore_search_shim(this: &CacheQueryOptions, val: bool);
    #[wasm_bindgen(method, setter = "ignoreVary")]
    fn ignore_vary_shim(this: &CacheQueryOptions, val: bool);
}
impl CacheQueryOptions {
    #[doc = "Construct a new `CacheQueryOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CacheQueryOptions`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `cacheName` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CacheQueryOptions`*"]
    pub fn cache_name(&mut self, val: &str) -> &mut Self {
        self.cache_name_shim(val);
        self
    }
    #[doc = "Change the `ignoreMethod` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CacheQueryOptions`*"]
    pub fn ignore_method(&mut self, val: bool) -> &mut Self {
        self.ignore_method_shim(val);
        self
    }
    #[doc = "Change the `ignoreSearch` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CacheQueryOptions`*"]
    pub fn ignore_search(&mut self, val: bool) -> &mut Self {
        self.ignore_search_shim(val);
        self
    }
    #[doc = "Change the `ignoreVary` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CacheQueryOptions`*"]
    pub fn ignore_vary(&mut self, val: bool) -> &mut Self {
        self.ignore_vary_shim(val);
        self
    }
}
impl Default for CacheQueryOptions {
    fn default() -> Self {
        Self::new()
    }
}
