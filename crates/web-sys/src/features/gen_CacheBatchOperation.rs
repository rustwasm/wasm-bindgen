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
    #[wasm_bindgen(method, getter = "options")]
    fn options_shim(this: &CacheBatchOperation) -> CacheQueryOptions;
    #[cfg(feature = "CacheQueryOptions")]
    #[wasm_bindgen(method, setter = "options")]
    fn set_options_shim(this: &CacheBatchOperation, val: &CacheQueryOptions);
    #[cfg(feature = "Request")]
    #[wasm_bindgen(method, getter = "request")]
    fn request_shim(this: &CacheBatchOperation) -> Request;
    #[cfg(feature = "Request")]
    #[wasm_bindgen(method, setter = "request")]
    fn set_request_shim(this: &CacheBatchOperation, val: &Request);
    #[cfg(feature = "Response")]
    #[wasm_bindgen(method, getter = "response")]
    fn response_shim(this: &CacheBatchOperation) -> Response;
    #[cfg(feature = "Response")]
    #[wasm_bindgen(method, setter = "response")]
    fn set_response_shim(this: &CacheBatchOperation, val: &Response);
    #[wasm_bindgen(method, getter = "type")]
    fn type__shim(this: &CacheBatchOperation) -> String;
    #[wasm_bindgen(method, setter = "type")]
    fn set_type__shim(this: &CacheBatchOperation, val: &str);
}
#[doc = "The trait to access properties on the `CacheBatchOperation` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `CacheBatchOperation`*"]
pub trait CacheBatchOperationGetters {
    #[cfg(feature = "CacheQueryOptions")]
    #[doc = "Get the `options` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CacheBatchOperation`, `CacheQueryOptions`*"]
    fn options(&self) -> CacheQueryOptions;
    #[cfg(feature = "Request")]
    #[doc = "Get the `request` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CacheBatchOperation`, `Request`*"]
    fn request(&self) -> Request;
    #[cfg(feature = "Response")]
    #[doc = "Get the `response` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CacheBatchOperation`, `Response`*"]
    fn response(&self) -> Response;
    #[doc = "Get the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CacheBatchOperation`*"]
    fn type_(&self) -> String;
}
impl CacheBatchOperationGetters for CacheBatchOperation {
    #[cfg(feature = "CacheQueryOptions")]
    fn options(&self) -> CacheQueryOptions {
        self.options_shim()
    }
    #[cfg(feature = "Request")]
    fn request(&self) -> Request {
        self.request_shim()
    }
    #[cfg(feature = "Response")]
    fn response(&self) -> Response {
        self.response_shim()
    }
    fn type_(&self) -> String {
        self.type__shim()
    }
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
        self.set_options_shim(val);
        self
    }
    #[cfg(feature = "Request")]
    #[doc = "Change the `request` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CacheBatchOperation`, `Request`*"]
    pub fn request(&mut self, val: &Request) -> &mut Self {
        self.set_request_shim(val);
        self
    }
    #[cfg(feature = "Response")]
    #[doc = "Change the `response` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CacheBatchOperation`, `Response`*"]
    pub fn response(&mut self, val: &Response) -> &mut Self {
        self.set_response_shim(val);
        self
    }
    #[doc = "Change the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CacheBatchOperation`*"]
    pub fn type_(&mut self, val: &str) -> &mut Self {
        self.set_type__shim(val);
        self
    }
}
impl Default for CacheBatchOperation {
    fn default() -> Self {
        Self::new()
    }
}
