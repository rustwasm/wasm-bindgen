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
    #[wasm_bindgen(method, getter = "cacheName")]
    fn cache_name_shim(this: &CacheQueryOptions) -> String;
    #[wasm_bindgen(method, setter = "cacheName")]
    fn set_cache_name_shim(this: &CacheQueryOptions, val: &str);
    #[wasm_bindgen(method, getter = "ignoreMethod")]
    fn ignore_method_shim(this: &CacheQueryOptions) -> bool;
    #[wasm_bindgen(method, setter = "ignoreMethod")]
    fn set_ignore_method_shim(this: &CacheQueryOptions, val: bool);
    #[wasm_bindgen(method, getter = "ignoreSearch")]
    fn ignore_search_shim(this: &CacheQueryOptions) -> bool;
    #[wasm_bindgen(method, setter = "ignoreSearch")]
    fn set_ignore_search_shim(this: &CacheQueryOptions, val: bool);
    #[wasm_bindgen(method, getter = "ignoreVary")]
    fn ignore_vary_shim(this: &CacheQueryOptions) -> bool;
    #[wasm_bindgen(method, setter = "ignoreVary")]
    fn set_ignore_vary_shim(this: &CacheQueryOptions, val: bool);
}
#[doc = "The trait to access properties on the `CacheQueryOptions` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `CacheQueryOptions`*"]
pub trait CacheQueryOptionsGetters {
    #[doc = "Get the `cacheName` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CacheQueryOptions`*"]
    fn cache_name(&self) -> String;
    #[doc = "Get the `ignoreMethod` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CacheQueryOptions`*"]
    fn ignore_method(&self) -> bool;
    #[doc = "Get the `ignoreSearch` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CacheQueryOptions`*"]
    fn ignore_search(&self) -> bool;
    #[doc = "Get the `ignoreVary` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CacheQueryOptions`*"]
    fn ignore_vary(&self) -> bool;
}
impl CacheQueryOptionsGetters for CacheQueryOptions {
    fn cache_name(&self) -> String {
        self.cache_name_shim()
    }
    fn ignore_method(&self) -> bool {
        self.ignore_method_shim()
    }
    fn ignore_search(&self) -> bool {
        self.ignore_search_shim()
    }
    fn ignore_vary(&self) -> bool {
        self.ignore_vary_shim()
    }
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
        self.set_cache_name_shim(val);
        self
    }
    #[doc = "Change the `ignoreMethod` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CacheQueryOptions`*"]
    pub fn ignore_method(&mut self, val: bool) -> &mut Self {
        self.set_ignore_method_shim(val);
        self
    }
    #[doc = "Change the `ignoreSearch` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CacheQueryOptions`*"]
    pub fn ignore_search(&mut self, val: bool) -> &mut Self {
        self.set_ignore_search_shim(val);
        self
    }
    #[doc = "Change the `ignoreVary` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CacheQueryOptions`*"]
    pub fn ignore_vary(&mut self, val: bool) -> &mut Self {
        self.set_ignore_vary_shim(val);
        self
    }
}
impl Default for CacheQueryOptions {
    fn default() -> Self {
        Self::new()
    }
}
