#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = DeviceAccelerationInit)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DeviceAccelerationInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceAccelerationInit`*"]
    pub type DeviceAccelerationInit;
    #[wasm_bindgen(method, getter = "x")]
    fn x_shim(this: &DeviceAccelerationInit) -> Option<f64>;
    #[wasm_bindgen(method, setter = "x")]
    fn set_x_shim(this: &DeviceAccelerationInit, val: Option<f64>);
    #[wasm_bindgen(method, getter = "y")]
    fn y_shim(this: &DeviceAccelerationInit) -> Option<f64>;
    #[wasm_bindgen(method, setter = "y")]
    fn set_y_shim(this: &DeviceAccelerationInit, val: Option<f64>);
    #[wasm_bindgen(method, getter = "z")]
    fn z_shim(this: &DeviceAccelerationInit) -> Option<f64>;
    #[wasm_bindgen(method, setter = "z")]
    fn set_z_shim(this: &DeviceAccelerationInit, val: Option<f64>);
}
#[doc = "The trait to access properties on the `DeviceAccelerationInit` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `DeviceAccelerationInit`*"]
pub trait DeviceAccelerationInitGetters {
    #[doc = "Get the `x` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceAccelerationInit`*"]
    fn x(&self) -> Option<f64>;
    #[doc = "Get the `y` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceAccelerationInit`*"]
    fn y(&self) -> Option<f64>;
    #[doc = "Get the `z` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceAccelerationInit`*"]
    fn z(&self) -> Option<f64>;
}
impl DeviceAccelerationInitGetters for DeviceAccelerationInit {
    fn x(&self) -> Option<f64> {
        self.x_shim()
    }
    fn y(&self) -> Option<f64> {
        self.y_shim()
    }
    fn z(&self) -> Option<f64> {
        self.z_shim()
    }
}
impl DeviceAccelerationInit {
    #[doc = "Construct a new `DeviceAccelerationInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceAccelerationInit`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `x` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceAccelerationInit`*"]
    pub fn x(&mut self, val: Option<f64>) -> &mut Self {
        self.set_x_shim(val);
        self
    }
    #[doc = "Change the `y` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceAccelerationInit`*"]
    pub fn y(&mut self, val: Option<f64>) -> &mut Self {
        self.set_y_shim(val);
        self
    }
    #[doc = "Change the `z` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceAccelerationInit`*"]
    pub fn z(&mut self, val: Option<f64>) -> &mut Self {
        self.set_z_shim(val);
        self
    }
}
impl Default for DeviceAccelerationInit {
    fn default() -> Self {
        Self::new()
    }
}
