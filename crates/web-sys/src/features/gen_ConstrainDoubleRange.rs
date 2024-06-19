#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = ConstrainDoubleRange)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ConstrainDoubleRange` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstrainDoubleRange`*"]
    pub type ConstrainDoubleRange;
    #[doc = "Get the `exact` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstrainDoubleRange`*"]
    #[wasm_bindgen(method, getter = "exact")]
    pub fn get_exact(this: &ConstrainDoubleRange) -> Option<f64>;
    #[wasm_bindgen(method, setter = "exact")]
    fn set_exact(this: &ConstrainDoubleRange, val: f64);
    #[doc = "Get the `ideal` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstrainDoubleRange`*"]
    #[wasm_bindgen(method, getter = "ideal")]
    pub fn get_ideal(this: &ConstrainDoubleRange) -> Option<f64>;
    #[wasm_bindgen(method, setter = "ideal")]
    fn set_ideal(this: &ConstrainDoubleRange, val: f64);
    #[doc = "Get the `max` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstrainDoubleRange`*"]
    #[wasm_bindgen(method, getter = "max")]
    pub fn get_max(this: &ConstrainDoubleRange) -> Option<f64>;
    #[wasm_bindgen(method, setter = "max")]
    fn set_max(this: &ConstrainDoubleRange, val: f64);
    #[doc = "Get the `min` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstrainDoubleRange`*"]
    #[wasm_bindgen(method, getter = "min")]
    pub fn get_min(this: &ConstrainDoubleRange) -> Option<f64>;
    #[wasm_bindgen(method, setter = "min")]
    fn set_min(this: &ConstrainDoubleRange, val: f64);
}
impl ConstrainDoubleRange {
    #[doc = "Construct a new `ConstrainDoubleRange`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstrainDoubleRange`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `exact` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstrainDoubleRange`*"]
    pub fn exact(&mut self, val: f64) -> &mut Self {
        self.set_exact(val);
        self
    }
    #[doc = "Change the `ideal` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstrainDoubleRange`*"]
    pub fn ideal(&mut self, val: f64) -> &mut Self {
        self.set_ideal(val);
        self
    }
    #[doc = "Change the `max` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstrainDoubleRange`*"]
    pub fn max(&mut self, val: f64) -> &mut Self {
        self.set_max(val);
        self
    }
    #[doc = "Change the `min` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstrainDoubleRange`*"]
    pub fn min(&mut self, val: f64) -> &mut Self {
        self.set_min(val);
        self
    }
}
impl Default for ConstrainDoubleRange {
    fn default() -> Self {
        Self::new()
    }
}
