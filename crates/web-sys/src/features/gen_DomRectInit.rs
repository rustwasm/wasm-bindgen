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
    #[wasm_bindgen(method, getter = "height")]
    fn height_shim(this: &DomRectInit) -> f64;
    #[wasm_bindgen(method, setter = "height")]
    fn set_height_shim(this: &DomRectInit, val: f64);
    #[wasm_bindgen(method, getter = "width")]
    fn width_shim(this: &DomRectInit) -> f64;
    #[wasm_bindgen(method, setter = "width")]
    fn set_width_shim(this: &DomRectInit, val: f64);
    #[wasm_bindgen(method, getter = "x")]
    fn x_shim(this: &DomRectInit) -> f64;
    #[wasm_bindgen(method, setter = "x")]
    fn set_x_shim(this: &DomRectInit, val: f64);
    #[wasm_bindgen(method, getter = "y")]
    fn y_shim(this: &DomRectInit) -> f64;
    #[wasm_bindgen(method, setter = "y")]
    fn set_y_shim(this: &DomRectInit, val: f64);
}
#[doc = "The trait to access properties on the `DomRectInit` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `DomRectInit`*"]
pub trait DomRectInitGetters {
    #[doc = "Get the `height` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomRectInit`*"]
    fn height(&self) -> f64;
    #[doc = "Get the `width` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomRectInit`*"]
    fn width(&self) -> f64;
    #[doc = "Get the `x` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomRectInit`*"]
    fn x(&self) -> f64;
    #[doc = "Get the `y` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomRectInit`*"]
    fn y(&self) -> f64;
}
impl DomRectInitGetters for DomRectInit {
    fn height(&self) -> f64 {
        self.height_shim()
    }
    fn width(&self) -> f64 {
        self.width_shim()
    }
    fn x(&self) -> f64 {
        self.x_shim()
    }
    fn y(&self) -> f64 {
        self.y_shim()
    }
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
        self.set_height_shim(val);
        self
    }
    #[doc = "Change the `width` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomRectInit`*"]
    pub fn width(&mut self, val: f64) -> &mut Self {
        self.set_width_shim(val);
        self
    }
    #[doc = "Change the `x` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomRectInit`*"]
    pub fn x(&mut self, val: f64) -> &mut Self {
        self.set_x_shim(val);
        self
    }
    #[doc = "Change the `y` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomRectInit`*"]
    pub fn y(&mut self, val: f64) -> &mut Self {
        self.set_y_shim(val);
        self
    }
}
impl Default for DomRectInit {
    fn default() -> Self {
        Self::new()
    }
}
