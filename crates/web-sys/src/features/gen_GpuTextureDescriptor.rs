#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = GPUTextureDescriptor)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GpuTextureDescriptor` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuTextureDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type GpuTextureDescriptor;
    #[wasm_bindgen(method, getter = "label")]
    fn label_shim(this: &GpuTextureDescriptor) -> String;
    #[wasm_bindgen(method, setter = "label")]
    fn set_label_shim(this: &GpuTextureDescriptor, val: &str);
    #[cfg(feature = "GpuTextureDimension")]
    #[wasm_bindgen(method, getter = "dimension")]
    fn dimension_shim(this: &GpuTextureDescriptor) -> GpuTextureDimension;
    #[cfg(feature = "GpuTextureDimension")]
    #[wasm_bindgen(method, setter = "dimension")]
    fn set_dimension_shim(this: &GpuTextureDescriptor, val: GpuTextureDimension);
    #[cfg(feature = "GpuTextureFormat")]
    #[wasm_bindgen(method, getter = "format")]
    fn format_shim(this: &GpuTextureDescriptor) -> GpuTextureFormat;
    #[cfg(feature = "GpuTextureFormat")]
    #[wasm_bindgen(method, setter = "format")]
    fn set_format_shim(this: &GpuTextureDescriptor, val: GpuTextureFormat);
    #[wasm_bindgen(method, getter = "mipLevelCount")]
    fn mip_level_count_shim(this: &GpuTextureDescriptor) -> u32;
    #[wasm_bindgen(method, setter = "mipLevelCount")]
    fn set_mip_level_count_shim(this: &GpuTextureDescriptor, val: u32);
    #[wasm_bindgen(method, getter = "sampleCount")]
    fn sample_count_shim(this: &GpuTextureDescriptor) -> u32;
    #[wasm_bindgen(method, setter = "sampleCount")]
    fn set_sample_count_shim(this: &GpuTextureDescriptor, val: u32);
    #[wasm_bindgen(method, getter = "size")]
    fn size_shim(this: &GpuTextureDescriptor) -> ::wasm_bindgen::JsValue;
    #[wasm_bindgen(method, setter = "size")]
    fn set_size_shim(this: &GpuTextureDescriptor, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, getter = "usage")]
    fn usage_shim(this: &GpuTextureDescriptor) -> u32;
    #[wasm_bindgen(method, setter = "usage")]
    fn set_usage_shim(this: &GpuTextureDescriptor, val: u32);
    #[wasm_bindgen(method, getter = "viewFormats")]
    fn view_formats_shim(this: &GpuTextureDescriptor) -> ::js_sys::Array;
    #[wasm_bindgen(method, setter = "viewFormats")]
    fn set_view_formats_shim(this: &GpuTextureDescriptor, val: &::wasm_bindgen::JsValue);
}
#[cfg(web_sys_unstable_apis)]
#[doc = "The trait to access properties on the `GpuTextureDescriptor` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `GpuTextureDescriptor`*"]
pub trait GpuTextureDescriptorGetters {
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `label` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuTextureDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn label(&self) -> String;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuTextureDimension")]
    #[doc = "Get the `dimension` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuTextureDescriptor`, `GpuTextureDimension`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn dimension(&self) -> GpuTextureDimension;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuTextureFormat")]
    #[doc = "Get the `format` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuTextureDescriptor`, `GpuTextureFormat`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn format(&self) -> GpuTextureFormat;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `mipLevelCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuTextureDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn mip_level_count(&self) -> u32;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `sampleCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuTextureDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn sample_count(&self) -> u32;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `size` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuTextureDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn size(&self) -> ::wasm_bindgen::JsValue;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `usage` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuTextureDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn usage(&self) -> u32;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `viewFormats` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuTextureDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn view_formats(&self) -> ::js_sys::Array;
}
#[cfg(web_sys_unstable_apis)]
impl GpuTextureDescriptorGetters for GpuTextureDescriptor {
    #[cfg(web_sys_unstable_apis)]
    fn label(&self) -> String {
        self.label_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuTextureDimension")]
    fn dimension(&self) -> GpuTextureDimension {
        self.dimension_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuTextureFormat")]
    fn format(&self) -> GpuTextureFormat {
        self.format_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn mip_level_count(&self) -> u32 {
        self.mip_level_count_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn sample_count(&self) -> u32 {
        self.sample_count_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn size(&self) -> ::wasm_bindgen::JsValue {
        self.size_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn usage(&self) -> u32 {
        self.usage_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn view_formats(&self) -> ::js_sys::Array {
        self.view_formats_shim()
    }
}
#[cfg(web_sys_unstable_apis)]
impl GpuTextureDescriptor {
    #[cfg(feature = "GpuTextureFormat")]
    #[doc = "Construct a new `GpuTextureDescriptor`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuTextureDescriptor`, `GpuTextureFormat`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new(format: GpuTextureFormat, size: &::wasm_bindgen::JsValue, usage: u32) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        Self::format(&mut ret, format);
        Self::size(&mut ret, size);
        Self::usage(&mut ret, usage);
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `label` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuTextureDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn label(&mut self, val: &str) -> &mut Self {
        self.set_label_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuTextureDimension")]
    #[doc = "Change the `dimension` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuTextureDescriptor`, `GpuTextureDimension`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn dimension(&mut self, val: GpuTextureDimension) -> &mut Self {
        self.set_dimension_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuTextureFormat")]
    #[doc = "Change the `format` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuTextureDescriptor`, `GpuTextureFormat`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn format(&mut self, val: GpuTextureFormat) -> &mut Self {
        self.set_format_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `mipLevelCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuTextureDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn mip_level_count(&mut self, val: u32) -> &mut Self {
        self.set_mip_level_count_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `sampleCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuTextureDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn sample_count(&mut self, val: u32) -> &mut Self {
        self.set_sample_count_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `size` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuTextureDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn size(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_size_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `usage` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuTextureDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn usage(&mut self, val: u32) -> &mut Self {
        self.set_usage_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `viewFormats` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuTextureDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn view_formats(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_view_formats_shim(val);
        self
    }
}
