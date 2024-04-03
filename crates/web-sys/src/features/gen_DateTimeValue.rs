#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = DateTimeValue)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DateTimeValue` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DateTimeValue`*"]
    pub type DateTimeValue;
    #[wasm_bindgen(method, setter = "day")]
    fn day_shim(this: &DateTimeValue, val: i32);
    #[wasm_bindgen(method, setter = "hour")]
    fn hour_shim(this: &DateTimeValue, val: i32);
    #[wasm_bindgen(method, setter = "minute")]
    fn minute_shim(this: &DateTimeValue, val: i32);
    #[wasm_bindgen(method, setter = "month")]
    fn month_shim(this: &DateTimeValue, val: i32);
    #[wasm_bindgen(method, setter = "year")]
    fn year_shim(this: &DateTimeValue, val: i32);
}
impl DateTimeValue {
    #[doc = "Construct a new `DateTimeValue`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DateTimeValue`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `day` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DateTimeValue`*"]
    pub fn day(&mut self, val: i32) -> &mut Self {
        self.day_shim(val);
        self
    }
    #[doc = "Change the `hour` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DateTimeValue`*"]
    pub fn hour(&mut self, val: i32) -> &mut Self {
        self.hour_shim(val);
        self
    }
    #[doc = "Change the `minute` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DateTimeValue`*"]
    pub fn minute(&mut self, val: i32) -> &mut Self {
        self.minute_shim(val);
        self
    }
    #[doc = "Change the `month` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DateTimeValue`*"]
    pub fn month(&mut self, val: i32) -> &mut Self {
        self.month_shim(val);
        self
    }
    #[doc = "Change the `year` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DateTimeValue`*"]
    pub fn year(&mut self, val: i32) -> &mut Self {
        self.year_shim(val);
        self
    }
}
impl Default for DateTimeValue {
    fn default() -> Self {
        Self::new()
    }
}
