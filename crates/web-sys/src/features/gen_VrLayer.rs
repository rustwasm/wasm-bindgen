#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = VRLayer)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `VrLayer` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrLayer`*"]
    pub type VrLayer;
    #[doc = "Get the `leftBounds` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrLayer`*"]
    #[wasm_bindgen(method, getter = "leftBounds")]
    pub fn get_left_bounds(this: &VrLayer) -> Option<::js_sys::Array>;
    #[wasm_bindgen(method, setter = "leftBounds")]
    fn set_left_bounds(this: &VrLayer, val: &::wasm_bindgen::JsValue);
    #[doc = "Get the `rightBounds` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrLayer`*"]
    #[wasm_bindgen(method, getter = "rightBounds")]
    pub fn get_right_bounds(this: &VrLayer) -> Option<::js_sys::Array>;
    #[wasm_bindgen(method, setter = "rightBounds")]
    fn set_right_bounds(this: &VrLayer, val: &::wasm_bindgen::JsValue);
    #[cfg(feature = "HtmlCanvasElement")]
    #[doc = "Get the `source` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlCanvasElement`, `VrLayer`*"]
    #[wasm_bindgen(method, getter = "source")]
    pub fn get_source(this: &VrLayer) -> Option<HtmlCanvasElement>;
    #[cfg(feature = "HtmlCanvasElement")]
    #[wasm_bindgen(method, setter = "source")]
    fn set_source(this: &VrLayer, val: Option<&HtmlCanvasElement>);
}
impl VrLayer {
    #[doc = "Construct a new `VrLayer`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrLayer`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `leftBounds` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrLayer`*"]
    pub fn left_bounds(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_left_bounds(val);
        self
    }
    #[doc = "Change the `rightBounds` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrLayer`*"]
    pub fn right_bounds(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_right_bounds(val);
        self
    }
    #[cfg(feature = "HtmlCanvasElement")]
    #[doc = "Change the `source` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlCanvasElement`, `VrLayer`*"]
    pub fn source(&mut self, val: Option<&HtmlCanvasElement>) -> &mut Self {
        self.set_source(val);
        self
    }
}
impl Default for VrLayer {
    fn default() -> Self {
        Self::new()
    }
}
