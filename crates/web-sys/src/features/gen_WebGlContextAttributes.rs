#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = WebGLContextAttributes)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `WebGlContextAttributes` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlContextAttributes`*"]
    pub type WebGlContextAttributes;
    #[wasm_bindgen(method, setter = "alpha")]
    fn alpha_shim(this: &WebGlContextAttributes, val: bool);
    #[wasm_bindgen(method, setter = "antialias")]
    fn antialias_shim(this: &WebGlContextAttributes, val: bool);
    #[wasm_bindgen(method, setter = "depth")]
    fn depth_shim(this: &WebGlContextAttributes, val: bool);
    #[wasm_bindgen(method, setter = "failIfMajorPerformanceCaveat")]
    fn fail_if_major_performance_caveat_shim(this: &WebGlContextAttributes, val: bool);
    #[cfg(feature = "WebGlPowerPreference")]
    #[wasm_bindgen(method, setter = "powerPreference")]
    fn power_preference_shim(this: &WebGlContextAttributes, val: WebGlPowerPreference);
    #[wasm_bindgen(method, setter = "premultipliedAlpha")]
    fn premultiplied_alpha_shim(this: &WebGlContextAttributes, val: bool);
    #[wasm_bindgen(method, setter = "preserveDrawingBuffer")]
    fn preserve_drawing_buffer_shim(this: &WebGlContextAttributes, val: bool);
    #[wasm_bindgen(method, setter = "stencil")]
    fn stencil_shim(this: &WebGlContextAttributes, val: bool);
    #[wasm_bindgen(method, setter = "xrCompatible")]
    fn xr_compatible_shim(this: &WebGlContextAttributes, val: bool);
}
impl WebGlContextAttributes {
    #[doc = "Construct a new `WebGlContextAttributes`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlContextAttributes`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `alpha` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlContextAttributes`*"]
    pub fn alpha(&mut self, val: bool) -> &mut Self {
        self.alpha_shim(val);
        self
    }
    #[doc = "Change the `antialias` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlContextAttributes`*"]
    pub fn antialias(&mut self, val: bool) -> &mut Self {
        self.antialias_shim(val);
        self
    }
    #[doc = "Change the `depth` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlContextAttributes`*"]
    pub fn depth(&mut self, val: bool) -> &mut Self {
        self.depth_shim(val);
        self
    }
    #[doc = "Change the `failIfMajorPerformanceCaveat` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlContextAttributes`*"]
    pub fn fail_if_major_performance_caveat(&mut self, val: bool) -> &mut Self {
        self.fail_if_major_performance_caveat_shim(val);
        self
    }
    #[cfg(feature = "WebGlPowerPreference")]
    #[doc = "Change the `powerPreference` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlContextAttributes`, `WebGlPowerPreference`*"]
    pub fn power_preference(&mut self, val: WebGlPowerPreference) -> &mut Self {
        self.power_preference_shim(val);
        self
    }
    #[doc = "Change the `premultipliedAlpha` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlContextAttributes`*"]
    pub fn premultiplied_alpha(&mut self, val: bool) -> &mut Self {
        self.premultiplied_alpha_shim(val);
        self
    }
    #[doc = "Change the `preserveDrawingBuffer` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlContextAttributes`*"]
    pub fn preserve_drawing_buffer(&mut self, val: bool) -> &mut Self {
        self.preserve_drawing_buffer_shim(val);
        self
    }
    #[doc = "Change the `stencil` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlContextAttributes`*"]
    pub fn stencil(&mut self, val: bool) -> &mut Self {
        self.stencil_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `xrCompatible` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlContextAttributes`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn xr_compatible(&mut self, val: bool) -> &mut Self {
        self.xr_compatible_shim(val);
        self
    }
}
impl Default for WebGlContextAttributes {
    fn default() -> Self {
        Self::new()
    }
}
