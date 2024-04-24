#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = DOMMatrixInit)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DomMatrixInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrixInit`*"]
    pub type DomMatrixInit;
    #[wasm_bindgen(method, getter = "a")]
    fn a_shim(this: &DomMatrixInit) -> f64;
    #[wasm_bindgen(method, setter = "a")]
    fn set_a_shim(this: &DomMatrixInit, val: f64);
    #[wasm_bindgen(method, getter = "b")]
    fn b_shim(this: &DomMatrixInit) -> f64;
    #[wasm_bindgen(method, setter = "b")]
    fn set_b_shim(this: &DomMatrixInit, val: f64);
    #[wasm_bindgen(method, getter = "c")]
    fn c_shim(this: &DomMatrixInit) -> f64;
    #[wasm_bindgen(method, setter = "c")]
    fn set_c_shim(this: &DomMatrixInit, val: f64);
    #[wasm_bindgen(method, getter = "d")]
    fn d_shim(this: &DomMatrixInit) -> f64;
    #[wasm_bindgen(method, setter = "d")]
    fn set_d_shim(this: &DomMatrixInit, val: f64);
    #[wasm_bindgen(method, getter = "e")]
    fn e_shim(this: &DomMatrixInit) -> f64;
    #[wasm_bindgen(method, setter = "e")]
    fn set_e_shim(this: &DomMatrixInit, val: f64);
    #[wasm_bindgen(method, getter = "f")]
    fn f_shim(this: &DomMatrixInit) -> f64;
    #[wasm_bindgen(method, setter = "f")]
    fn set_f_shim(this: &DomMatrixInit, val: f64);
    #[wasm_bindgen(method, getter = "m11")]
    fn m11_shim(this: &DomMatrixInit) -> f64;
    #[wasm_bindgen(method, setter = "m11")]
    fn set_m11_shim(this: &DomMatrixInit, val: f64);
    #[wasm_bindgen(method, getter = "m12")]
    fn m12_shim(this: &DomMatrixInit) -> f64;
    #[wasm_bindgen(method, setter = "m12")]
    fn set_m12_shim(this: &DomMatrixInit, val: f64);
    #[wasm_bindgen(method, getter = "m21")]
    fn m21_shim(this: &DomMatrixInit) -> f64;
    #[wasm_bindgen(method, setter = "m21")]
    fn set_m21_shim(this: &DomMatrixInit, val: f64);
    #[wasm_bindgen(method, getter = "m22")]
    fn m22_shim(this: &DomMatrixInit) -> f64;
    #[wasm_bindgen(method, setter = "m22")]
    fn set_m22_shim(this: &DomMatrixInit, val: f64);
    #[wasm_bindgen(method, getter = "m41")]
    fn m41_shim(this: &DomMatrixInit) -> f64;
    #[wasm_bindgen(method, setter = "m41")]
    fn set_m41_shim(this: &DomMatrixInit, val: f64);
    #[wasm_bindgen(method, getter = "m42")]
    fn m42_shim(this: &DomMatrixInit) -> f64;
    #[wasm_bindgen(method, setter = "m42")]
    fn set_m42_shim(this: &DomMatrixInit, val: f64);
    #[wasm_bindgen(method, getter = "is2D")]
    fn is_2d_shim(this: &DomMatrixInit) -> bool;
    #[wasm_bindgen(method, setter = "is2D")]
    fn set_is_2d_shim(this: &DomMatrixInit, val: bool);
    #[wasm_bindgen(method, getter = "m13")]
    fn m13_shim(this: &DomMatrixInit) -> f64;
    #[wasm_bindgen(method, setter = "m13")]
    fn set_m13_shim(this: &DomMatrixInit, val: f64);
    #[wasm_bindgen(method, getter = "m14")]
    fn m14_shim(this: &DomMatrixInit) -> f64;
    #[wasm_bindgen(method, setter = "m14")]
    fn set_m14_shim(this: &DomMatrixInit, val: f64);
    #[wasm_bindgen(method, getter = "m23")]
    fn m23_shim(this: &DomMatrixInit) -> f64;
    #[wasm_bindgen(method, setter = "m23")]
    fn set_m23_shim(this: &DomMatrixInit, val: f64);
    #[wasm_bindgen(method, getter = "m24")]
    fn m24_shim(this: &DomMatrixInit) -> f64;
    #[wasm_bindgen(method, setter = "m24")]
    fn set_m24_shim(this: &DomMatrixInit, val: f64);
    #[wasm_bindgen(method, getter = "m31")]
    fn m31_shim(this: &DomMatrixInit) -> f64;
    #[wasm_bindgen(method, setter = "m31")]
    fn set_m31_shim(this: &DomMatrixInit, val: f64);
    #[wasm_bindgen(method, getter = "m32")]
    fn m32_shim(this: &DomMatrixInit) -> f64;
    #[wasm_bindgen(method, setter = "m32")]
    fn set_m32_shim(this: &DomMatrixInit, val: f64);
    #[wasm_bindgen(method, getter = "m33")]
    fn m33_shim(this: &DomMatrixInit) -> f64;
    #[wasm_bindgen(method, setter = "m33")]
    fn set_m33_shim(this: &DomMatrixInit, val: f64);
    #[wasm_bindgen(method, getter = "m34")]
    fn m34_shim(this: &DomMatrixInit) -> f64;
    #[wasm_bindgen(method, setter = "m34")]
    fn set_m34_shim(this: &DomMatrixInit, val: f64);
    #[wasm_bindgen(method, getter = "m43")]
    fn m43_shim(this: &DomMatrixInit) -> f64;
    #[wasm_bindgen(method, setter = "m43")]
    fn set_m43_shim(this: &DomMatrixInit, val: f64);
    #[wasm_bindgen(method, getter = "m44")]
    fn m44_shim(this: &DomMatrixInit) -> f64;
    #[wasm_bindgen(method, setter = "m44")]
    fn set_m44_shim(this: &DomMatrixInit, val: f64);
}
#[doc = "The trait to access properties on the `DomMatrixInit` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `DomMatrixInit`*"]
pub trait DomMatrixInitGetters {
    #[doc = "Get the `a` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrixInit`*"]
    fn a(&self) -> f64;
    #[doc = "Get the `b` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrixInit`*"]
    fn b(&self) -> f64;
    #[doc = "Get the `c` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrixInit`*"]
    fn c(&self) -> f64;
    #[doc = "Get the `d` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrixInit`*"]
    fn d(&self) -> f64;
    #[doc = "Get the `e` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrixInit`*"]
    fn e(&self) -> f64;
    #[doc = "Get the `f` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrixInit`*"]
    fn f(&self) -> f64;
    #[doc = "Get the `m11` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrixInit`*"]
    fn m11(&self) -> f64;
    #[doc = "Get the `m12` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrixInit`*"]
    fn m12(&self) -> f64;
    #[doc = "Get the `m21` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrixInit`*"]
    fn m21(&self) -> f64;
    #[doc = "Get the `m22` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrixInit`*"]
    fn m22(&self) -> f64;
    #[doc = "Get the `m41` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrixInit`*"]
    fn m41(&self) -> f64;
    #[doc = "Get the `m42` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrixInit`*"]
    fn m42(&self) -> f64;
    #[doc = "Get the `is2D` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrixInit`*"]
    fn is_2d(&self) -> bool;
    #[doc = "Get the `m13` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrixInit`*"]
    fn m13(&self) -> f64;
    #[doc = "Get the `m14` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrixInit`*"]
    fn m14(&self) -> f64;
    #[doc = "Get the `m23` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrixInit`*"]
    fn m23(&self) -> f64;
    #[doc = "Get the `m24` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrixInit`*"]
    fn m24(&self) -> f64;
    #[doc = "Get the `m31` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrixInit`*"]
    fn m31(&self) -> f64;
    #[doc = "Get the `m32` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrixInit`*"]
    fn m32(&self) -> f64;
    #[doc = "Get the `m33` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrixInit`*"]
    fn m33(&self) -> f64;
    #[doc = "Get the `m34` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrixInit`*"]
    fn m34(&self) -> f64;
    #[doc = "Get the `m43` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrixInit`*"]
    fn m43(&self) -> f64;
    #[doc = "Get the `m44` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrixInit`*"]
    fn m44(&self) -> f64;
}
impl DomMatrixInitGetters for DomMatrixInit {
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
    fn is_2d(&self) -> bool {
        self.is_2d_shim()
    }
    fn m13(&self) -> f64 {
        self.m13_shim()
    }
    fn m14(&self) -> f64 {
        self.m14_shim()
    }
    fn m23(&self) -> f64 {
        self.m23_shim()
    }
    fn m24(&self) -> f64 {
        self.m24_shim()
    }
    fn m31(&self) -> f64 {
        self.m31_shim()
    }
    fn m32(&self) -> f64 {
        self.m32_shim()
    }
    fn m33(&self) -> f64 {
        self.m33_shim()
    }
    fn m34(&self) -> f64 {
        self.m34_shim()
    }
    fn m43(&self) -> f64 {
        self.m43_shim()
    }
    fn m44(&self) -> f64 {
        self.m44_shim()
    }
}
impl DomMatrixInit {
    #[doc = "Construct a new `DomMatrixInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrixInit`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `a` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrixInit`*"]
    pub fn a(&mut self, val: f64) -> &mut Self {
        self.set_a_shim(val);
        self
    }
    #[doc = "Change the `b` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrixInit`*"]
    pub fn b(&mut self, val: f64) -> &mut Self {
        self.set_b_shim(val);
        self
    }
    #[doc = "Change the `c` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrixInit`*"]
    pub fn c(&mut self, val: f64) -> &mut Self {
        self.set_c_shim(val);
        self
    }
    #[doc = "Change the `d` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrixInit`*"]
    pub fn d(&mut self, val: f64) -> &mut Self {
        self.set_d_shim(val);
        self
    }
    #[doc = "Change the `e` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrixInit`*"]
    pub fn e(&mut self, val: f64) -> &mut Self {
        self.set_e_shim(val);
        self
    }
    #[doc = "Change the `f` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrixInit`*"]
    pub fn f(&mut self, val: f64) -> &mut Self {
        self.set_f_shim(val);
        self
    }
    #[doc = "Change the `m11` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrixInit`*"]
    pub fn m11(&mut self, val: f64) -> &mut Self {
        self.set_m11_shim(val);
        self
    }
    #[doc = "Change the `m12` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrixInit`*"]
    pub fn m12(&mut self, val: f64) -> &mut Self {
        self.set_m12_shim(val);
        self
    }
    #[doc = "Change the `m21` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrixInit`*"]
    pub fn m21(&mut self, val: f64) -> &mut Self {
        self.set_m21_shim(val);
        self
    }
    #[doc = "Change the `m22` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrixInit`*"]
    pub fn m22(&mut self, val: f64) -> &mut Self {
        self.set_m22_shim(val);
        self
    }
    #[doc = "Change the `m41` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrixInit`*"]
    pub fn m41(&mut self, val: f64) -> &mut Self {
        self.set_m41_shim(val);
        self
    }
    #[doc = "Change the `m42` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrixInit`*"]
    pub fn m42(&mut self, val: f64) -> &mut Self {
        self.set_m42_shim(val);
        self
    }
    #[doc = "Change the `is2D` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrixInit`*"]
    pub fn is_2d(&mut self, val: bool) -> &mut Self {
        self.set_is_2d_shim(val);
        self
    }
    #[doc = "Change the `m13` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrixInit`*"]
    pub fn m13(&mut self, val: f64) -> &mut Self {
        self.set_m13_shim(val);
        self
    }
    #[doc = "Change the `m14` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrixInit`*"]
    pub fn m14(&mut self, val: f64) -> &mut Self {
        self.set_m14_shim(val);
        self
    }
    #[doc = "Change the `m23` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrixInit`*"]
    pub fn m23(&mut self, val: f64) -> &mut Self {
        self.set_m23_shim(val);
        self
    }
    #[doc = "Change the `m24` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrixInit`*"]
    pub fn m24(&mut self, val: f64) -> &mut Self {
        self.set_m24_shim(val);
        self
    }
    #[doc = "Change the `m31` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrixInit`*"]
    pub fn m31(&mut self, val: f64) -> &mut Self {
        self.set_m31_shim(val);
        self
    }
    #[doc = "Change the `m32` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrixInit`*"]
    pub fn m32(&mut self, val: f64) -> &mut Self {
        self.set_m32_shim(val);
        self
    }
    #[doc = "Change the `m33` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrixInit`*"]
    pub fn m33(&mut self, val: f64) -> &mut Self {
        self.set_m33_shim(val);
        self
    }
    #[doc = "Change the `m34` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrixInit`*"]
    pub fn m34(&mut self, val: f64) -> &mut Self {
        self.set_m34_shim(val);
        self
    }
    #[doc = "Change the `m43` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrixInit`*"]
    pub fn m43(&mut self, val: f64) -> &mut Self {
        self.set_m43_shim(val);
        self
    }
    #[doc = "Change the `m44` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrixInit`*"]
    pub fn m44(&mut self, val: f64) -> &mut Self {
        self.set_m44_shim(val);
        self
    }
}
impl Default for DomMatrixInit {
    fn default() -> Self {
        Self::new()
    }
}
