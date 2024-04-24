#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = VideoDecoderConfig)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `VideoDecoderConfig` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoDecoderConfig`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type VideoDecoderConfig;
    #[wasm_bindgen(method, getter = "codec")]
    fn codec_shim(this: &VideoDecoderConfig) -> &str;
    #[wasm_bindgen(method, setter = "codec")]
    fn set_codec_shim(this: &VideoDecoderConfig, val: &str);
    #[wasm_bindgen(method, getter = "codedHeight")]
    fn coded_height_shim(this: &VideoDecoderConfig) -> u32;
    #[wasm_bindgen(method, setter = "codedHeight")]
    fn set_coded_height_shim(this: &VideoDecoderConfig, val: u32);
    #[wasm_bindgen(method, getter = "codedWidth")]
    fn coded_width_shim(this: &VideoDecoderConfig) -> u32;
    #[wasm_bindgen(method, setter = "codedWidth")]
    fn set_coded_width_shim(this: &VideoDecoderConfig, val: u32);
    #[cfg(feature = "VideoColorSpaceInit")]
    #[wasm_bindgen(method, getter = "colorSpace")]
    fn color_space_shim(this: &VideoDecoderConfig) -> &VideoColorSpaceInit;
    #[cfg(feature = "VideoColorSpaceInit")]
    #[wasm_bindgen(method, setter = "colorSpace")]
    fn set_color_space_shim(this: &VideoDecoderConfig, val: &VideoColorSpaceInit);
    #[wasm_bindgen(method, getter = "description")]
    fn description_shim(this: &VideoDecoderConfig) -> &::js_sys::Object;
    #[wasm_bindgen(method, setter = "description")]
    fn set_description_shim(this: &VideoDecoderConfig, val: &::js_sys::Object);
    #[wasm_bindgen(method, getter = "displayAspectHeight")]
    fn display_aspect_height_shim(this: &VideoDecoderConfig) -> u32;
    #[wasm_bindgen(method, setter = "displayAspectHeight")]
    fn set_display_aspect_height_shim(this: &VideoDecoderConfig, val: u32);
    #[wasm_bindgen(method, getter = "displayAspectWidth")]
    fn display_aspect_width_shim(this: &VideoDecoderConfig) -> u32;
    #[wasm_bindgen(method, setter = "displayAspectWidth")]
    fn set_display_aspect_width_shim(this: &VideoDecoderConfig, val: u32);
    #[cfg(feature = "HardwareAcceleration")]
    #[wasm_bindgen(method, getter = "hardwareAcceleration")]
    fn hardware_acceleration_shim(this: &VideoDecoderConfig) -> HardwareAcceleration;
    #[cfg(feature = "HardwareAcceleration")]
    #[wasm_bindgen(method, setter = "hardwareAcceleration")]
    fn set_hardware_acceleration_shim(this: &VideoDecoderConfig, val: HardwareAcceleration);
    #[wasm_bindgen(method, getter = "optimizeForLatency")]
    fn optimize_for_latency_shim(this: &VideoDecoderConfig) -> bool;
    #[wasm_bindgen(method, setter = "optimizeForLatency")]
    fn set_optimize_for_latency_shim(this: &VideoDecoderConfig, val: bool);
}
#[cfg(web_sys_unstable_apis)]
#[doc = "The trait to access properties on the `VideoDecoderConfig` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `VideoDecoderConfig`*"]
pub trait VideoDecoderConfigGetters {
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `codec` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoDecoderConfig`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn codec(&self) -> &str;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `codedHeight` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoDecoderConfig`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn coded_height(&self) -> u32;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `codedWidth` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoDecoderConfig`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn coded_width(&self) -> u32;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "VideoColorSpaceInit")]
    #[doc = "Get the `colorSpace` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoColorSpaceInit`, `VideoDecoderConfig`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn color_space(&self) -> &VideoColorSpaceInit;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `description` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoDecoderConfig`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn description(&self) -> &::js_sys::Object;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `displayAspectHeight` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoDecoderConfig`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn display_aspect_height(&self) -> u32;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `displayAspectWidth` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoDecoderConfig`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn display_aspect_width(&self) -> u32;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "HardwareAcceleration")]
    #[doc = "Get the `hardwareAcceleration` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HardwareAcceleration`, `VideoDecoderConfig`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn hardware_acceleration(&self) -> HardwareAcceleration;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `optimizeForLatency` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoDecoderConfig`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn optimize_for_latency(&self) -> bool;
}
#[cfg(web_sys_unstable_apis)]
impl VideoDecoderConfigGetters for VideoDecoderConfig {
    #[cfg(web_sys_unstable_apis)]
    fn codec(&self) -> &str {
        self.codec_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn coded_height(&self) -> u32 {
        self.coded_height_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn coded_width(&self) -> u32 {
        self.coded_width_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "VideoColorSpaceInit")]
    fn color_space(&self) -> &VideoColorSpaceInit {
        self.color_space_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn description(&self) -> &::js_sys::Object {
        self.description_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn display_aspect_height(&self) -> u32 {
        self.display_aspect_height_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn display_aspect_width(&self) -> u32 {
        self.display_aspect_width_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "HardwareAcceleration")]
    fn hardware_acceleration(&self) -> HardwareAcceleration {
        self.hardware_acceleration_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn optimize_for_latency(&self) -> bool {
        self.optimize_for_latency_shim()
    }
}
#[cfg(web_sys_unstable_apis)]
impl VideoDecoderConfig {
    #[doc = "Construct a new `VideoDecoderConfig`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoDecoderConfig`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new(codec: &str) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.codec(codec);
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `codec` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoDecoderConfig`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn codec(&mut self, val: &str) -> &mut Self {
        self.set_codec_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `codedHeight` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoDecoderConfig`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn coded_height(&mut self, val: u32) -> &mut Self {
        self.set_coded_height_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `codedWidth` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoDecoderConfig`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn coded_width(&mut self, val: u32) -> &mut Self {
        self.set_coded_width_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "VideoColorSpaceInit")]
    #[doc = "Change the `colorSpace` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoColorSpaceInit`, `VideoDecoderConfig`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn color_space(&mut self, val: &VideoColorSpaceInit) -> &mut Self {
        self.set_color_space_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `description` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoDecoderConfig`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn description(&mut self, val: &::js_sys::Object) -> &mut Self {
        self.set_description_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `displayAspectHeight` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoDecoderConfig`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn display_aspect_height(&mut self, val: u32) -> &mut Self {
        self.set_display_aspect_height_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `displayAspectWidth` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoDecoderConfig`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn display_aspect_width(&mut self, val: u32) -> &mut Self {
        self.set_display_aspect_width_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "HardwareAcceleration")]
    #[doc = "Change the `hardwareAcceleration` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HardwareAcceleration`, `VideoDecoderConfig`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn hardware_acceleration(&mut self, val: HardwareAcceleration) -> &mut Self {
        self.set_hardware_acceleration_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `optimizeForLatency` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoDecoderConfig`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn optimize_for_latency(&mut self, val: bool) -> &mut Self {
        self.set_optimize_for_latency_shim(val);
        self
    }
}
