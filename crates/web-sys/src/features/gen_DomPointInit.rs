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
    #[wasm_bindgen(method, getter = "w")]
    fn w_shim(this: &DomPointInit) -> f64;
    #[wasm_bindgen(method, setter = "w")]
    fn set_w_shim(this: &DomPointInit, val: f64);
    #[wasm_bindgen(method, getter = "x")]
    fn x_shim(this: &DomPointInit) -> f64;
    #[wasm_bindgen(method, setter = "x")]
    fn set_x_shim(this: &DomPointInit, val: f64);
    #[wasm_bindgen(method, getter = "y")]
    fn y_shim(this: &DomPointInit) -> f64;
    #[wasm_bindgen(method, setter = "y")]
    fn set_y_shim(this: &DomPointInit, val: f64);
    #[wasm_bindgen(method, getter = "z")]
    fn z_shim(this: &DomPointInit) -> f64;
    #[wasm_bindgen(method, setter = "z")]
    fn set_z_shim(this: &DomPointInit, val: f64);
}
#[doc = "The trait to access properties on the `DomPointInit` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `DomPointInit`*"]
pub trait DomPointInitGetters {
    #[doc = "Get the `w` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPointInit`*"]
    fn w(&self) -> f64;
    #[doc = "Get the `x` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPointInit`*"]
    fn x(&self) -> f64;
    #[doc = "Get the `y` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPointInit`*"]
    fn y(&self) -> f64;
    #[doc = "Get the `z` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPointInit`*"]
    fn z(&self) -> f64;
}
impl DomPointInitGetters for DomPointInit {
    fn w(&self) -> f64 {
        self.w_shim()
    }
    fn x(&self) -> f64 {
        self.x_shim()
    }
    fn y(&self) -> f64 {
        self.y_shim()
    }
    fn z(&self) -> f64 {
        self.z_shim()
    }
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
        self.set_w_shim(val);
        self
    }
    #[doc = "Change the `x` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPointInit`*"]
    pub fn x(&mut self, val: f64) -> &mut Self {
        self.set_x_shim(val);
        self
    }
    #[doc = "Change the `y` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPointInit`*"]
    pub fn y(&mut self, val: f64) -> &mut Self {
        self.set_y_shim(val);
        self
    }
    #[doc = "Change the `z` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPointInit`*"]
    pub fn z(&mut self, val: f64) -> &mut Self {
        self.set_z_shim(val);
        self
    }
}
impl Default for DomPointInit {
    fn default() -> Self {
        Self::new()
    }
}
