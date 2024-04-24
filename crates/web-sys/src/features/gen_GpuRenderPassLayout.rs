#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = GPURenderPassLayout)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GpuRenderPassLayout` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuRenderPassLayout`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type GpuRenderPassLayout;
    #[wasm_bindgen(method, getter = "label")]
    fn label_shim(this: &GpuRenderPassLayout) -> String;
    #[wasm_bindgen(method, setter = "label")]
    fn set_label_shim(this: &GpuRenderPassLayout, val: &str);
    #[wasm_bindgen(method, getter = "colorFormats")]
    fn color_formats_shim(this: &GpuRenderPassLayout) -> ::js_sys::Array;
    #[wasm_bindgen(method, setter = "colorFormats")]
    fn set_color_formats_shim(this: &GpuRenderPassLayout, val: &::wasm_bindgen::JsValue);
    #[cfg(feature = "GpuTextureFormat")]
    #[wasm_bindgen(method, getter = "depthStencilFormat")]
    fn depth_stencil_format_shim(this: &GpuRenderPassLayout) -> GpuTextureFormat;
    #[cfg(feature = "GpuTextureFormat")]
    #[wasm_bindgen(method, setter = "depthStencilFormat")]
    fn set_depth_stencil_format_shim(this: &GpuRenderPassLayout, val: GpuTextureFormat);
    #[wasm_bindgen(method, getter = "sampleCount")]
    fn sample_count_shim(this: &GpuRenderPassLayout) -> u32;
    #[wasm_bindgen(method, setter = "sampleCount")]
    fn set_sample_count_shim(this: &GpuRenderPassLayout, val: u32);
}
#[cfg(web_sys_unstable_apis)]
#[doc = "The trait to access properties on the `GpuRenderPassLayout` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `GpuRenderPassLayout`*"]
pub trait GpuRenderPassLayoutGetters {
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `label` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuRenderPassLayout`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn label(&self) -> String;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `colorFormats` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuRenderPassLayout`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn color_formats(&self) -> ::js_sys::Array;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuTextureFormat")]
    #[doc = "Get the `depthStencilFormat` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuRenderPassLayout`, `GpuTextureFormat`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn depth_stencil_format(&self) -> GpuTextureFormat;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `sampleCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuRenderPassLayout`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn sample_count(&self) -> u32;
}
#[cfg(web_sys_unstable_apis)]
impl GpuRenderPassLayoutGetters for GpuRenderPassLayout {
    #[cfg(web_sys_unstable_apis)]
    fn label(&self) -> String {
        self.label_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn color_formats(&self) -> ::js_sys::Array {
        self.color_formats_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuTextureFormat")]
    fn depth_stencil_format(&self) -> GpuTextureFormat {
        self.depth_stencil_format_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn sample_count(&self) -> u32 {
        self.sample_count_shim()
    }
}
#[cfg(web_sys_unstable_apis)]
impl GpuRenderPassLayout {
    #[doc = "Construct a new `GpuRenderPassLayout`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuRenderPassLayout`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new(color_formats: &::wasm_bindgen::JsValue) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        Self::color_formats(&mut ret, color_formats);
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `label` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuRenderPassLayout`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn label(&mut self, val: &str) -> &mut Self {
        self.set_label_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `colorFormats` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuRenderPassLayout`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn color_formats(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_color_formats_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuTextureFormat")]
    #[doc = "Change the `depthStencilFormat` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuRenderPassLayout`, `GpuTextureFormat`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn depth_stencil_format(&mut self, val: GpuTextureFormat) -> &mut Self {
        self.set_depth_stencil_format_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `sampleCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuRenderPassLayout`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn sample_count(&mut self, val: u32) -> &mut Self {
        self.set_sample_count_shim(val);
        self
    }
}
