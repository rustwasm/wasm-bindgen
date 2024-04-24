#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = GPUBufferBinding)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GpuBufferBinding` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBufferBinding`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type GpuBufferBinding;
    #[cfg(feature = "GpuBuffer")]
    #[wasm_bindgen(method, getter = "buffer")]
    fn buffer_shim(this: &GpuBufferBinding) -> GpuBuffer;
    #[cfg(feature = "GpuBuffer")]
    #[wasm_bindgen(method, setter = "buffer")]
    fn set_buffer_shim(this: &GpuBufferBinding, val: &GpuBuffer);
    #[wasm_bindgen(method, getter = "offset")]
    fn offset_shim(this: &GpuBufferBinding) -> f64;
    #[wasm_bindgen(method, setter = "offset")]
    fn set_offset_shim(this: &GpuBufferBinding, val: f64);
    #[wasm_bindgen(method, getter = "size")]
    fn size_shim(this: &GpuBufferBinding) -> f64;
    #[wasm_bindgen(method, setter = "size")]
    fn set_size_shim(this: &GpuBufferBinding, val: f64);
}
#[cfg(web_sys_unstable_apis)]
#[doc = "The trait to access properties on the `GpuBufferBinding` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `GpuBufferBinding`*"]
pub trait GpuBufferBindingGetters {
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    #[doc = "Get the `buffer` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBuffer`, `GpuBufferBinding`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn buffer(&self) -> GpuBuffer;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `offset` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBufferBinding`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn offset(&self) -> f64;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `size` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBufferBinding`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn size(&self) -> f64;
}
#[cfg(web_sys_unstable_apis)]
impl GpuBufferBindingGetters for GpuBufferBinding {
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    fn buffer(&self) -> GpuBuffer {
        self.buffer_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn offset(&self) -> f64 {
        self.offset_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn size(&self) -> f64 {
        self.size_shim()
    }
}
#[cfg(web_sys_unstable_apis)]
impl GpuBufferBinding {
    #[cfg(feature = "GpuBuffer")]
    #[doc = "Construct a new `GpuBufferBinding`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBuffer`, `GpuBufferBinding`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new(buffer: &GpuBuffer) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.buffer(buffer);
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    #[doc = "Change the `buffer` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBuffer`, `GpuBufferBinding`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn buffer(&mut self, val: &GpuBuffer) -> &mut Self {
        self.set_buffer_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `offset` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBufferBinding`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn offset(&mut self, val: f64) -> &mut Self {
        self.set_offset_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `size` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBufferBinding`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn size(&mut self, val: f64) -> &mut Self {
        self.set_size_shim(val);
        self
    }
}
