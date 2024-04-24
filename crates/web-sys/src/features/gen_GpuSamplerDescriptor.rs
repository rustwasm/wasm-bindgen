#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = GPUSamplerDescriptor)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GpuSamplerDescriptor` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuSamplerDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type GpuSamplerDescriptor;
    #[wasm_bindgen(method, getter = "label")]
    fn label_shim(this: &GpuSamplerDescriptor) -> &str;
    #[wasm_bindgen(method, setter = "label")]
    fn set_label_shim(this: &GpuSamplerDescriptor, val: &str);
    #[cfg(feature = "GpuAddressMode")]
    #[wasm_bindgen(method, getter = "addressModeU")]
    fn address_mode_u_shim(this: &GpuSamplerDescriptor) -> GpuAddressMode;
    #[cfg(feature = "GpuAddressMode")]
    #[wasm_bindgen(method, setter = "addressModeU")]
    fn set_address_mode_u_shim(this: &GpuSamplerDescriptor, val: GpuAddressMode);
    #[cfg(feature = "GpuAddressMode")]
    #[wasm_bindgen(method, getter = "addressModeV")]
    fn address_mode_v_shim(this: &GpuSamplerDescriptor) -> GpuAddressMode;
    #[cfg(feature = "GpuAddressMode")]
    #[wasm_bindgen(method, setter = "addressModeV")]
    fn set_address_mode_v_shim(this: &GpuSamplerDescriptor, val: GpuAddressMode);
    #[cfg(feature = "GpuAddressMode")]
    #[wasm_bindgen(method, getter = "addressModeW")]
    fn address_mode_w_shim(this: &GpuSamplerDescriptor) -> GpuAddressMode;
    #[cfg(feature = "GpuAddressMode")]
    #[wasm_bindgen(method, setter = "addressModeW")]
    fn set_address_mode_w_shim(this: &GpuSamplerDescriptor, val: GpuAddressMode);
    #[cfg(feature = "GpuCompareFunction")]
    #[wasm_bindgen(method, getter = "compare")]
    fn compare_shim(this: &GpuSamplerDescriptor) -> GpuCompareFunction;
    #[cfg(feature = "GpuCompareFunction")]
    #[wasm_bindgen(method, setter = "compare")]
    fn set_compare_shim(this: &GpuSamplerDescriptor, val: GpuCompareFunction);
    #[wasm_bindgen(method, getter = "lodMaxClamp")]
    fn lod_max_clamp_shim(this: &GpuSamplerDescriptor) -> f32;
    #[wasm_bindgen(method, setter = "lodMaxClamp")]
    fn set_lod_max_clamp_shim(this: &GpuSamplerDescriptor, val: f32);
    #[wasm_bindgen(method, getter = "lodMinClamp")]
    fn lod_min_clamp_shim(this: &GpuSamplerDescriptor) -> f32;
    #[wasm_bindgen(method, setter = "lodMinClamp")]
    fn set_lod_min_clamp_shim(this: &GpuSamplerDescriptor, val: f32);
    #[cfg(feature = "GpuFilterMode")]
    #[wasm_bindgen(method, getter = "magFilter")]
    fn mag_filter_shim(this: &GpuSamplerDescriptor) -> GpuFilterMode;
    #[cfg(feature = "GpuFilterMode")]
    #[wasm_bindgen(method, setter = "magFilter")]
    fn set_mag_filter_shim(this: &GpuSamplerDescriptor, val: GpuFilterMode);
    #[wasm_bindgen(method, getter = "maxAnisotropy")]
    fn max_anisotropy_shim(this: &GpuSamplerDescriptor) -> u16;
    #[wasm_bindgen(method, setter = "maxAnisotropy")]
    fn set_max_anisotropy_shim(this: &GpuSamplerDescriptor, val: u16);
    #[cfg(feature = "GpuFilterMode")]
    #[wasm_bindgen(method, getter = "minFilter")]
    fn min_filter_shim(this: &GpuSamplerDescriptor) -> GpuFilterMode;
    #[cfg(feature = "GpuFilterMode")]
    #[wasm_bindgen(method, setter = "minFilter")]
    fn set_min_filter_shim(this: &GpuSamplerDescriptor, val: GpuFilterMode);
    #[cfg(feature = "GpuMipmapFilterMode")]
    #[wasm_bindgen(method, getter = "mipmapFilter")]
    fn mipmap_filter_shim(this: &GpuSamplerDescriptor) -> GpuMipmapFilterMode;
    #[cfg(feature = "GpuMipmapFilterMode")]
    #[wasm_bindgen(method, setter = "mipmapFilter")]
    fn set_mipmap_filter_shim(this: &GpuSamplerDescriptor, val: GpuMipmapFilterMode);
}
#[cfg(web_sys_unstable_apis)]
#[doc = "The trait to access properties on the `GpuSamplerDescriptor` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `GpuSamplerDescriptor`*"]
pub trait GpuSamplerDescriptorGetters {
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `label` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuSamplerDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn label(&self) -> &str;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuAddressMode")]
    #[doc = "Get the `addressModeU` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuAddressMode`, `GpuSamplerDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn address_mode_u(&self) -> GpuAddressMode;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuAddressMode")]
    #[doc = "Get the `addressModeV` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuAddressMode`, `GpuSamplerDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn address_mode_v(&self) -> GpuAddressMode;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuAddressMode")]
    #[doc = "Get the `addressModeW` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuAddressMode`, `GpuSamplerDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn address_mode_w(&self) -> GpuAddressMode;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuCompareFunction")]
    #[doc = "Get the `compare` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuCompareFunction`, `GpuSamplerDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn compare(&self) -> GpuCompareFunction;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `lodMaxClamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuSamplerDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn lod_max_clamp(&self) -> f32;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `lodMinClamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuSamplerDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn lod_min_clamp(&self) -> f32;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuFilterMode")]
    #[doc = "Get the `magFilter` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuFilterMode`, `GpuSamplerDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn mag_filter(&self) -> GpuFilterMode;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `maxAnisotropy` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuSamplerDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn max_anisotropy(&self) -> u16;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuFilterMode")]
    #[doc = "Get the `minFilter` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuFilterMode`, `GpuSamplerDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn min_filter(&self) -> GpuFilterMode;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuMipmapFilterMode")]
    #[doc = "Get the `mipmapFilter` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuMipmapFilterMode`, `GpuSamplerDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn mipmap_filter(&self) -> GpuMipmapFilterMode;
}
#[cfg(web_sys_unstable_apis)]
impl GpuSamplerDescriptorGetters for GpuSamplerDescriptor {
    #[cfg(web_sys_unstable_apis)]
    fn label(&self) -> &str {
        self.label_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuAddressMode")]
    fn address_mode_u(&self) -> GpuAddressMode {
        self.address_mode_u_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuAddressMode")]
    fn address_mode_v(&self) -> GpuAddressMode {
        self.address_mode_v_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuAddressMode")]
    fn address_mode_w(&self) -> GpuAddressMode {
        self.address_mode_w_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuCompareFunction")]
    fn compare(&self) -> GpuCompareFunction {
        self.compare_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn lod_max_clamp(&self) -> f32 {
        self.lod_max_clamp_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn lod_min_clamp(&self) -> f32 {
        self.lod_min_clamp_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuFilterMode")]
    fn mag_filter(&self) -> GpuFilterMode {
        self.mag_filter_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn max_anisotropy(&self) -> u16 {
        self.max_anisotropy_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuFilterMode")]
    fn min_filter(&self) -> GpuFilterMode {
        self.min_filter_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuMipmapFilterMode")]
    fn mipmap_filter(&self) -> GpuMipmapFilterMode {
        self.mipmap_filter_shim()
    }
}
#[cfg(web_sys_unstable_apis)]
impl GpuSamplerDescriptor {
    #[doc = "Construct a new `GpuSamplerDescriptor`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuSamplerDescriptor`*"]
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
    #[doc = "*This API requires the following crate features to be activated: `GpuSamplerDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn label(&mut self, val: &str) -> &mut Self {
        self.set_label_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuAddressMode")]
    #[doc = "Change the `addressModeU` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuAddressMode`, `GpuSamplerDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn address_mode_u(&mut self, val: GpuAddressMode) -> &mut Self {
        self.set_address_mode_u_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuAddressMode")]
    #[doc = "Change the `addressModeV` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuAddressMode`, `GpuSamplerDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn address_mode_v(&mut self, val: GpuAddressMode) -> &mut Self {
        self.set_address_mode_v_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuAddressMode")]
    #[doc = "Change the `addressModeW` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuAddressMode`, `GpuSamplerDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn address_mode_w(&mut self, val: GpuAddressMode) -> &mut Self {
        self.set_address_mode_w_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuCompareFunction")]
    #[doc = "Change the `compare` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuCompareFunction`, `GpuSamplerDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn compare(&mut self, val: GpuCompareFunction) -> &mut Self {
        self.set_compare_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `lodMaxClamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuSamplerDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn lod_max_clamp(&mut self, val: f32) -> &mut Self {
        self.set_lod_max_clamp_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `lodMinClamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuSamplerDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn lod_min_clamp(&mut self, val: f32) -> &mut Self {
        self.set_lod_min_clamp_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuFilterMode")]
    #[doc = "Change the `magFilter` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuFilterMode`, `GpuSamplerDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn mag_filter(&mut self, val: GpuFilterMode) -> &mut Self {
        self.set_mag_filter_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `maxAnisotropy` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuSamplerDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn max_anisotropy(&mut self, val: u16) -> &mut Self {
        self.set_max_anisotropy_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuFilterMode")]
    #[doc = "Change the `minFilter` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuFilterMode`, `GpuSamplerDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn min_filter(&mut self, val: GpuFilterMode) -> &mut Self {
        self.set_min_filter_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuMipmapFilterMode")]
    #[doc = "Change the `mipmapFilter` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuMipmapFilterMode`, `GpuSamplerDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn mipmap_filter(&mut self, val: GpuMipmapFilterMode) -> &mut Self {
        self.set_mipmap_filter_shim(val);
        self
    }
}
#[cfg(web_sys_unstable_apis)]
impl Default for GpuSamplerDescriptor {
    fn default() -> Self {
        Self::new()
    }
}
