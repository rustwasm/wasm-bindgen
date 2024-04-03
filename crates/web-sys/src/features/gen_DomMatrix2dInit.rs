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
    #[wasm_bindgen(method, setter = "a")]
    fn a_shim(this: &DomMatrix2dInit, val: f64);
    #[wasm_bindgen(method, setter = "b")]
    fn b_shim(this: &DomMatrix2dInit, val: f64);
    #[wasm_bindgen(method, setter = "c")]
    fn c_shim(this: &DomMatrix2dInit, val: f64);
    #[wasm_bindgen(method, setter = "d")]
    fn d_shim(this: &DomMatrix2dInit, val: f64);
    #[wasm_bindgen(method, setter = "e")]
    fn e_shim(this: &DomMatrix2dInit, val: f64);
    #[wasm_bindgen(method, setter = "f")]
    fn f_shim(this: &DomMatrix2dInit, val: f64);
    #[wasm_bindgen(method, setter = "m11")]
    fn m11_shim(this: &DomMatrix2dInit, val: f64);
    #[wasm_bindgen(method, setter = "m12")]
    fn m12_shim(this: &DomMatrix2dInit, val: f64);
    #[wasm_bindgen(method, setter = "m21")]
    fn m21_shim(this: &DomMatrix2dInit, val: f64);
    #[wasm_bindgen(method, setter = "m22")]
    fn m22_shim(this: &DomMatrix2dInit, val: f64);
    #[wasm_bindgen(method, setter = "m41")]
    fn m41_shim(this: &DomMatrix2dInit, val: f64);
    #[wasm_bindgen(method, setter = "m42")]
    fn m42_shim(this: &DomMatrix2dInit, val: f64);
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
        self.a_shim(val);
        self
    }
    #[doc = "Change the `b` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix2dInit`*"]
    pub fn b(&mut self, val: f64) -> &mut Self {
        self.b_shim(val);
        self
    }
    #[doc = "Change the `c` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix2dInit`*"]
    pub fn c(&mut self, val: f64) -> &mut Self {
        self.c_shim(val);
        self
    }
    #[doc = "Change the `d` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix2dInit`*"]
    pub fn d(&mut self, val: f64) -> &mut Self {
        self.d_shim(val);
        self
    }
    #[doc = "Change the `e` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix2dInit`*"]
    pub fn e(&mut self, val: f64) -> &mut Self {
        self.e_shim(val);
        self
    }
    #[doc = "Change the `f` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix2dInit`*"]
    pub fn f(&mut self, val: f64) -> &mut Self {
        self.f_shim(val);
        self
    }
    #[doc = "Change the `m11` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix2dInit`*"]
    pub fn m11(&mut self, val: f64) -> &mut Self {
        self.m11_shim(val);
        self
    }
    #[doc = "Change the `m12` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix2dInit`*"]
    pub fn m12(&mut self, val: f64) -> &mut Self {
        self.m12_shim(val);
        self
    }
    #[doc = "Change the `m21` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix2dInit`*"]
    pub fn m21(&mut self, val: f64) -> &mut Self {
        self.m21_shim(val);
        self
    }
    #[doc = "Change the `m22` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix2dInit`*"]
    pub fn m22(&mut self, val: f64) -> &mut Self {
        self.m22_shim(val);
        self
    }
    #[doc = "Change the `m41` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix2dInit`*"]
    pub fn m41(&mut self, val: f64) -> &mut Self {
        self.m41_shim(val);
        self
    }
    #[doc = "Change the `m42` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix2dInit`*"]
    pub fn m42(&mut self, val: f64) -> &mut Self {
        self.m42_shim(val);
        self
    }
}
impl Default for DomMatrix2dInit {
    fn default() -> Self {
        Self::new()
    }
}
