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
    #[wasm_bindgen(method, getter = "baseLayer")]
    fn base_layer_shim(this: &XrRenderStateInit) -> Option<XrWebGlLayer>;
    #[cfg(feature = "XrWebGlLayer")]
    #[wasm_bindgen(method, setter = "baseLayer")]
    fn set_base_layer_shim(this: &XrRenderStateInit, val: Option<&XrWebGlLayer>);
    #[wasm_bindgen(method, getter = "depthFar")]
    fn depth_far_shim(this: &XrRenderStateInit) -> f64;
    #[wasm_bindgen(method, setter = "depthFar")]
    fn set_depth_far_shim(this: &XrRenderStateInit, val: f64);
    #[wasm_bindgen(method, getter = "depthNear")]
    fn depth_near_shim(this: &XrRenderStateInit) -> f64;
    #[wasm_bindgen(method, setter = "depthNear")]
    fn set_depth_near_shim(this: &XrRenderStateInit, val: f64);
    #[wasm_bindgen(method, getter = "inlineVerticalFieldOfView")]
    fn inline_vertical_field_of_view_shim(this: &XrRenderStateInit) -> f64;
    #[wasm_bindgen(method, setter = "inlineVerticalFieldOfView")]
    fn set_inline_vertical_field_of_view_shim(this: &XrRenderStateInit, val: f64);
    #[wasm_bindgen(method, getter = "layers")]
    fn layers_shim(this: &XrRenderStateInit) -> Option<::js_sys::Array>;
    #[wasm_bindgen(method, setter = "layers")]
    fn set_layers_shim(this: &XrRenderStateInit, val: &::wasm_bindgen::JsValue);
}
#[cfg(web_sys_unstable_apis)]
#[doc = "The trait to access properties on the `XrRenderStateInit` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `XrRenderStateInit`*"]
pub trait XrRenderStateInitGetters {
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "XrWebGlLayer")]
    #[doc = "Get the `baseLayer` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrRenderStateInit`, `XrWebGlLayer`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn base_layer(&self) -> Option<XrWebGlLayer>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `depthFar` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrRenderStateInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn depth_far(&self) -> f64;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `depthNear` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrRenderStateInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn depth_near(&self) -> f64;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `inlineVerticalFieldOfView` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrRenderStateInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn inline_vertical_field_of_view(&self) -> f64;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `layers` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrRenderStateInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn layers(&self) -> Option<::js_sys::Array>;
}
#[cfg(web_sys_unstable_apis)]
impl XrRenderStateInitGetters for XrRenderStateInit {
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "XrWebGlLayer")]
    fn base_layer(&self) -> Option<XrWebGlLayer> {
        self.base_layer_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn depth_far(&self) -> f64 {
        self.depth_far_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn depth_near(&self) -> f64 {
        self.depth_near_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn inline_vertical_field_of_view(&self) -> f64 {
        self.inline_vertical_field_of_view_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn layers(&self) -> Option<::js_sys::Array> {
        self.layers_shim()
    }
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
        self.set_base_layer_shim(val);
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
        self.set_depth_far_shim(val);
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
        self.set_depth_near_shim(val);
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
        self.set_inline_vertical_field_of_view_shim(val);
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
        self.set_layers_shim(val.unwrap_or(&::wasm_bindgen::JsValue::NULL));
        self
    }
}
#[cfg(web_sys_unstable_apis)]
impl Default for XrRenderStateInit {
    fn default() -> Self {
        Self::new()
    }
}
