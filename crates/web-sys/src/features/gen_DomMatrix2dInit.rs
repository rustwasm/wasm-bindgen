#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = DOMMatrix2DInit)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DomMatrix2dInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix2dInit`*"]
    pub type DomMatrix2dInit;
    #[wasm_bindgen(method, getter = "a")]
    fn a_shim(this: &DomMatrix2dInit) -> f64;
    #[wasm_bindgen(method, setter = "a")]
    fn set_a_shim(this: &DomMatrix2dInit, val: f64);
    #[wasm_bindgen(method, getter = "b")]
    fn b_shim(this: &DomMatrix2dInit) -> f64;
    #[wasm_bindgen(method, setter = "b")]
    fn set_b_shim(this: &DomMatrix2dInit, val: f64);
    #[wasm_bindgen(method, getter = "c")]
    fn c_shim(this: &DomMatrix2dInit) -> f64;
    #[wasm_bindgen(method, setter = "c")]
    fn set_c_shim(this: &DomMatrix2dInit, val: f64);
    #[wasm_bindgen(method, getter = "d")]
    fn d_shim(this: &DomMatrix2dInit) -> f64;
    #[wasm_bindgen(method, setter = "d")]
    fn set_d_shim(this: &DomMatrix2dInit, val: f64);
    #[wasm_bindgen(method, getter = "e")]
    fn e_shim(this: &DomMatrix2dInit) -> f64;
    #[wasm_bindgen(method, setter = "e")]
    fn set_e_shim(this: &DomMatrix2dInit, val: f64);
    #[wasm_bindgen(method, getter = "f")]
    fn f_shim(this: &DomMatrix2dInit) -> f64;
    #[wasm_bindgen(method, setter = "f")]
    fn set_f_shim(this: &DomMatrix2dInit, val: f64);
    #[wasm_bindgen(method, getter = "m11")]
    fn m11_shim(this: &DomMatrix2dInit) -> f64;
    #[wasm_bindgen(method, setter = "m11")]
    fn set_m11_shim(this: &DomMatrix2dInit, val: f64);
    #[wasm_bindgen(method, getter = "m12")]
    fn m12_shim(this: &DomMatrix2dInit) -> f64;
    #[wasm_bindgen(method, setter = "m12")]
    fn set_m12_shim(this: &DomMatrix2dInit, val: f64);
    #[wasm_bindgen(method, getter = "m21")]
    fn m21_shim(this: &DomMatrix2dInit) -> f64;
    #[wasm_bindgen(method, setter = "m21")]
    fn set_m21_shim(this: &DomMatrix2dInit, val: f64);
    #[wasm_bindgen(method, getter = "m22")]
    fn m22_shim(this: &DomMatrix2dInit) -> f64;
    #[wasm_bindgen(method, setter = "m22")]
    fn set_m22_shim(this: &DomMatrix2dInit, val: f64);
    #[wasm_bindgen(method, getter = "m41")]
    fn m41_shim(this: &DomMatrix2dInit) -> f64;
    #[wasm_bindgen(method, setter = "m41")]
    fn set_m41_shim(this: &DomMatrix2dInit, val: f64);
    #[wasm_bindgen(method, getter = "m42")]
    fn m42_shim(this: &DomMatrix2dInit) -> f64;
    #[wasm_bindgen(method, setter = "m42")]
    fn set_m42_shim(this: &DomMatrix2dInit, val: f64);
}
#[doc = "The trait to access properties on the `DomMatrix2dInit` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `DomMatrix2dInit`*"]
pub trait DomMatrix2dInitGetters {
    #[doc = "Get the `a` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix2dInit`*"]
    fn a(&self) -> f64;
    #[doc = "Get the `b` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix2dInit`*"]
    fn b(&self) -> f64;
    #[doc = "Get the `c` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix2dInit`*"]
    fn c(&self) -> f64;
    #[doc = "Get the `d` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix2dInit`*"]
    fn d(&self) -> f64;
    #[doc = "Get the `e` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix2dInit`*"]
    fn e(&self) -> f64;
    #[doc = "Get the `f` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix2dInit`*"]
    fn f(&self) -> f64;
    #[doc = "Get the `m11` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix2dInit`*"]
    fn m11(&self) -> f64;
    #[doc = "Get the `m12` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix2dInit`*"]
    fn m12(&self) -> f64;
    #[doc = "Get the `m21` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix2dInit`*"]
    fn m21(&self) -> f64;
    #[doc = "Get the `m22` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix2dInit`*"]
    fn m22(&self) -> f64;
    #[doc = "Get the `m41` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix2dInit`*"]
    fn m41(&self) -> f64;
    #[doc = "Get the `m42` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix2dInit`*"]
    fn m42(&self) -> f64;
}
impl DomMatrix2dInitGetters for DomMatrix2dInit {
    fn a(&self) -> f64 {
        self.a_shim()
    }
    fn b(&self) -> f64 {
        self.b_shim()
    }
    fn c(&self) -> f64 {
        self.c_shim()
    }
    fn d(&self) -> f64 {
        self.d_shim()
    }
    fn e(&self) -> f64 {
        self.e_shim()
    }
    fn f(&self) -> f64 {
        self.f_shim()
    }
    fn m11(&self) -> f64 {
        self.m11_shim()
    }
    fn m12(&self) -> f64 {
        self.m12_shim()
    }
    fn m21(&self) -> f64 {
        self.m21_shim()
    }
    fn m22(&self) -> f64 {
        self.m22_shim()
    }
    fn m41(&self) -> f64 {
        self.m41_shim()
    }
    fn m42(&self) -> f64 {
        self.m42_shim()
    }
}
impl DomMatrix2dInit {
    #[doc = "Construct a new `DomMatrix2dInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix2dInit`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `a` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix2dInit`*"]
    pub fn a(&mut self, val: f64) -> &mut Self {
        self.set_a_shim(val);
        self
    }
    #[doc = "Change the `b` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix2dInit`*"]
    pub fn b(&mut self, val: f64) -> &mut Self {
        self.set_b_shim(val);
        self
    }
    #[doc = "Change the `c` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix2dInit`*"]
    pub fn c(&mut self, val: f64) -> &mut Self {
        self.set_c_shim(val);
        self
    }
    #[doc = "Change the `d` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix2dInit`*"]
    pub fn d(&mut self, val: f64) -> &mut Self {
        self.set_d_shim(val);
        self
    }
    #[doc = "Change the `e` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix2dInit`*"]
    pub fn e(&mut self, val: f64) -> &mut Self {
        self.set_e_shim(val);
        self
    }
    #[doc = "Change the `f` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix2dInit`*"]
    pub fn f(&mut self, val: f64) -> &mut Self {
        self.set_f_shim(val);
        self
    }
    #[doc = "Change the `m11` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix2dInit`*"]
    pub fn m11(&mut self, val: f64) -> &mut Self {
        self.set_m11_shim(val);
        self
    }
    #[doc = "Change the `m12` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix2dInit`*"]
    pub fn m12(&mut self, val: f64) -> &mut Self {
        self.set_m12_shim(val);
        self
    }
    #[doc = "Change the `m21` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix2dInit`*"]
    pub fn m21(&mut self, val: f64) -> &mut Self {
        self.set_m21_shim(val);
        self
    }
    #[doc = "Change the `m22` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix2dInit`*"]
    pub fn m22(&mut self, val: f64) -> &mut Self {
        self.set_m22_shim(val);
        self
    }
    #[doc = "Change the `m41` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix2dInit`*"]
    pub fn m41(&mut self, val: f64) -> &mut Self {
        self.set_m41_shim(val);
        self
    }
    #[doc = "Change the `m42` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix2dInit`*"]
    pub fn m42(&mut self, val: f64) -> &mut Self {
        self.set_m42_shim(val);
        self
    }
}
impl Default for DomMatrix2dInit {
    fn default() -> Self {
        Self::new()
    }
}
