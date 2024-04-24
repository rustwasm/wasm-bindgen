#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = GPUDepthStencilState)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GpuDepthStencilState` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuDepthStencilState`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type GpuDepthStencilState;
    #[wasm_bindgen(method, getter = "depthBias")]
    fn depth_bias_shim(this: &GpuDepthStencilState) -> i32;
    #[wasm_bindgen(method, setter = "depthBias")]
    fn set_depth_bias_shim(this: &GpuDepthStencilState, val: i32);
    #[wasm_bindgen(method, getter = "depthBiasClamp")]
    fn depth_bias_clamp_shim(this: &GpuDepthStencilState) -> f32;
    #[wasm_bindgen(method, setter = "depthBiasClamp")]
    fn set_depth_bias_clamp_shim(this: &GpuDepthStencilState, val: f32);
    #[wasm_bindgen(method, getter = "depthBiasSlopeScale")]
    fn depth_bias_slope_scale_shim(this: &GpuDepthStencilState) -> f32;
    #[wasm_bindgen(method, setter = "depthBiasSlopeScale")]
    fn set_depth_bias_slope_scale_shim(this: &GpuDepthStencilState, val: f32);
    #[cfg(feature = "GpuCompareFunction")]
    #[wasm_bindgen(method, getter = "depthCompare")]
    fn depth_compare_shim(this: &GpuDepthStencilState) -> GpuCompareFunction;
    #[cfg(feature = "GpuCompareFunction")]
    #[wasm_bindgen(method, setter = "depthCompare")]
    fn set_depth_compare_shim(this: &GpuDepthStencilState, val: GpuCompareFunction);
    #[wasm_bindgen(method, getter = "depthWriteEnabled")]
    fn depth_write_enabled_shim(this: &GpuDepthStencilState) -> bool;
    #[wasm_bindgen(method, setter = "depthWriteEnabled")]
    fn set_depth_write_enabled_shim(this: &GpuDepthStencilState, val: bool);
    #[cfg(feature = "GpuTextureFormat")]
    #[wasm_bindgen(method, getter = "format")]
    fn format_shim(this: &GpuDepthStencilState) -> GpuTextureFormat;
    #[cfg(feature = "GpuTextureFormat")]
    #[wasm_bindgen(method, setter = "format")]
    fn set_format_shim(this: &GpuDepthStencilState, val: GpuTextureFormat);
    #[cfg(feature = "GpuStencilFaceState")]
    #[wasm_bindgen(method, getter = "stencilBack")]
    fn stencil_back_shim(this: &GpuDepthStencilState) -> &GpuStencilFaceState;
    #[cfg(feature = "GpuStencilFaceState")]
    #[wasm_bindgen(method, setter = "stencilBack")]
    fn set_stencil_back_shim(this: &GpuDepthStencilState, val: &GpuStencilFaceState);
    #[cfg(feature = "GpuStencilFaceState")]
    #[wasm_bindgen(method, getter = "stencilFront")]
    fn stencil_front_shim(this: &GpuDepthStencilState) -> &GpuStencilFaceState;
    #[cfg(feature = "GpuStencilFaceState")]
    #[wasm_bindgen(method, setter = "stencilFront")]
    fn set_stencil_front_shim(this: &GpuDepthStencilState, val: &GpuStencilFaceState);
    #[wasm_bindgen(method, getter = "stencilReadMask")]
    fn stencil_read_mask_shim(this: &GpuDepthStencilState) -> u32;
    #[wasm_bindgen(method, setter = "stencilReadMask")]
    fn set_stencil_read_mask_shim(this: &GpuDepthStencilState, val: u32);
    #[wasm_bindgen(method, getter = "stencilWriteMask")]
    fn stencil_write_mask_shim(this: &GpuDepthStencilState) -> u32;
    #[wasm_bindgen(method, setter = "stencilWriteMask")]
    fn set_stencil_write_mask_shim(this: &GpuDepthStencilState, val: u32);
}
#[cfg(web_sys_unstable_apis)]
#[doc = "The trait to access properties on the `GpuDepthStencilState` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `GpuDepthStencilState`*"]
pub trait GpuDepthStencilStateGetters {
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `depthBias` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuDepthStencilState`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn depth_bias(&self) -> i32;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `depthBiasClamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuDepthStencilState`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn depth_bias_clamp(&self) -> f32;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `depthBiasSlopeScale` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuDepthStencilState`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn depth_bias_slope_scale(&self) -> f32;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuCompareFunction")]
    #[doc = "Get the `depthCompare` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuCompareFunction`, `GpuDepthStencilState`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn depth_compare(&self) -> GpuCompareFunction;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `depthWriteEnabled` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuDepthStencilState`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn depth_write_enabled(&self) -> bool;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuTextureFormat")]
    #[doc = "Get the `format` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuDepthStencilState`, `GpuTextureFormat`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn format(&self) -> GpuTextureFormat;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuStencilFaceState")]
    #[doc = "Get the `stencilBack` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuDepthStencilState`, `GpuStencilFaceState`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn stencil_back(&self) -> &GpuStencilFaceState;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuStencilFaceState")]
    #[doc = "Get the `stencilFront` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuDepthStencilState`, `GpuStencilFaceState`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn stencil_front(&self) -> &GpuStencilFaceState;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `stencilReadMask` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuDepthStencilState`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn stencil_read_mask(&self) -> u32;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `stencilWriteMask` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuDepthStencilState`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn stencil_write_mask(&self) -> u32;
}
#[cfg(web_sys_unstable_apis)]
impl GpuDepthStencilStateGetters for GpuDepthStencilState {
    #[cfg(web_sys_unstable_apis)]
    fn depth_bias(&self) -> i32 {
        self.depth_bias_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn depth_bias_clamp(&self) -> f32 {
        self.depth_bias_clamp_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn depth_bias_slope_scale(&self) -> f32 {
        self.depth_bias_slope_scale_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuCompareFunction")]
    fn depth_compare(&self) -> GpuCompareFunction {
        self.depth_compare_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn depth_write_enabled(&self) -> bool {
        self.depth_write_enabled_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuTextureFormat")]
    fn format(&self) -> GpuTextureFormat {
        self.format_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuStencilFaceState")]
    fn stencil_back(&self) -> &GpuStencilFaceState {
        self.stencil_back_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuStencilFaceState")]
    fn stencil_front(&self) -> &GpuStencilFaceState {
        self.stencil_front_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn stencil_read_mask(&self) -> u32 {
        self.stencil_read_mask_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn stencil_write_mask(&self) -> u32 {
        self.stencil_write_mask_shim()
    }
}
#[cfg(web_sys_unstable_apis)]
impl GpuDepthStencilState {
    #[cfg(feature = "GpuTextureFormat")]
    #[doc = "Construct a new `GpuDepthStencilState`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuDepthStencilState`, `GpuTextureFormat`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new(format: GpuTextureFormat) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.format(format);
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `depthBias` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuDepthStencilState`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn depth_bias(&mut self, val: i32) -> &mut Self {
        self.set_depth_bias_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `depthBiasClamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuDepthStencilState`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn depth_bias_clamp(&mut self, val: f32) -> &mut Self {
        self.set_depth_bias_clamp_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `depthBiasSlopeScale` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuDepthStencilState`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn depth_bias_slope_scale(&mut self, val: f32) -> &mut Self {
        self.set_depth_bias_slope_scale_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuCompareFunction")]
    #[doc = "Change the `depthCompare` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuCompareFunction`, `GpuDepthStencilState`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn depth_compare(&mut self, val: GpuCompareFunction) -> &mut Self {
        self.set_depth_compare_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `depthWriteEnabled` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuDepthStencilState`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn depth_write_enabled(&mut self, val: bool) -> &mut Self {
        self.set_depth_write_enabled_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuTextureFormat")]
    #[doc = "Change the `format` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuDepthStencilState`, `GpuTextureFormat`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn format(&mut self, val: GpuTextureFormat) -> &mut Self {
        self.set_format_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuStencilFaceState")]
    #[doc = "Change the `stencilBack` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuDepthStencilState`, `GpuStencilFaceState`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn stencil_back(&mut self, val: &GpuStencilFaceState) -> &mut Self {
        self.set_stencil_back_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuStencilFaceState")]
    #[doc = "Change the `stencilFront` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuDepthStencilState`, `GpuStencilFaceState`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn stencil_front(&mut self, val: &GpuStencilFaceState) -> &mut Self {
        self.set_stencil_front_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `stencilReadMask` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuDepthStencilState`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn stencil_read_mask(&mut self, val: u32) -> &mut Self {
        self.set_stencil_read_mask_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `stencilWriteMask` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuDepthStencilState`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn stencil_write_mask(&mut self, val: u32) -> &mut Self {
        self.set_stencil_write_mask_shim(val);
        self
    }
}
