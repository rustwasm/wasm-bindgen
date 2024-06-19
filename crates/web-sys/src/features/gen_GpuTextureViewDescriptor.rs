#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = GPUTextureViewDescriptor)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GpuTextureViewDescriptor` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuTextureViewDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type GpuTextureViewDescriptor;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `label` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuTextureViewDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "label")]
    pub fn get_label(this: &GpuTextureViewDescriptor) -> Option<String>;
    #[wasm_bindgen(method, setter = "label")]
    fn set_label(this: &GpuTextureViewDescriptor, val: &str);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `arrayLayerCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuTextureViewDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "arrayLayerCount")]
    pub fn get_array_layer_count(this: &GpuTextureViewDescriptor) -> Option<u32>;
    #[wasm_bindgen(method, setter = "arrayLayerCount")]
    fn set_array_layer_count(this: &GpuTextureViewDescriptor, val: u32);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuTextureAspect")]
    #[doc = "Get the `aspect` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuTextureAspect`, `GpuTextureViewDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "aspect")]
    pub fn get_aspect(this: &GpuTextureViewDescriptor) -> Option<GpuTextureAspect>;
    #[cfg(feature = "GpuTextureAspect")]
    #[wasm_bindgen(method, setter = "aspect")]
    fn set_aspect(this: &GpuTextureViewDescriptor, val: GpuTextureAspect);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `baseArrayLayer` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuTextureViewDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "baseArrayLayer")]
    pub fn get_base_array_layer(this: &GpuTextureViewDescriptor) -> Option<u32>;
    #[wasm_bindgen(method, setter = "baseArrayLayer")]
    fn set_base_array_layer(this: &GpuTextureViewDescriptor, val: u32);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `baseMipLevel` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuTextureViewDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "baseMipLevel")]
    pub fn get_base_mip_level(this: &GpuTextureViewDescriptor) -> Option<u32>;
    #[wasm_bindgen(method, setter = "baseMipLevel")]
    fn set_base_mip_level(this: &GpuTextureViewDescriptor, val: u32);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuTextureViewDimension")]
    #[doc = "Get the `dimension` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuTextureViewDescriptor`, `GpuTextureViewDimension`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "dimension")]
    pub fn get_dimension(this: &GpuTextureViewDescriptor) -> Option<GpuTextureViewDimension>;
    #[cfg(feature = "GpuTextureViewDimension")]
    #[wasm_bindgen(method, setter = "dimension")]
    fn set_dimension(this: &GpuTextureViewDescriptor, val: GpuTextureViewDimension);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuTextureFormat")]
    #[doc = "Get the `format` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuTextureFormat`, `GpuTextureViewDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "format")]
    pub fn get_format(this: &GpuTextureViewDescriptor) -> Option<GpuTextureFormat>;
    #[cfg(feature = "GpuTextureFormat")]
    #[wasm_bindgen(method, setter = "format")]
    fn set_format(this: &GpuTextureViewDescriptor, val: GpuTextureFormat);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `mipLevelCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuTextureViewDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "mipLevelCount")]
    pub fn get_mip_level_count(this: &GpuTextureViewDescriptor) -> Option<u32>;
    #[wasm_bindgen(method, setter = "mipLevelCount")]
    fn set_mip_level_count(this: &GpuTextureViewDescriptor, val: u32);
}
#[cfg(web_sys_unstable_apis)]
impl GpuTextureViewDescriptor {
    #[doc = "Construct a new `GpuTextureViewDescriptor`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuTextureViewDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `label` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuTextureViewDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn label(&mut self, val: &str) -> &mut Self {
        self.set_label(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `arrayLayerCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuTextureViewDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn array_layer_count(&mut self, val: u32) -> &mut Self {
        self.set_array_layer_count(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuTextureAspect")]
    #[doc = "Change the `aspect` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuTextureAspect`, `GpuTextureViewDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn aspect(&mut self, val: GpuTextureAspect) -> &mut Self {
        self.set_aspect(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `baseArrayLayer` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuTextureViewDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn base_array_layer(&mut self, val: u32) -> &mut Self {
        self.set_base_array_layer(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `baseMipLevel` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuTextureViewDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn base_mip_level(&mut self, val: u32) -> &mut Self {
        self.set_base_mip_level(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuTextureViewDimension")]
    #[doc = "Change the `dimension` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuTextureViewDescriptor`, `GpuTextureViewDimension`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn dimension(&mut self, val: GpuTextureViewDimension) -> &mut Self {
        self.set_dimension(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuTextureFormat")]
    #[doc = "Change the `format` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuTextureFormat`, `GpuTextureViewDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn format(&mut self, val: GpuTextureFormat) -> &mut Self {
        self.set_format(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `mipLevelCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuTextureViewDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn mip_level_count(&mut self, val: u32) -> &mut Self {
        self.set_mip_level_count(val);
        self
    }
}
#[cfg(web_sys_unstable_apis)]
impl Default for GpuTextureViewDescriptor {
    fn default() -> Self {
        Self::new()
    }
}
