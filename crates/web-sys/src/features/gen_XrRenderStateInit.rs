#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = XRRenderStateInit)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `XrRenderStateInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrRenderStateInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type XrRenderStateInit;
    #[cfg(feature = "XrWebGlLayer")]
    #[wasm_bindgen(method, setter = "baseLayer")]
    fn base_layer_shim(this: &XrRenderStateInit, val: Option<&XrWebGlLayer>);
    #[wasm_bindgen(method, setter = "depthFar")]
    fn depth_far_shim(this: &XrRenderStateInit, val: f64);
    #[wasm_bindgen(method, setter = "depthNear")]
    fn depth_near_shim(this: &XrRenderStateInit, val: f64);
    #[wasm_bindgen(method, setter = "inlineVerticalFieldOfView")]
    fn inline_vertical_field_of_view_shim(this: &XrRenderStateInit, val: f64);
    #[wasm_bindgen(method, setter = "layers")]
    fn layers_shim(this: &XrRenderStateInit, val: &::wasm_bindgen::JsValue);
}
#[cfg(web_sys_unstable_apis)]
impl XrRenderStateInit {
    #[doc = "Construct a new `XrRenderStateInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrRenderStateInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "XrWebGlLayer")]
    #[doc = "Change the `baseLayer` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrRenderStateInit`, `XrWebGlLayer`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn base_layer(&mut self, val: Option<&XrWebGlLayer>) -> &mut Self {
        self.base_layer_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `depthFar` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrRenderStateInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn depth_far(&mut self, val: f64) -> &mut Self {
        self.depth_far_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `depthNear` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrRenderStateInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn depth_near(&mut self, val: f64) -> &mut Self {
        self.depth_near_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `inlineVerticalFieldOfView` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrRenderStateInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn inline_vertical_field_of_view(&mut self, val: f64) -> &mut Self {
        self.inline_vertical_field_of_view_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `layers` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrRenderStateInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn layers(&mut self, val: Option<&::wasm_bindgen::JsValue>) -> &mut Self {
        self.layers_shim(val.unwrap_or(&::wasm_bindgen::JsValue::NULL));
        self
    }
}
#[cfg(web_sys_unstable_apis)]
impl Default for XrRenderStateInit {
    fn default() -> Self {
        Self::new()
    }
}
