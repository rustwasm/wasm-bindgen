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
    #[wasm_bindgen(method, getter = "day")]
    fn day_shim(this: &DateTimeValue) -> i32;
    #[wasm_bindgen(method, setter = "day")]
    fn set_day_shim(this: &DateTimeValue, val: i32);
    #[wasm_bindgen(method, getter = "hour")]
    fn hour_shim(this: &DateTimeValue) -> i32;
    #[wasm_bindgen(method, setter = "hour")]
    fn set_hour_shim(this: &DateTimeValue, val: i32);
    #[wasm_bindgen(method, getter = "minute")]
    fn minute_shim(this: &DateTimeValue) -> i32;
    #[wasm_bindgen(method, setter = "minute")]
    fn set_minute_shim(this: &DateTimeValue, val: i32);
    #[wasm_bindgen(method, getter = "month")]
    fn month_shim(this: &DateTimeValue) -> i32;
    #[wasm_bindgen(method, setter = "month")]
    fn set_month_shim(this: &DateTimeValue, val: i32);
    #[wasm_bindgen(method, getter = "year")]
    fn year_shim(this: &DateTimeValue) -> i32;
    #[wasm_bindgen(method, setter = "year")]
    fn set_year_shim(this: &DateTimeValue, val: i32);
}
#[doc = "The trait to access properties on the `DateTimeValue` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `DateTimeValue`*"]
pub trait DateTimeValueGetters {
    #[doc = "Get the `day` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DateTimeValue`*"]
    fn day(&self) -> i32;
    #[doc = "Get the `hour` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DateTimeValue`*"]
    fn hour(&self) -> i32;
    #[doc = "Get the `minute` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DateTimeValue`*"]
    fn minute(&self) -> i32;
    #[doc = "Get the `month` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DateTimeValue`*"]
    fn month(&self) -> i32;
    #[doc = "Get the `year` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DateTimeValue`*"]
    fn year(&self) -> i32;
}
impl DateTimeValueGetters for DateTimeValue {
    fn day(&self) -> i32 {
        self.day_shim()
    }
    fn hour(&self) -> i32 {
        self.hour_shim()
    }
    fn minute(&self) -> i32 {
        self.minute_shim()
    }
    fn month(&self) -> i32 {
        self.month_shim()
    }
    fn year(&self) -> i32 {
        self.year_shim()
    }
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
        self.set_day_shim(val);
        self
    }
    #[doc = "Change the `hour` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DateTimeValue`*"]
    pub fn hour(&mut self, val: i32) -> &mut Self {
        self.set_hour_shim(val);
        self
    }
    #[doc = "Change the `minute` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DateTimeValue`*"]
    pub fn minute(&mut self, val: i32) -> &mut Self {
        self.set_minute_shim(val);
        self
    }
    #[doc = "Change the `month` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DateTimeValue`*"]
    pub fn month(&mut self, val: i32) -> &mut Self {
        self.set_month_shim(val);
        self
    }
    #[doc = "Change the `year` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DateTimeValue`*"]
    pub fn year(&mut self, val: i32) -> &mut Self {
        self.set_year_shim(val);
        self
    }
}
impl Default for DateTimeValue {
    fn default() -> Self {
        Self::new()
    }
}
