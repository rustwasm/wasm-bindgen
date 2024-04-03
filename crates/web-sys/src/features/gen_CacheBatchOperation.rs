#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = CacheBatchOperation)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `CacheBatchOperation` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CacheBatchOperation`*"]
    pub type CacheBatchOperation;
    #[cfg(feature = "CacheQueryOptions")]
    #[wasm_bindgen(method, setter = "options")]
    fn options_shim(this: &CacheBatchOperation, val: &CacheQueryOptions);
    #[cfg(feature = "Request")]
    #[wasm_bindgen(method, setter = "request")]
    fn request_shim(this: &CacheBatchOperation, val: &Request);
    #[cfg(feature = "Response")]
    #[wasm_bindgen(method, setter = "response")]
    fn response_shim(this: &CacheBatchOperation, val: &Response);
    #[wasm_bindgen(method, setter = "type")]
    fn type__shim(this: &CacheBatchOperation, val: &str);
}
impl CacheBatchOperation {
    #[doc = "Construct a new `CacheBatchOperation`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CacheBatchOperation`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(feature = "CacheQueryOptions")]
    #[doc = "Change the `options` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CacheBatchOperation`, `CacheQueryOptions`*"]
    pub fn options(&mut self, val: &CacheQueryOptions) -> &mut Self {
        self.options_shim(val);
        self
    }
    #[cfg(feature = "Request")]
    #[doc = "Change the `request` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CacheBatchOperation`, `Request`*"]
    pub fn request(&mut self, val: &Request) -> &mut Self {
        self.request_shim(val);
        self
    }
    #[cfg(feature = "Response")]
    #[doc = "Change the `response` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CacheBatchOperation`, `Response`*"]
    pub fn response(&mut self, val: &Response) -> &mut Self {
        self.response_shim(val);
        self
    }
    #[doc = "Change the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CacheBatchOperation`*"]
    pub fn type_(&mut self, val: &str) -> &mut Self {
        self.type__shim(val);
        self
    }
}
impl Default for CacheBatchOperation {
    fn default() -> Self {
        Self::new()
    }
}
