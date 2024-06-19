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
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `alpha` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrWebGlLayerInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "alpha")]
    pub fn get_alpha(this: &XrWebGlLayerInit) -> Option<bool>;
    #[wasm_bindgen(method, setter = "alpha")]
    fn set_alpha(this: &XrWebGlLayerInit, val: bool);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `antialias` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrWebGlLayerInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "antialias")]
    pub fn get_antialias(this: &XrWebGlLayerInit) -> Option<bool>;
    #[wasm_bindgen(method, setter = "antialias")]
    fn set_antialias(this: &XrWebGlLayerInit, val: bool);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `depth` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrWebGlLayerInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "depth")]
    pub fn get_depth(this: &XrWebGlLayerInit) -> Option<bool>;
    #[wasm_bindgen(method, setter = "depth")]
    fn set_depth(this: &XrWebGlLayerInit, val: bool);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `framebufferScaleFactor` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrWebGlLayerInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "framebufferScaleFactor")]
    pub fn get_framebuffer_scale_factor(this: &XrWebGlLayerInit) -> Option<f64>;
    #[wasm_bindgen(method, setter = "framebufferScaleFactor")]
    fn set_framebuffer_scale_factor(this: &XrWebGlLayerInit, val: f64);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `ignoreDepthValues` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrWebGlLayerInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "ignoreDepthValues")]
    pub fn get_ignore_depth_values(this: &XrWebGlLayerInit) -> Option<bool>;
    #[wasm_bindgen(method, setter = "ignoreDepthValues")]
    fn set_ignore_depth_values(this: &XrWebGlLayerInit, val: bool);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `stencil` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrWebGlLayerInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "stencil")]
    pub fn get_stencil(this: &XrWebGlLayerInit) -> Option<bool>;
    #[wasm_bindgen(method, setter = "stencil")]
    fn set_stencil(this: &XrWebGlLayerInit, val: bool);
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
        self.set_alpha(val);
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
        self.set_antialias(val);
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
        self.set_depth(val);
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
        self.set_framebuffer_scale_factor(val);
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
        self.set_ignore_depth_values(val);
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
        self.set_stencil(val);
        self
    }
}
#[cfg(web_sys_unstable_apis)]
impl Default for XrWebGlLayerInit {
    fn default() -> Self {
        Self::new()
    }
}
