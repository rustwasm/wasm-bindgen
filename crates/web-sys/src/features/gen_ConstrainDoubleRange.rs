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
    #[wasm_bindgen(method, getter = "exact")]
    fn exact_shim(this: &ConstrainDoubleRange) -> f64;
    #[wasm_bindgen(method, setter = "exact")]
    fn set_exact_shim(this: &ConstrainDoubleRange, val: f64);
    #[wasm_bindgen(method, getter = "ideal")]
    fn ideal_shim(this: &ConstrainDoubleRange) -> f64;
    #[wasm_bindgen(method, setter = "ideal")]
    fn set_ideal_shim(this: &ConstrainDoubleRange, val: f64);
    #[wasm_bindgen(method, getter = "max")]
    fn max_shim(this: &ConstrainDoubleRange) -> f64;
    #[wasm_bindgen(method, setter = "max")]
    fn set_max_shim(this: &ConstrainDoubleRange, val: f64);
    #[wasm_bindgen(method, getter = "min")]
    fn min_shim(this: &ConstrainDoubleRange) -> f64;
    #[wasm_bindgen(method, setter = "min")]
    fn set_min_shim(this: &ConstrainDoubleRange, val: f64);
}
#[doc = "The trait to access properties on the `ConstrainDoubleRange` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `ConstrainDoubleRange`*"]
pub trait ConstrainDoubleRangeGetters {
    #[doc = "Get the `exact` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstrainDoubleRange`*"]
    fn exact(&self) -> f64;
    #[doc = "Get the `ideal` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstrainDoubleRange`*"]
    fn ideal(&self) -> f64;
    #[doc = "Get the `max` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstrainDoubleRange`*"]
    fn max(&self) -> f64;
    #[doc = "Get the `min` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstrainDoubleRange`*"]
    fn min(&self) -> f64;
}
impl ConstrainDoubleRangeGetters for ConstrainDoubleRange {
    fn exact(&self) -> f64 {
        self.exact_shim()
    }
    fn ideal(&self) -> f64 {
        self.ideal_shim()
    }
    fn max(&self) -> f64 {
        self.max_shim()
    }
    fn min(&self) -> f64 {
        self.min_shim()
    }
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
        self.set_exact_shim(val);
        self
    }
    #[doc = "Change the `ideal` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstrainDoubleRange`*"]
    pub fn ideal(&mut self, val: f64) -> &mut Self {
        self.set_ideal_shim(val);
        self
    }
    #[doc = "Change the `max` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstrainDoubleRange`*"]
    pub fn max(&mut self, val: f64) -> &mut Self {
        self.set_max_shim(val);
        self
    }
    #[doc = "Change the `min` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstrainDoubleRange`*"]
    pub fn min(&mut self, val: f64) -> &mut Self {
        self.set_min_shim(val);
        self
    }
}
impl Default for ConstrainDoubleRange {
    fn default() -> Self {
        Self::new()
    }
}
