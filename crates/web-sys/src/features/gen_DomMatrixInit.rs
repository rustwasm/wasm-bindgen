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
    #[wasm_bindgen(method, setter = "a")]
    fn a_shim(this: &DomMatrixInit, val: f64);
    #[wasm_bindgen(method, setter = "b")]
    fn b_shim(this: &DomMatrixInit, val: f64);
    #[wasm_bindgen(method, setter = "c")]
    fn c_shim(this: &DomMatrixInit, val: f64);
    #[wasm_bindgen(method, setter = "d")]
    fn d_shim(this: &DomMatrixInit, val: f64);
    #[wasm_bindgen(method, setter = "e")]
    fn e_shim(this: &DomMatrixInit, val: f64);
    #[wasm_bindgen(method, setter = "f")]
    fn f_shim(this: &DomMatrixInit, val: f64);
    #[wasm_bindgen(method, setter = "m11")]
    fn m11_shim(this: &DomMatrixInit, val: f64);
    #[wasm_bindgen(method, setter = "m12")]
    fn m12_shim(this: &DomMatrixInit, val: f64);
    #[wasm_bindgen(method, setter = "m21")]
    fn m21_shim(this: &DomMatrixInit, val: f64);
    #[wasm_bindgen(method, setter = "m22")]
    fn m22_shim(this: &DomMatrixInit, val: f64);
    #[wasm_bindgen(method, setter = "m41")]
    fn m41_shim(this: &DomMatrixInit, val: f64);
    #[wasm_bindgen(method, setter = "m42")]
    fn m42_shim(this: &DomMatrixInit, val: f64);
    #[wasm_bindgen(method, setter = "is2D")]
    fn is_2d_shim(this: &DomMatrixInit, val: bool);
    #[wasm_bindgen(method, setter = "m13")]
    fn m13_shim(this: &DomMatrixInit, val: f64);
    #[wasm_bindgen(method, setter = "m14")]
    fn m14_shim(this: &DomMatrixInit, val: f64);
    #[wasm_bindgen(method, setter = "m23")]
    fn m23_shim(this: &DomMatrixInit, val: f64);
    #[wasm_bindgen(method, setter = "m24")]
    fn m24_shim(this: &DomMatrixInit, val: f64);
    #[wasm_bindgen(method, setter = "m31")]
    fn m31_shim(this: &DomMatrixInit, val: f64);
    #[wasm_bindgen(method, setter = "m32")]
    fn m32_shim(this: &DomMatrixInit, val: f64);
    #[wasm_bindgen(method, setter = "m33")]
    fn m33_shim(this: &DomMatrixInit, val: f64);
    #[wasm_bindgen(method, setter = "m34")]
    fn m34_shim(this: &DomMatrixInit, val: f64);
    #[wasm_bindgen(method, setter = "m43")]
    fn m43_shim(this: &DomMatrixInit, val: f64);
    #[wasm_bindgen(method, setter = "m44")]
    fn m44_shim(this: &DomMatrixInit, val: f64);
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
        self.a_shim(val);
        self
    }
    #[doc = "Change the `b` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrixInit`*"]
    pub fn b(&mut self, val: f64) -> &mut Self {
        self.b_shim(val);
        self
    }
    #[doc = "Change the `c` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrixInit`*"]
    pub fn c(&mut self, val: f64) -> &mut Self {
        self.c_shim(val);
        self
    }
    #[doc = "Change the `d` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrixInit`*"]
    pub fn d(&mut self, val: f64) -> &mut Self {
        self.d_shim(val);
        self
    }
    #[doc = "Change the `e` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrixInit`*"]
    pub fn e(&mut self, val: f64) -> &mut Self {
        self.e_shim(val);
        self
    }
    #[doc = "Change the `f` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrixInit`*"]
    pub fn f(&mut self, val: f64) -> &mut Self {
        self.f_shim(val);
        self
    }
    #[doc = "Change the `m11` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrixInit`*"]
    pub fn m11(&mut self, val: f64) -> &mut Self {
        self.m11_shim(val);
        self
    }
    #[doc = "Change the `m12` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrixInit`*"]
    pub fn m12(&mut self, val: f64) -> &mut Self {
        self.m12_shim(val);
        self
    }
    #[doc = "Change the `m21` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrixInit`*"]
    pub fn m21(&mut self, val: f64) -> &mut Self {
        self.m21_shim(val);
        self
    }
    #[doc = "Change the `m22` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrixInit`*"]
    pub fn m22(&mut self, val: f64) -> &mut Self {
        self.m22_shim(val);
        self
    }
    #[doc = "Change the `m41` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrixInit`*"]
    pub fn m41(&mut self, val: f64) -> &mut Self {
        self.m41_shim(val);
        self
    }
    #[doc = "Change the `m42` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrixInit`*"]
    pub fn m42(&mut self, val: f64) -> &mut Self {
        self.m42_shim(val);
        self
    }
    #[doc = "Change the `is2D` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrixInit`*"]
    pub fn is_2d(&mut self, val: bool) -> &mut Self {
        self.is_2d_shim(val);
        self
    }
    #[doc = "Change the `m13` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrixInit`*"]
    pub fn m13(&mut self, val: f64) -> &mut Self {
        self.m13_shim(val);
        self
    }
    #[doc = "Change the `m14` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrixInit`*"]
    pub fn m14(&mut self, val: f64) -> &mut Self {
        self.m14_shim(val);
        self
    }
    #[doc = "Change the `m23` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrixInit`*"]
    pub fn m23(&mut self, val: f64) -> &mut Self {
        self.m23_shim(val);
        self
    }
    #[doc = "Change the `m24` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrixInit`*"]
    pub fn m24(&mut self, val: f64) -> &mut Self {
        self.m24_shim(val);
        self
    }
    #[doc = "Change the `m31` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrixInit`*"]
    pub fn m31(&mut self, val: f64) -> &mut Self {
        self.m31_shim(val);
        self
    }
    #[doc = "Change the `m32` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrixInit`*"]
    pub fn m32(&mut self, val: f64) -> &mut Self {
        self.m32_shim(val);
        self
    }
    #[doc = "Change the `m33` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrixInit`*"]
    pub fn m33(&mut self, val: f64) -> &mut Self {
        self.m33_shim(val);
        self
    }
    #[doc = "Change the `m34` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrixInit`*"]
    pub fn m34(&mut self, val: f64) -> &mut Self {
        self.m34_shim(val);
        self
    }
    #[doc = "Change the `m43` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrixInit`*"]
    pub fn m43(&mut self, val: f64) -> &mut Self {
        self.m43_shim(val);
        self
    }
    #[doc = "Change the `m44` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrixInit`*"]
    pub fn m44(&mut self, val: f64) -> &mut Self {
        self.m44_shim(val);
        self
    }
}
impl Default for DomMatrixInit {
    fn default() -> Self {
        Self::new()
    }
}
