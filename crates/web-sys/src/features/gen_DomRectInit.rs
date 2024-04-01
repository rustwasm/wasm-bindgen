#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = DOMRectInit)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DomRectInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomRectInit`*"]
    pub type DomRectInit;
    #[wasm_bindgen(method, setter = "height")]
    fn height_shim(this: &DomRectInit, val: f64);
    #[wasm_bindgen(method, setter = "width")]
    fn width_shim(this: &DomRectInit, val: f64);
    #[wasm_bindgen(method, setter = "x")]
    fn x_shim(this: &DomRectInit, val: f64);
    #[wasm_bindgen(method, setter = "y")]
    fn y_shim(this: &DomRectInit, val: f64);
}
impl DomRectInit {
    #[doc = "Construct a new `DomRectInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomRectInit`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `height` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomRectInit`*"]
    pub fn height(&mut self, val: f64) -> &mut Self {
        self.height_shim(val);
        self
    }
    #[doc = "Change the `width` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomRectInit`*"]
    pub fn width(&mut self, val: f64) -> &mut Self {
        self.width_shim(val);
        self
    }
    #[doc = "Change the `x` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomRectInit`*"]
    pub fn x(&mut self, val: f64) -> &mut Self {
        self.x_shim(val);
        self
    }
    #[doc = "Change the `y` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomRectInit`*"]
    pub fn y(&mut self, val: f64) -> &mut Self {
        self.y_shim(val);
        self
    }
}
impl Default for DomRectInit {
    fn default() -> Self {
        Self::new()
    }
}
