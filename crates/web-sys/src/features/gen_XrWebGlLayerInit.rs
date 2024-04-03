#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = XRWebGLLayerInit)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `XrWebGlLayerInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrWebGlLayerInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type XrWebGlLayerInit;
    #[wasm_bindgen(method, setter = "alpha")]
    fn alpha_shim(this: &XrWebGlLayerInit, val: bool);
    #[wasm_bindgen(method, setter = "antialias")]
    fn antialias_shim(this: &XrWebGlLayerInit, val: bool);
    #[wasm_bindgen(method, setter = "depth")]
    fn depth_shim(this: &XrWebGlLayerInit, val: bool);
    #[wasm_bindgen(method, setter = "framebufferScaleFactor")]
    fn framebuffer_scale_factor_shim(this: &XrWebGlLayerInit, val: f64);
    #[wasm_bindgen(method, setter = "ignoreDepthValues")]
    fn ignore_depth_values_shim(this: &XrWebGlLayerInit, val: bool);
    #[wasm_bindgen(method, setter = "stencil")]
    fn stencil_shim(this: &XrWebGlLayerInit, val: bool);
}
#[cfg(web_sys_unstable_apis)]
impl XrWebGlLayerInit {
    #[doc = "Construct a new `XrWebGlLayerInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrWebGlLayerInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `alpha` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrWebGlLayerInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn alpha(&mut self, val: bool) -> &mut Self {
        self.alpha_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `antialias` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrWebGlLayerInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn antialias(&mut self, val: bool) -> &mut Self {
        self.antialias_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `depth` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrWebGlLayerInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn depth(&mut self, val: bool) -> &mut Self {
        self.depth_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `framebufferScaleFactor` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrWebGlLayerInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn framebuffer_scale_factor(&mut self, val: f64) -> &mut Self {
        self.framebuffer_scale_factor_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `ignoreDepthValues` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrWebGlLayerInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn ignore_depth_values(&mut self, val: bool) -> &mut Self {
        self.ignore_depth_values_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `stencil` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrWebGlLayerInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn stencil(&mut self, val: bool) -> &mut Self {
        self.stencil_shim(val);
        self
    }
}
#[cfg(web_sys_unstable_apis)]
impl Default for XrWebGlLayerInit {
    fn default() -> Self {
        Self::new()
    }
}
