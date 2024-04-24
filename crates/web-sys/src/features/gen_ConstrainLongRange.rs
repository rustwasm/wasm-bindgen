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
    #[wasm_bindgen(method, getter = "exact")]
    fn exact_shim(this: &ConstrainLongRange) -> i32;
    #[wasm_bindgen(method, setter = "exact")]
    fn set_exact_shim(this: &ConstrainLongRange, val: i32);
    #[wasm_bindgen(method, getter = "ideal")]
    fn ideal_shim(this: &ConstrainLongRange) -> i32;
    #[wasm_bindgen(method, setter = "ideal")]
    fn set_ideal_shim(this: &ConstrainLongRange, val: i32);
    #[wasm_bindgen(method, getter = "max")]
    fn max_shim(this: &ConstrainLongRange) -> i32;
    #[wasm_bindgen(method, setter = "max")]
    fn set_max_shim(this: &ConstrainLongRange, val: i32);
    #[wasm_bindgen(method, getter = "min")]
    fn min_shim(this: &ConstrainLongRange) -> i32;
    #[wasm_bindgen(method, setter = "min")]
    fn set_min_shim(this: &ConstrainLongRange, val: i32);
}
#[doc = "The trait to access properties on the `ConstrainLongRange` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `ConstrainLongRange`*"]
pub trait ConstrainLongRangeGetters {
    #[doc = "Get the `exact` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstrainLongRange`*"]
    fn exact(&self) -> i32;
    #[doc = "Get the `ideal` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstrainLongRange`*"]
    fn ideal(&self) -> i32;
    #[doc = "Get the `max` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstrainLongRange`*"]
    fn max(&self) -> i32;
    #[doc = "Get the `min` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstrainLongRange`*"]
    fn min(&self) -> i32;
}
impl ConstrainLongRangeGetters for ConstrainLongRange {
    fn exact(&self) -> i32 {
        self.exact_shim()
    }
    fn ideal(&self) -> i32 {
        self.ideal_shim()
    }
    fn max(&self) -> i32 {
        self.max_shim()
    }
    fn min(&self) -> i32 {
        self.min_shim()
    }
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
        self.set_exact_shim(val);
        self
    }
    #[doc = "Change the `ideal` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstrainLongRange`*"]
    pub fn ideal(&mut self, val: i32) -> &mut Self {
        self.set_ideal_shim(val);
        self
    }
    #[doc = "Change the `max` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstrainLongRange`*"]
    pub fn max(&mut self, val: i32) -> &mut Self {
        self.set_max_shim(val);
        self
    }
    #[doc = "Change the `min` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstrainLongRange`*"]
    pub fn min(&mut self, val: i32) -> &mut Self {
        self.set_min_shim(val);
        self
    }
}
impl Default for ConstrainLongRange {
    fn default() -> Self {
        Self::new()
    }
}
