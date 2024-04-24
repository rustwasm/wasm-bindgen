#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = VideoFrameInit)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `VideoFrameInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoFrameInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type VideoFrameInit;
    #[cfg(feature = "AlphaOption")]
    #[wasm_bindgen(method, getter = "alpha")]
    fn alpha_shim(this: &VideoFrameInit) -> AlphaOption;
    #[cfg(feature = "AlphaOption")]
    #[wasm_bindgen(method, setter = "alpha")]
    fn set_alpha_shim(this: &VideoFrameInit, val: AlphaOption);
    #[wasm_bindgen(method, getter = "displayHeight")]
    fn display_height_shim(this: &VideoFrameInit) -> u32;
    #[wasm_bindgen(method, setter = "displayHeight")]
    fn set_display_height_shim(this: &VideoFrameInit, val: u32);
    #[wasm_bindgen(method, getter = "displayWidth")]
    fn display_width_shim(this: &VideoFrameInit) -> u32;
    #[wasm_bindgen(method, setter = "displayWidth")]
    fn set_display_width_shim(this: &VideoFrameInit, val: u32);
    #[wasm_bindgen(method, getter = "duration")]
    fn duration_shim(this: &VideoFrameInit) -> f64;
    #[wasm_bindgen(method, setter = "duration")]
    fn set_duration_shim(this: &VideoFrameInit, val: f64);
    #[wasm_bindgen(method, getter = "timestamp")]
    fn timestamp_shim(this: &VideoFrameInit) -> f64;
    #[wasm_bindgen(method, setter = "timestamp")]
    fn set_timestamp_shim(this: &VideoFrameInit, val: f64);
    #[cfg(feature = "DomRectInit")]
    #[wasm_bindgen(method, getter = "visibleRect")]
    fn visible_rect_shim(this: &VideoFrameInit) -> DomRectInit;
    #[cfg(feature = "DomRectInit")]
    #[wasm_bindgen(method, setter = "visibleRect")]
    fn set_visible_rect_shim(this: &VideoFrameInit, val: &DomRectInit);
}
#[cfg(web_sys_unstable_apis)]
#[doc = "The trait to access properties on the `VideoFrameInit` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `VideoFrameInit`*"]
pub trait VideoFrameInitGetters {
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "AlphaOption")]
    #[doc = "Get the `alpha` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AlphaOption`, `VideoFrameInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn alpha(&self) -> AlphaOption;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `displayHeight` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoFrameInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn display_height(&self) -> u32;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `displayWidth` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoFrameInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn display_width(&self) -> u32;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `duration` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoFrameInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn duration(&self) -> f64;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoFrameInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn timestamp(&self) -> f64;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "DomRectInit")]
    #[doc = "Get the `visibleRect` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomRectInit`, `VideoFrameInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn visible_rect(&self) -> DomRectInit;
}
#[cfg(web_sys_unstable_apis)]
impl VideoFrameInitGetters for VideoFrameInit {
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "AlphaOption")]
    fn alpha(&self) -> AlphaOption {
        self.alpha_shim()
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
    fn duration(&self) -> f64 {
        self.duration_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn timestamp(&self) -> f64 {
        self.timestamp_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "DomRectInit")]
    fn visible_rect(&self) -> DomRectInit {
        self.visible_rect_shim()
    }
}
#[cfg(web_sys_unstable_apis)]
impl VideoFrameInit {
    #[doc = "Construct a new `VideoFrameInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoFrameInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "AlphaOption")]
    #[doc = "Change the `alpha` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AlphaOption`, `VideoFrameInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn alpha(&mut self, val: AlphaOption) -> &mut Self {
        self.set_alpha_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `displayHeight` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoFrameInit`*"]
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
    #[doc = "*This API requires the following crate features to be activated: `VideoFrameInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn display_width(&mut self, val: u32) -> &mut Self {
        self.set_display_width_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `duration` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoFrameInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn duration(&mut self, val: f64) -> &mut Self {
        self.set_duration_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoFrameInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn timestamp(&mut self, val: f64) -> &mut Self {
        self.set_timestamp_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "DomRectInit")]
    #[doc = "Change the `visibleRect` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomRectInit`, `VideoFrameInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn visible_rect(&mut self, val: &DomRectInit) -> &mut Self {
        self.set_visible_rect_shim(val);
        self
    }
}
#[cfg(web_sys_unstable_apis)]
impl Default for VideoFrameInit {
    fn default() -> Self {
        Self::new()
    }
}
