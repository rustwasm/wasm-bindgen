#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = VideoEncoderConfig)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `VideoEncoderConfig` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoEncoderConfig`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type VideoEncoderConfig;
    #[cfg(feature = "AlphaOption")]
    #[wasm_bindgen(method, getter = "alpha")]
    fn alpha_shim(this: &VideoEncoderConfig) -> AlphaOption;
    #[cfg(feature = "AlphaOption")]
    #[wasm_bindgen(method, setter = "alpha")]
    fn set_alpha_shim(this: &VideoEncoderConfig, val: AlphaOption);
    #[wasm_bindgen(method, getter = "bitrate")]
    fn bitrate_shim(this: &VideoEncoderConfig) -> f64;
    #[wasm_bindgen(method, setter = "bitrate")]
    fn set_bitrate_shim(this: &VideoEncoderConfig, val: f64);
    #[wasm_bindgen(method, getter = "codec")]
    fn codec_shim(this: &VideoEncoderConfig) -> String;
    #[wasm_bindgen(method, setter = "codec")]
    fn set_codec_shim(this: &VideoEncoderConfig, val: &str);
    #[wasm_bindgen(method, getter = "displayHeight")]
    fn display_height_shim(this: &VideoEncoderConfig) -> u32;
    #[wasm_bindgen(method, setter = "displayHeight")]
    fn set_display_height_shim(this: &VideoEncoderConfig, val: u32);
    #[wasm_bindgen(method, getter = "displayWidth")]
    fn display_width_shim(this: &VideoEncoderConfig) -> u32;
    #[wasm_bindgen(method, setter = "displayWidth")]
    fn set_display_width_shim(this: &VideoEncoderConfig, val: u32);
    #[wasm_bindgen(method, getter = "framerate")]
    fn framerate_shim(this: &VideoEncoderConfig) -> f64;
    #[wasm_bindgen(method, setter = "framerate")]
    fn set_framerate_shim(this: &VideoEncoderConfig, val: f64);
    #[cfg(feature = "HardwareAcceleration")]
    #[wasm_bindgen(method, getter = "hardwareAcceleration")]
    fn hardware_acceleration_shim(this: &VideoEncoderConfig) -> HardwareAcceleration;
    #[cfg(feature = "HardwareAcceleration")]
    #[wasm_bindgen(method, setter = "hardwareAcceleration")]
    fn set_hardware_acceleration_shim(this: &VideoEncoderConfig, val: HardwareAcceleration);
    #[wasm_bindgen(method, getter = "height")]
    fn height_shim(this: &VideoEncoderConfig) -> u32;
    #[wasm_bindgen(method, setter = "height")]
    fn set_height_shim(this: &VideoEncoderConfig, val: u32);
    #[cfg(feature = "LatencyMode")]
    #[wasm_bindgen(method, getter = "latencyMode")]
    fn latency_mode_shim(this: &VideoEncoderConfig) -> LatencyMode;
    #[cfg(feature = "LatencyMode")]
    #[wasm_bindgen(method, setter = "latencyMode")]
    fn set_latency_mode_shim(this: &VideoEncoderConfig, val: LatencyMode);
    #[wasm_bindgen(method, getter = "scalabilityMode")]
    fn scalability_mode_shim(this: &VideoEncoderConfig) -> String;
    #[wasm_bindgen(method, setter = "scalabilityMode")]
    fn set_scalability_mode_shim(this: &VideoEncoderConfig, val: &str);
    #[wasm_bindgen(method, getter = "width")]
    fn width_shim(this: &VideoEncoderConfig) -> u32;
    #[wasm_bindgen(method, setter = "width")]
    fn set_width_shim(this: &VideoEncoderConfig, val: u32);
}
#[cfg(web_sys_unstable_apis)]
#[doc = "The trait to access properties on the `VideoEncoderConfig` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `VideoEncoderConfig`*"]
pub trait VideoEncoderConfigGetters {
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "AlphaOption")]
    #[doc = "Get the `alpha` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AlphaOption`, `VideoEncoderConfig`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn alpha(&self) -> AlphaOption;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `bitrate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoEncoderConfig`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn bitrate(&self) -> f64;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `codec` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoEncoderConfig`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn codec(&self) -> String;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `displayHeight` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoEncoderConfig`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn display_height(&self) -> u32;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `displayWidth` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoEncoderConfig`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn display_width(&self) -> u32;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `framerate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoEncoderConfig`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn framerate(&self) -> f64;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "HardwareAcceleration")]
    #[doc = "Get the `hardwareAcceleration` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HardwareAcceleration`, `VideoEncoderConfig`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn hardware_acceleration(&self) -> HardwareAcceleration;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `height` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoEncoderConfig`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn height(&self) -> u32;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "LatencyMode")]
    #[doc = "Get the `latencyMode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `LatencyMode`, `VideoEncoderConfig`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn latency_mode(&self) -> LatencyMode;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `scalabilityMode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoEncoderConfig`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn scalability_mode(&self) -> String;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `width` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoEncoderConfig`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn width(&self) -> u32;
}
#[cfg(web_sys_unstable_apis)]
impl VideoEncoderConfigGetters for VideoEncoderConfig {
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "AlphaOption")]
    fn alpha(&self) -> AlphaOption {
        self.alpha_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn bitrate(&self) -> f64 {
        self.bitrate_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn codec(&self) -> String {
        self.codec_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn display_height(&self) -> u32 {
        self.display_height_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn display_width(&self) -> u32 {
        self.display_width_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn framerate(&self) -> f64 {
        self.framerate_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "HardwareAcceleration")]
    fn hardware_acceleration(&self) -> HardwareAcceleration {
        self.hardware_acceleration_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn height(&self) -> u32 {
        self.height_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "LatencyMode")]
    fn latency_mode(&self) -> LatencyMode {
        self.latency_mode_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn scalability_mode(&self) -> String {
        self.scalability_mode_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn width(&self) -> u32 {
        self.width_shim()
    }
}
#[cfg(web_sys_unstable_apis)]
impl VideoEncoderConfig {
    #[doc = "Construct a new `VideoEncoderConfig`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoEncoderConfig`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new(codec: &str, height: u32, width: u32) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.codec(codec);
        ret.height(height);
        ret.width(width);
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "AlphaOption")]
    #[doc = "Change the `alpha` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AlphaOption`, `VideoEncoderConfig`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn alpha(&mut self, val: AlphaOption) -> &mut Self {
        self.set_alpha_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `bitrate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoEncoderConfig`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn bitrate(&mut self, val: f64) -> &mut Self {
        self.set_bitrate_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `codec` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoEncoderConfig`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn codec(&mut self, val: &str) -> &mut Self {
        self.set_codec_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `displayHeight` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoEncoderConfig`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn display_height(&mut self, val: u32) -> &mut Self {
        self.set_display_height_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `displayWidth` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoEncoderConfig`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn display_width(&mut self, val: u32) -> &mut Self {
        self.set_display_width_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `framerate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoEncoderConfig`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn framerate(&mut self, val: f64) -> &mut Self {
        self.set_framerate_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "HardwareAcceleration")]
    #[doc = "Change the `hardwareAcceleration` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HardwareAcceleration`, `VideoEncoderConfig`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn hardware_acceleration(&mut self, val: HardwareAcceleration) -> &mut Self {
        self.set_hardware_acceleration_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `height` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoEncoderConfig`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn height(&mut self, val: u32) -> &mut Self {
        self.set_height_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "LatencyMode")]
    #[doc = "Change the `latencyMode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `LatencyMode`, `VideoEncoderConfig`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn latency_mode(&mut self, val: LatencyMode) -> &mut Self {
        self.set_latency_mode_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `scalabilityMode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoEncoderConfig`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn scalability_mode(&mut self, val: &str) -> &mut Self {
        self.set_scalability_mode_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `width` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoEncoderConfig`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn width(&mut self, val: u32) -> &mut Self {
        self.set_width_shim(val);
        self
    }
}
