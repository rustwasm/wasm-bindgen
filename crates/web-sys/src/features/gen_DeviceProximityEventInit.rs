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
    #[wasm_bindgen(method, getter = "bubbles")]
    fn bubbles_shim(this: &DeviceProximityEventInit) -> bool;
    #[wasm_bindgen(method, setter = "bubbles")]
    fn set_bubbles_shim(this: &DeviceProximityEventInit, val: bool);
    #[wasm_bindgen(method, getter = "cancelable")]
    fn cancelable_shim(this: &DeviceProximityEventInit) -> bool;
    #[wasm_bindgen(method, setter = "cancelable")]
    fn set_cancelable_shim(this: &DeviceProximityEventInit, val: bool);
    #[wasm_bindgen(method, getter = "composed")]
    fn composed_shim(this: &DeviceProximityEventInit) -> bool;
    #[wasm_bindgen(method, setter = "composed")]
    fn set_composed_shim(this: &DeviceProximityEventInit, val: bool);
    #[wasm_bindgen(method, getter = "max")]
    fn max_shim(this: &DeviceProximityEventInit) -> f64;
    #[wasm_bindgen(method, setter = "max")]
    fn set_max_shim(this: &DeviceProximityEventInit, val: f64);
    #[wasm_bindgen(method, getter = "min")]
    fn min_shim(this: &DeviceProximityEventInit) -> f64;
    #[wasm_bindgen(method, setter = "min")]
    fn set_min_shim(this: &DeviceProximityEventInit, val: f64);
    #[wasm_bindgen(method, getter = "value")]
    fn value_shim(this: &DeviceProximityEventInit) -> f64;
    #[wasm_bindgen(method, setter = "value")]
    fn set_value_shim(this: &DeviceProximityEventInit, val: f64);
}
#[doc = "The trait to access properties on the `DeviceProximityEventInit` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `DeviceProximityEventInit`*"]
pub trait DeviceProximityEventInitGetters {
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceProximityEventInit`*"]
    fn bubbles(&self) -> bool;
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceProximityEventInit`*"]
    fn cancelable(&self) -> bool;
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceProximityEventInit`*"]
    fn composed(&self) -> bool;
    #[doc = "Get the `max` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceProximityEventInit`*"]
    fn max(&self) -> f64;
    #[doc = "Get the `min` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceProximityEventInit`*"]
    fn min(&self) -> f64;
    #[doc = "Get the `value` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceProximityEventInit`*"]
    fn value(&self) -> f64;
}
impl DeviceProximityEventInitGetters for DeviceProximityEventInit {
    fn bubbles(&self) -> bool {
        self.bubbles_shim()
    }
    fn cancelable(&self) -> bool {
        self.cancelable_shim()
    }
    fn composed(&self) -> bool {
        self.composed_shim()
    }
    fn max(&self) -> f64 {
        self.max_shim()
    }
    fn min(&self) -> f64 {
        self.min_shim()
    }
    fn value(&self) -> f64 {
        self.value_shim()
    }
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
        self.set_bubbles_shim(val);
        self
    }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceProximityEventInit`*"]
    pub fn cancelable(&mut self, val: bool) -> &mut Self {
        self.set_cancelable_shim(val);
        self
    }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceProximityEventInit`*"]
    pub fn composed(&mut self, val: bool) -> &mut Self {
        self.set_composed_shim(val);
        self
    }
    #[doc = "Change the `max` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceProximityEventInit`*"]
    pub fn max(&mut self, val: f64) -> &mut Self {
        self.set_max_shim(val);
        self
    }
    #[doc = "Change the `min` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceProximityEventInit`*"]
    pub fn min(&mut self, val: f64) -> &mut Self {
        self.set_min_shim(val);
        self
    }
    #[doc = "Change the `value` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceProximityEventInit`*"]
    pub fn value(&mut self, val: f64) -> &mut Self {
        self.set_value_shim(val);
        self
    }
}
impl Default for DeviceProximityEventInit {
    fn default() -> Self {
        Self::new()
    }
}
