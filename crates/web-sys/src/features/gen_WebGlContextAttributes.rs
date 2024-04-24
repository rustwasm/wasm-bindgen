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
    #[wasm_bindgen(method, getter = "alpha")]
    fn alpha_shim(this: &WebGlContextAttributes) -> bool;
    #[wasm_bindgen(method, setter = "alpha")]
    fn set_alpha_shim(this: &WebGlContextAttributes, val: bool);
    #[wasm_bindgen(method, getter = "antialias")]
    fn antialias_shim(this: &WebGlContextAttributes) -> bool;
    #[wasm_bindgen(method, setter = "antialias")]
    fn set_antialias_shim(this: &WebGlContextAttributes, val: bool);
    #[wasm_bindgen(method, getter = "depth")]
    fn depth_shim(this: &WebGlContextAttributes) -> bool;
    #[wasm_bindgen(method, setter = "depth")]
    fn set_depth_shim(this: &WebGlContextAttributes, val: bool);
    #[wasm_bindgen(method, getter = "failIfMajorPerformanceCaveat")]
    fn fail_if_major_performance_caveat_shim(this: &WebGlContextAttributes) -> bool;
    #[wasm_bindgen(method, setter = "failIfMajorPerformanceCaveat")]
    fn set_fail_if_major_performance_caveat_shim(this: &WebGlContextAttributes, val: bool);
    #[cfg(feature = "WebGlPowerPreference")]
    #[wasm_bindgen(method, getter = "powerPreference")]
    fn power_preference_shim(this: &WebGlContextAttributes) -> WebGlPowerPreference;
    #[cfg(feature = "WebGlPowerPreference")]
    #[wasm_bindgen(method, setter = "powerPreference")]
    fn set_power_preference_shim(this: &WebGlContextAttributes, val: WebGlPowerPreference);
    #[wasm_bindgen(method, getter = "premultipliedAlpha")]
    fn premultiplied_alpha_shim(this: &WebGlContextAttributes) -> bool;
    #[wasm_bindgen(method, setter = "premultipliedAlpha")]
    fn set_premultiplied_alpha_shim(this: &WebGlContextAttributes, val: bool);
    #[wasm_bindgen(method, getter = "preserveDrawingBuffer")]
    fn preserve_drawing_buffer_shim(this: &WebGlContextAttributes) -> bool;
    #[wasm_bindgen(method, setter = "preserveDrawingBuffer")]
    fn set_preserve_drawing_buffer_shim(this: &WebGlContextAttributes, val: bool);
    #[wasm_bindgen(method, getter = "stencil")]
    fn stencil_shim(this: &WebGlContextAttributes) -> bool;
    #[wasm_bindgen(method, setter = "stencil")]
    fn set_stencil_shim(this: &WebGlContextAttributes, val: bool);
    #[wasm_bindgen(method, getter = "xrCompatible")]
    fn xr_compatible_shim(this: &WebGlContextAttributes) -> bool;
    #[wasm_bindgen(method, setter = "xrCompatible")]
    fn set_xr_compatible_shim(this: &WebGlContextAttributes, val: bool);
}
#[doc = "The trait to access properties on the `WebGlContextAttributes` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `WebGlContextAttributes`*"]
pub trait WebGlContextAttributesGetters {
    #[doc = "Get the `alpha` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlContextAttributes`*"]
    fn alpha(&self) -> bool;
    #[doc = "Get the `antialias` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlContextAttributes`*"]
    fn antialias(&self) -> bool;
    #[doc = "Get the `depth` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlContextAttributes`*"]
    fn depth(&self) -> bool;
    #[doc = "Get the `failIfMajorPerformanceCaveat` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlContextAttributes`*"]
    fn fail_if_major_performance_caveat(&self) -> bool;
    #[cfg(feature = "WebGlPowerPreference")]
    #[doc = "Get the `powerPreference` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlContextAttributes`, `WebGlPowerPreference`*"]
    fn power_preference(&self) -> WebGlPowerPreference;
    #[doc = "Get the `premultipliedAlpha` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlContextAttributes`*"]
    fn premultiplied_alpha(&self) -> bool;
    #[doc = "Get the `preserveDrawingBuffer` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlContextAttributes`*"]
    fn preserve_drawing_buffer(&self) -> bool;
    #[doc = "Get the `stencil` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlContextAttributes`*"]
    fn stencil(&self) -> bool;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `xrCompatible` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlContextAttributes`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn xr_compatible(&self) -> bool;
}
impl WebGlContextAttributesGetters for WebGlContextAttributes {
    fn alpha(&self) -> bool {
        self.alpha_shim()
    }
    fn antialias(&self) -> bool {
        self.antialias_shim()
    }
    fn depth(&self) -> bool {
        self.depth_shim()
    }
    fn fail_if_major_performance_caveat(&self) -> bool {
        self.fail_if_major_performance_caveat_shim()
    }
    #[cfg(feature = "WebGlPowerPreference")]
    fn power_preference(&self) -> WebGlPowerPreference {
        self.power_preference_shim()
    }
    fn premultiplied_alpha(&self) -> bool {
        self.premultiplied_alpha_shim()
    }
    fn preserve_drawing_buffer(&self) -> bool {
        self.preserve_drawing_buffer_shim()
    }
    fn stencil(&self) -> bool {
        self.stencil_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn xr_compatible(&self) -> bool {
        self.xr_compatible_shim()
    }
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
        self.set_alpha_shim(val);
        self
    }
    #[doc = "Change the `antialias` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlContextAttributes`*"]
    pub fn antialias(&mut self, val: bool) -> &mut Self {
        self.set_antialias_shim(val);
        self
    }
    #[doc = "Change the `depth` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlContextAttributes`*"]
    pub fn depth(&mut self, val: bool) -> &mut Self {
        self.set_depth_shim(val);
        self
    }
    #[doc = "Change the `failIfMajorPerformanceCaveat` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlContextAttributes`*"]
    pub fn fail_if_major_performance_caveat(&mut self, val: bool) -> &mut Self {
        self.set_fail_if_major_performance_caveat_shim(val);
        self
    }
    #[cfg(feature = "WebGlPowerPreference")]
    #[doc = "Change the `powerPreference` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlContextAttributes`, `WebGlPowerPreference`*"]
    pub fn power_preference(&mut self, val: WebGlPowerPreference) -> &mut Self {
        self.set_power_preference_shim(val);
        self
    }
    #[doc = "Change the `premultipliedAlpha` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlContextAttributes`*"]
    pub fn premultiplied_alpha(&mut self, val: bool) -> &mut Self {
        self.set_premultiplied_alpha_shim(val);
        self
    }
    #[doc = "Change the `preserveDrawingBuffer` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlContextAttributes`*"]
    pub fn preserve_drawing_buffer(&mut self, val: bool) -> &mut Self {
        self.set_preserve_drawing_buffer_shim(val);
        self
    }
    #[doc = "Change the `stencil` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlContextAttributes`*"]
    pub fn stencil(&mut self, val: bool) -> &mut Self {
        self.set_stencil_shim(val);
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
        self.set_xr_compatible_shim(val);
        self
    }
}
impl Default for WebGlContextAttributes {
    fn default() -> Self {
        Self::new()
    }
}
