#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = UnderlyingSource)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `UnderlyingSource` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UnderlyingSource`*"]
    pub type UnderlyingSource;
    #[wasm_bindgen(method, setter = "autoAllocateChunkSize")]
    fn auto_allocate_chunk_size_shim(this: &UnderlyingSource, val: f64);
    #[wasm_bindgen(method, setter = "cancel")]
    fn cancel_shim(this: &UnderlyingSource, val: &::js_sys::Function);
    #[wasm_bindgen(method, setter = "pull")]
    fn pull_shim(this: &UnderlyingSource, val: &::js_sys::Function);
    #[wasm_bindgen(method, setter = "start")]
    fn start_shim(this: &UnderlyingSource, val: &::js_sys::Function);
    #[cfg(feature = "ReadableStreamType")]
    #[wasm_bindgen(method, setter = "type")]
    fn type__shim(this: &UnderlyingSource, val: ReadableStreamType);
}
impl UnderlyingSource {
    #[doc = "Construct a new `UnderlyingSource`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UnderlyingSource`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `autoAllocateChunkSize` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UnderlyingSource`*"]
    pub fn auto_allocate_chunk_size(&mut self, val: f64) -> &mut Self {
        self.auto_allocate_chunk_size_shim(val);
        self
    }
    #[doc = "Change the `cancel` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UnderlyingSource`*"]
    pub fn cancel(&mut self, val: &::js_sys::Function) -> &mut Self {
        self.cancel_shim(val);
        self
    }
    #[doc = "Change the `pull` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UnderlyingSource`*"]
    pub fn pull(&mut self, val: &::js_sys::Function) -> &mut Self {
        self.pull_shim(val);
        self
    }
    #[doc = "Change the `start` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UnderlyingSource`*"]
    pub fn start(&mut self, val: &::js_sys::Function) -> &mut Self {
        self.start_shim(val);
        self
    }
    #[cfg(feature = "ReadableStreamType")]
    #[doc = "Change the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStreamType`, `UnderlyingSource`*"]
    pub fn type_(&mut self, val: ReadableStreamType) -> &mut Self {
        self.type__shim(val);
        self
    }
}
impl Default for UnderlyingSource {
    fn default() -> Self {
        Self::new()
    }
}
