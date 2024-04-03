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
    #[wasm_bindgen(method, setter = "w")]
    fn w_shim(this: &DomPointInit, val: f64);
    #[wasm_bindgen(method, setter = "x")]
    fn x_shim(this: &DomPointInit, val: f64);
    #[wasm_bindgen(method, setter = "y")]
    fn y_shim(this: &DomPointInit, val: f64);
    #[wasm_bindgen(method, setter = "z")]
    fn z_shim(this: &DomPointInit, val: f64);
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
        self.w_shim(val);
        self
    }
    #[doc = "Change the `x` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPointInit`*"]
    pub fn x(&mut self, val: f64) -> &mut Self {
        self.x_shim(val);
        self
    }
    #[doc = "Change the `y` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPointInit`*"]
    pub fn y(&mut self, val: f64) -> &mut Self {
        self.y_shim(val);
        self
    }
    #[doc = "Change the `z` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPointInit`*"]
    pub fn z(&mut self, val: f64) -> &mut Self {
        self.z_shim(val);
        self
    }
}
impl Default for DomPointInit {
    fn default() -> Self {
        Self::new()
    }
}
