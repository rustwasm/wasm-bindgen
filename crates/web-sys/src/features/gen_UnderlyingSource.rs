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
    #[wasm_bindgen(method, getter = "autoAllocateChunkSize")]
    fn auto_allocate_chunk_size_shim(this: &UnderlyingSource) -> f64;
    #[wasm_bindgen(method, setter = "autoAllocateChunkSize")]
    fn set_auto_allocate_chunk_size_shim(this: &UnderlyingSource, val: f64);
    #[wasm_bindgen(method, getter = "cancel")]
    fn cancel_shim(this: &UnderlyingSource) -> &::js_sys::Function;
    #[wasm_bindgen(method, setter = "cancel")]
    fn set_cancel_shim(this: &UnderlyingSource, val: &::js_sys::Function);
    #[wasm_bindgen(method, getter = "pull")]
    fn pull_shim(this: &UnderlyingSource) -> &::js_sys::Function;
    #[wasm_bindgen(method, setter = "pull")]
    fn set_pull_shim(this: &UnderlyingSource, val: &::js_sys::Function);
    #[wasm_bindgen(method, getter = "start")]
    fn start_shim(this: &UnderlyingSource) -> &::js_sys::Function;
    #[wasm_bindgen(method, setter = "start")]
    fn set_start_shim(this: &UnderlyingSource, val: &::js_sys::Function);
    #[cfg(feature = "ReadableStreamType")]
    #[wasm_bindgen(method, getter = "type")]
    fn type__shim(this: &UnderlyingSource) -> ReadableStreamType;
    #[cfg(feature = "ReadableStreamType")]
    #[wasm_bindgen(method, setter = "type")]
    fn set_type__shim(this: &UnderlyingSource, val: ReadableStreamType);
}
#[doc = "The trait to access properties on the `UnderlyingSource` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `UnderlyingSource`*"]
pub trait UnderlyingSourceGetters {
    #[doc = "Get the `autoAllocateChunkSize` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UnderlyingSource`*"]
    fn auto_allocate_chunk_size(&self) -> f64;
    #[doc = "Get the `cancel` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UnderlyingSource`*"]
    fn cancel(&self) -> &::js_sys::Function;
    #[doc = "Get the `pull` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UnderlyingSource`*"]
    fn pull(&self) -> &::js_sys::Function;
    #[doc = "Get the `start` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UnderlyingSource`*"]
    fn start(&self) -> &::js_sys::Function;
    #[cfg(feature = "ReadableStreamType")]
    #[doc = "Get the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStreamType`, `UnderlyingSource`*"]
    fn type_(&self) -> ReadableStreamType;
}
impl UnderlyingSourceGetters for UnderlyingSource {
    fn auto_allocate_chunk_size(&self) -> f64 {
        self.auto_allocate_chunk_size_shim()
    }
    fn cancel(&self) -> &::js_sys::Function {
        self.cancel_shim()
    }
    fn pull(&self) -> &::js_sys::Function {
        self.pull_shim()
    }
    fn start(&self) -> &::js_sys::Function {
        self.start_shim()
    }
    #[cfg(feature = "ReadableStreamType")]
    fn type_(&self) -> ReadableStreamType {
        self.type__shim()
    }
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
        self.set_auto_allocate_chunk_size_shim(val);
        self
    }
    #[doc = "Change the `cancel` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UnderlyingSource`*"]
    pub fn cancel(&mut self, val: &::js_sys::Function) -> &mut Self {
        self.set_cancel_shim(val);
        self
    }
    #[doc = "Change the `pull` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UnderlyingSource`*"]
    pub fn pull(&mut self, val: &::js_sys::Function) -> &mut Self {
        self.set_pull_shim(val);
        self
    }
    #[doc = "Change the `start` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UnderlyingSource`*"]
    pub fn start(&mut self, val: &::js_sys::Function) -> &mut Self {
        self.set_start_shim(val);
        self
    }
    #[cfg(feature = "ReadableStreamType")]
    #[doc = "Change the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStreamType`, `UnderlyingSource`*"]
    pub fn type_(&mut self, val: ReadableStreamType) -> &mut Self {
        self.set_type__shim(val);
        self
    }
}
impl Default for UnderlyingSource {
    fn default() -> Self {
        Self::new()
    }
}
