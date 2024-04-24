#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = GPUImageCopyBuffer)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GpuImageCopyBuffer` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuImageCopyBuffer`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type GpuImageCopyBuffer;
    #[wasm_bindgen(method, getter = "bytesPerRow")]
    fn bytes_per_row_shim(this: &GpuImageCopyBuffer) -> u32;
    #[wasm_bindgen(method, setter = "bytesPerRow")]
    fn set_bytes_per_row_shim(this: &GpuImageCopyBuffer, val: u32);
    #[wasm_bindgen(method, getter = "offset")]
    fn offset_shim(this: &GpuImageCopyBuffer) -> f64;
    #[wasm_bindgen(method, setter = "offset")]
    fn set_offset_shim(this: &GpuImageCopyBuffer, val: f64);
    #[wasm_bindgen(method, getter = "rowsPerImage")]
    fn rows_per_image_shim(this: &GpuImageCopyBuffer) -> u32;
    #[wasm_bindgen(method, setter = "rowsPerImage")]
    fn set_rows_per_image_shim(this: &GpuImageCopyBuffer, val: u32);
    #[cfg(feature = "GpuBuffer")]
    #[wasm_bindgen(method, getter = "buffer")]
    fn buffer_shim(this: &GpuImageCopyBuffer) -> &GpuBuffer;
    #[cfg(feature = "GpuBuffer")]
    #[wasm_bindgen(method, setter = "buffer")]
    fn set_buffer_shim(this: &GpuImageCopyBuffer, val: &GpuBuffer);
}
#[cfg(web_sys_unstable_apis)]
#[doc = "The trait to access properties on the `GpuImageCopyBuffer` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `GpuImageCopyBuffer`*"]
pub trait GpuImageCopyBufferGetters {
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `bytesPerRow` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuImageCopyBuffer`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn bytes_per_row(&self) -> u32;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `offset` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuImageCopyBuffer`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn offset(&self) -> f64;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `rowsPerImage` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuImageCopyBuffer`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn rows_per_image(&self) -> u32;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    #[doc = "Get the `buffer` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBuffer`, `GpuImageCopyBuffer`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn buffer(&self) -> &GpuBuffer;
}
#[cfg(web_sys_unstable_apis)]
impl GpuImageCopyBufferGetters for GpuImageCopyBuffer {
    #[cfg(web_sys_unstable_apis)]
    fn bytes_per_row(&self) -> u32 {
        self.bytes_per_row_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn offset(&self) -> f64 {
        self.offset_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn rows_per_image(&self) -> u32 {
        self.rows_per_image_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    fn buffer(&self) -> &GpuBuffer {
        self.buffer_shim()
    }
}
#[cfg(web_sys_unstable_apis)]
impl GpuImageCopyBuffer {
    #[cfg(feature = "GpuBuffer")]
    #[doc = "Construct a new `GpuImageCopyBuffer`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBuffer`, `GpuImageCopyBuffer`*"]
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
    #[doc = "Change the `bytesPerRow` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuImageCopyBuffer`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn bytes_per_row(&mut self, val: u32) -> &mut Self {
        self.set_bytes_per_row_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `offset` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuImageCopyBuffer`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn offset(&mut self, val: f64) -> &mut Self {
        self.set_offset_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `rowsPerImage` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuImageCopyBuffer`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn rows_per_image(&mut self, val: u32) -> &mut Self {
        self.set_rows_per_image_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    #[doc = "Change the `buffer` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBuffer`, `GpuImageCopyBuffer`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn buffer(&mut self, val: &GpuBuffer) -> &mut Self {
        self.set_buffer_shim(val);
        self
    }
}
