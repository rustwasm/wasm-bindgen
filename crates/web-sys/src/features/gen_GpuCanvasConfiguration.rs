#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = GPUCanvasConfiguration)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GpuCanvasConfiguration` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuCanvasConfiguration`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type GpuCanvasConfiguration;
    #[cfg(feature = "GpuCanvasAlphaMode")]
    #[wasm_bindgen(method, getter = "alphaMode")]
    fn alpha_mode_shim(this: &GpuCanvasConfiguration) -> GpuCanvasAlphaMode;
    #[cfg(feature = "GpuCanvasAlphaMode")]
    #[wasm_bindgen(method, setter = "alphaMode")]
    fn set_alpha_mode_shim(this: &GpuCanvasConfiguration, val: GpuCanvasAlphaMode);
    #[cfg(feature = "GpuDevice")]
    #[wasm_bindgen(method, getter = "device")]
    fn device_shim(this: &GpuCanvasConfiguration) -> GpuDevice;
    #[cfg(feature = "GpuDevice")]
    #[wasm_bindgen(method, setter = "device")]
    fn set_device_shim(this: &GpuCanvasConfiguration, val: &GpuDevice);
    #[cfg(feature = "GpuTextureFormat")]
    #[wasm_bindgen(method, getter = "format")]
    fn format_shim(this: &GpuCanvasConfiguration) -> GpuTextureFormat;
    #[cfg(feature = "GpuTextureFormat")]
    #[wasm_bindgen(method, setter = "format")]
    fn set_format_shim(this: &GpuCanvasConfiguration, val: GpuTextureFormat);
    #[wasm_bindgen(method, getter = "usage")]
    fn usage_shim(this: &GpuCanvasConfiguration) -> u32;
    #[wasm_bindgen(method, setter = "usage")]
    fn set_usage_shim(this: &GpuCanvasConfiguration, val: u32);
    #[wasm_bindgen(method, getter = "viewFormats")]
    fn view_formats_shim(this: &GpuCanvasConfiguration) -> ::js_sys::Array;
    #[wasm_bindgen(method, setter = "viewFormats")]
    fn set_view_formats_shim(this: &GpuCanvasConfiguration, val: &::wasm_bindgen::JsValue);
}
#[cfg(web_sys_unstable_apis)]
#[doc = "The trait to access properties on the `GpuCanvasConfiguration` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `GpuCanvasConfiguration`*"]
pub trait GpuCanvasConfigurationGetters {
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuCanvasAlphaMode")]
    #[doc = "Get the `alphaMode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuCanvasAlphaMode`, `GpuCanvasConfiguration`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn alpha_mode(&self) -> GpuCanvasAlphaMode;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuDevice")]
    #[doc = "Get the `device` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuCanvasConfiguration`, `GpuDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn device(&self) -> GpuDevice;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuTextureFormat")]
    #[doc = "Get the `format` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuCanvasConfiguration`, `GpuTextureFormat`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn format(&self) -> GpuTextureFormat;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `usage` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuCanvasConfiguration`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn usage(&self) -> u32;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `viewFormats` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuCanvasConfiguration`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn view_formats(&self) -> ::js_sys::Array;
}
#[cfg(web_sys_unstable_apis)]
impl GpuCanvasConfigurationGetters for GpuCanvasConfiguration {
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuCanvasAlphaMode")]
    fn alpha_mode(&self) -> GpuCanvasAlphaMode {
        self.alpha_mode_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuDevice")]
    fn device(&self) -> GpuDevice {
        self.device_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuTextureFormat")]
    fn format(&self) -> GpuTextureFormat {
        self.format_shim()
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
impl GpuCanvasConfiguration {
    #[cfg(all(feature = "GpuDevice", feature = "GpuTextureFormat",))]
    #[doc = "Construct a new `GpuCanvasConfiguration`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuCanvasConfiguration`, `GpuDevice`, `GpuTextureFormat`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new(device: &GpuDevice, format: GpuTextureFormat) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        Self::device(&mut ret, device);
        Self::format(&mut ret, format);
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuCanvasAlphaMode")]
    #[doc = "Change the `alphaMode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuCanvasAlphaMode`, `GpuCanvasConfiguration`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn alpha_mode(&mut self, val: GpuCanvasAlphaMode) -> &mut Self {
        self.set_alpha_mode_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuDevice")]
    #[doc = "Change the `device` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuCanvasConfiguration`, `GpuDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn device(&mut self, val: &GpuDevice) -> &mut Self {
        self.set_device_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuTextureFormat")]
    #[doc = "Change the `format` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuCanvasConfiguration`, `GpuTextureFormat`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn format(&mut self, val: GpuTextureFormat) -> &mut Self {
        self.set_format_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `usage` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuCanvasConfiguration`*"]
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
    #[doc = "*This API requires the following crate features to be activated: `GpuCanvasConfiguration`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn view_formats(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_view_formats_shim(val);
        self
    }
}
