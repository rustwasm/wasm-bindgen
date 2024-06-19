#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = ConstrainLongRange)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ConstrainLongRange` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstrainLongRange`*"]
    pub type ConstrainLongRange;
    #[doc = "Get the `exact` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstrainLongRange`*"]
    #[wasm_bindgen(method, getter = "exact")]
    pub fn get_exact(this: &ConstrainLongRange) -> Option<i32>;
    #[wasm_bindgen(method, setter = "exact")]
    fn set_exact(this: &ConstrainLongRange, val: i32);
    #[doc = "Get the `ideal` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstrainLongRange`*"]
    #[wasm_bindgen(method, getter = "ideal")]
    pub fn get_ideal(this: &ConstrainLongRange) -> Option<i32>;
    #[wasm_bindgen(method, setter = "ideal")]
    fn set_ideal(this: &ConstrainLongRange, val: i32);
    #[doc = "Get the `max` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstrainLongRange`*"]
    #[wasm_bindgen(method, getter = "max")]
    pub fn get_max(this: &ConstrainLongRange) -> Option<i32>;
    #[wasm_bindgen(method, setter = "max")]
    fn set_max(this: &ConstrainLongRange, val: i32);
    #[doc = "Get the `min` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstrainLongRange`*"]
    #[wasm_bindgen(method, getter = "min")]
    pub fn get_min(this: &ConstrainLongRange) -> Option<i32>;
    #[wasm_bindgen(method, setter = "min")]
    fn set_min(this: &ConstrainLongRange, val: i32);
}
impl ConstrainLongRange {
    #[doc = "Construct a new `ConstrainLongRange`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstrainLongRange`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `exact` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstrainLongRange`*"]
    pub fn exact(&mut self, val: i32) -> &mut Self {
        self.set_exact(val);
        self
    }
    #[doc = "Change the `ideal` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstrainLongRange`*"]
    pub fn ideal(&mut self, val: i32) -> &mut Self {
        self.set_ideal(val);
        self
    }
    #[doc = "Change the `max` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstrainLongRange`*"]
    pub fn max(&mut self, val: i32) -> &mut Self {
        self.set_max(val);
        self
    }
    #[doc = "Change the `min` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstrainLongRange`*"]
    pub fn min(&mut self, val: i32) -> &mut Self {
        self.set_min(val);
        self
    }
}
impl Default for ConstrainLongRange {
    fn default() -> Self {
        Self::new()
    }
}
