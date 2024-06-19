#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = DeviceProximityEventInit)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DeviceProximityEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceProximityEventInit`*"]
    pub type DeviceProximityEventInit;
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceProximityEventInit`*"]
    #[wasm_bindgen(method, getter = "bubbles")]
    pub fn get_bubbles(this: &DeviceProximityEventInit) -> Option<bool>;
    #[wasm_bindgen(method, setter = "bubbles")]
    fn set_bubbles(this: &DeviceProximityEventInit, val: bool);
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceProximityEventInit`*"]
    #[wasm_bindgen(method, getter = "cancelable")]
    pub fn get_cancelable(this: &DeviceProximityEventInit) -> Option<bool>;
    #[wasm_bindgen(method, setter = "cancelable")]
    fn set_cancelable(this: &DeviceProximityEventInit, val: bool);
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceProximityEventInit`*"]
    #[wasm_bindgen(method, getter = "composed")]
    pub fn get_composed(this: &DeviceProximityEventInit) -> Option<bool>;
    #[wasm_bindgen(method, setter = "composed")]
    fn set_composed(this: &DeviceProximityEventInit, val: bool);
    #[doc = "Get the `max` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceProximityEventInit`*"]
    #[wasm_bindgen(method, getter = "max")]
    pub fn get_max(this: &DeviceProximityEventInit) -> Option<f64>;
    #[wasm_bindgen(method, setter = "max")]
    fn set_max(this: &DeviceProximityEventInit, val: f64);
    #[doc = "Get the `min` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceProximityEventInit`*"]
    #[wasm_bindgen(method, getter = "min")]
    pub fn get_min(this: &DeviceProximityEventInit) -> Option<f64>;
    #[wasm_bindgen(method, setter = "min")]
    fn set_min(this: &DeviceProximityEventInit, val: f64);
    #[doc = "Get the `value` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceProximityEventInit`*"]
    #[wasm_bindgen(method, getter = "value")]
    pub fn get_value(this: &DeviceProximityEventInit) -> Option<f64>;
    #[wasm_bindgen(method, setter = "value")]
    fn set_value(this: &DeviceProximityEventInit, val: f64);
}
impl DeviceProximityEventInit {
    #[doc = "Construct a new `DeviceProximityEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceProximityEventInit`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceProximityEventInit`*"]
    pub fn bubbles(&mut self, val: bool) -> &mut Self {
        self.set_bubbles(val);
        self
    }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceProximityEventInit`*"]
    pub fn cancelable(&mut self, val: bool) -> &mut Self {
        self.set_cancelable(val);
        self
    }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceProximityEventInit`*"]
    pub fn composed(&mut self, val: bool) -> &mut Self {
        self.set_composed(val);
        self
    }
    #[doc = "Change the `max` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceProximityEventInit`*"]
    pub fn max(&mut self, val: f64) -> &mut Self {
        self.set_max(val);
        self
    }
    #[doc = "Change the `min` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceProximityEventInit`*"]
    pub fn min(&mut self, val: f64) -> &mut Self {
        self.set_min(val);
        self
    }
    #[doc = "Change the `value` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceProximityEventInit`*"]
    pub fn value(&mut self, val: f64) -> &mut Self {
        self.set_value(val);
        self
    }
}
impl Default for DeviceProximityEventInit {
    fn default() -> Self {
        Self::new()
    }
}
