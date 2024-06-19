#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = DOMPointInit)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DomPointInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPointInit`*"]
    pub type DomPointInit;
    #[doc = "Get the `w` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPointInit`*"]
    #[wasm_bindgen(method, getter = "w")]
    pub fn get_w(this: &DomPointInit) -> Option<f64>;
    #[wasm_bindgen(method, setter = "w")]
    fn set_w(this: &DomPointInit, val: f64);
    #[doc = "Get the `x` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPointInit`*"]
    #[wasm_bindgen(method, getter = "x")]
    pub fn get_x(this: &DomPointInit) -> Option<f64>;
    #[wasm_bindgen(method, setter = "x")]
    fn set_x(this: &DomPointInit, val: f64);
    #[doc = "Get the `y` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPointInit`*"]
    #[wasm_bindgen(method, getter = "y")]
    pub fn get_y(this: &DomPointInit) -> Option<f64>;
    #[wasm_bindgen(method, setter = "y")]
    fn set_y(this: &DomPointInit, val: f64);
    #[doc = "Get the `z` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPointInit`*"]
    #[wasm_bindgen(method, getter = "z")]
    pub fn get_z(this: &DomPointInit) -> Option<f64>;
    #[wasm_bindgen(method, setter = "z")]
    fn set_z(this: &DomPointInit, val: f64);
}
impl DomPointInit {
    #[doc = "Construct a new `DomPointInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPointInit`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `w` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPointInit`*"]
    pub fn w(&mut self, val: f64) -> &mut Self {
        self.set_w(val);
        self
    }
    #[doc = "Change the `x` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPointInit`*"]
    pub fn x(&mut self, val: f64) -> &mut Self {
        self.set_x(val);
        self
    }
    #[doc = "Change the `y` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPointInit`*"]
    pub fn y(&mut self, val: f64) -> &mut Self {
        self.set_y(val);
        self
    }
    #[doc = "Change the `z` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPointInit`*"]
    pub fn z(&mut self, val: f64) -> &mut Self {
        self.set_z(val);
        self
    }
}
impl Default for DomPointInit {
    fn default() -> Self {
        Self::new()
    }
}
