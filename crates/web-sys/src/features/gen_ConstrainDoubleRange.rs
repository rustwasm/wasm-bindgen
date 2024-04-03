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
    #[wasm_bindgen(method, setter = "exact")]
    fn exact_shim(this: &ConstrainDoubleRange, val: f64);
    #[wasm_bindgen(method, setter = "ideal")]
    fn ideal_shim(this: &ConstrainDoubleRange, val: f64);
    #[wasm_bindgen(method, setter = "max")]
    fn max_shim(this: &ConstrainDoubleRange, val: f64);
    #[wasm_bindgen(method, setter = "min")]
    fn min_shim(this: &ConstrainDoubleRange, val: f64);
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
        self.exact_shim(val);
        self
    }
    #[doc = "Change the `ideal` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstrainDoubleRange`*"]
    pub fn ideal(&mut self, val: f64) -> &mut Self {
        self.ideal_shim(val);
        self
    }
    #[doc = "Change the `max` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstrainDoubleRange`*"]
    pub fn max(&mut self, val: f64) -> &mut Self {
        self.max_shim(val);
        self
    }
    #[doc = "Change the `min` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstrainDoubleRange`*"]
    pub fn min(&mut self, val: f64) -> &mut Self {
        self.min_shim(val);
        self
    }
}
impl Default for ConstrainDoubleRange {
    fn default() -> Self {
        Self::new()
    }
}
