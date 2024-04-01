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
    #[wasm_bindgen(method, setter = "bubbles")]
    fn bubbles_shim(this: &DeviceProximityEventInit, val: bool);
    #[wasm_bindgen(method, setter = "cancelable")]
    fn cancelable_shim(this: &DeviceProximityEventInit, val: bool);
    #[wasm_bindgen(method, setter = "composed")]
    fn composed_shim(this: &DeviceProximityEventInit, val: bool);
    #[wasm_bindgen(method, setter = "max")]
    fn max_shim(this: &DeviceProximityEventInit, val: f64);
    #[wasm_bindgen(method, setter = "min")]
    fn min_shim(this: &DeviceProximityEventInit, val: f64);
    #[wasm_bindgen(method, setter = "value")]
    fn value_shim(this: &DeviceProximityEventInit, val: f64);
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
        self.bubbles_shim(val);
        self
    }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceProximityEventInit`*"]
    pub fn cancelable(&mut self, val: bool) -> &mut Self {
        self.cancelable_shim(val);
        self
    }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceProximityEventInit`*"]
    pub fn composed(&mut self, val: bool) -> &mut Self {
        self.composed_shim(val);
        self
    }
    #[doc = "Change the `max` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceProximityEventInit`*"]
    pub fn max(&mut self, val: f64) -> &mut Self {
        self.max_shim(val);
        self
    }
    #[doc = "Change the `min` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceProximityEventInit`*"]
    pub fn min(&mut self, val: f64) -> &mut Self {
        self.min_shim(val);
        self
    }
    #[doc = "Change the `value` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceProximityEventInit`*"]
    pub fn value(&mut self, val: f64) -> &mut Self {
        self.value_shim(val);
        self
    }
}
impl Default for DeviceProximityEventInit {
    fn default() -> Self {
        Self::new()
    }
}
