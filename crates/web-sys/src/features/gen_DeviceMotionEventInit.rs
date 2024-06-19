#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = DeviceMotionEventInit)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DeviceMotionEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceMotionEventInit`*"]
    pub type DeviceMotionEventInit;
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceMotionEventInit`*"]
    #[wasm_bindgen(method, getter = "bubbles")]
    pub fn get_bubbles(this: &DeviceMotionEventInit) -> Option<bool>;
    #[wasm_bindgen(method, setter = "bubbles")]
    fn set_bubbles(this: &DeviceMotionEventInit, val: bool);
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceMotionEventInit`*"]
    #[wasm_bindgen(method, getter = "cancelable")]
    pub fn get_cancelable(this: &DeviceMotionEventInit) -> Option<bool>;
    #[wasm_bindgen(method, setter = "cancelable")]
    fn set_cancelable(this: &DeviceMotionEventInit, val: bool);
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceMotionEventInit`*"]
    #[wasm_bindgen(method, getter = "composed")]
    pub fn get_composed(this: &DeviceMotionEventInit) -> Option<bool>;
    #[wasm_bindgen(method, setter = "composed")]
    fn set_composed(this: &DeviceMotionEventInit, val: bool);
    #[cfg(feature = "DeviceAccelerationInit")]
    #[doc = "Get the `acceleration` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceAccelerationInit`, `DeviceMotionEventInit`*"]
    #[wasm_bindgen(method, getter = "acceleration")]
    pub fn get_acceleration(this: &DeviceMotionEventInit) -> Option<DeviceAccelerationInit>;
    #[cfg(feature = "DeviceAccelerationInit")]
    #[wasm_bindgen(method, setter = "acceleration")]
    fn set_acceleration(this: &DeviceMotionEventInit, val: &DeviceAccelerationInit);
    #[cfg(feature = "DeviceAccelerationInit")]
    #[doc = "Get the `accelerationIncludingGravity` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceAccelerationInit`, `DeviceMotionEventInit`*"]
    #[wasm_bindgen(method, getter = "accelerationIncludingGravity")]
    pub fn get_acceleration_including_gravity(
        this: &DeviceMotionEventInit,
    ) -> Option<DeviceAccelerationInit>;
    #[cfg(feature = "DeviceAccelerationInit")]
    #[wasm_bindgen(method, setter = "accelerationIncludingGravity")]
    fn set_acceleration_including_gravity(
        this: &DeviceMotionEventInit,
        val: &DeviceAccelerationInit,
    );
    #[doc = "Get the `interval` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceMotionEventInit`*"]
    #[wasm_bindgen(method, getter = "interval")]
    pub fn get_interval(this: &DeviceMotionEventInit) -> Option<f64>;
    #[wasm_bindgen(method, setter = "interval")]
    fn set_interval(this: &DeviceMotionEventInit, val: Option<f64>);
    #[cfg(feature = "DeviceRotationRateInit")]
    #[doc = "Get the `rotationRate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceMotionEventInit`, `DeviceRotationRateInit`*"]
    #[wasm_bindgen(method, getter = "rotationRate")]
    pub fn get_rotation_rate(this: &DeviceMotionEventInit) -> Option<DeviceRotationRateInit>;
    #[cfg(feature = "DeviceRotationRateInit")]
    #[wasm_bindgen(method, setter = "rotationRate")]
    fn set_rotation_rate(this: &DeviceMotionEventInit, val: &DeviceRotationRateInit);
}
impl DeviceMotionEventInit {
    #[doc = "Construct a new `DeviceMotionEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceMotionEventInit`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceMotionEventInit`*"]
    pub fn bubbles(&mut self, val: bool) -> &mut Self {
        self.set_bubbles(val);
        self
    }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceMotionEventInit`*"]
    pub fn cancelable(&mut self, val: bool) -> &mut Self {
        self.set_cancelable(val);
        self
    }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceMotionEventInit`*"]
    pub fn composed(&mut self, val: bool) -> &mut Self {
        self.set_composed(val);
        self
    }
    #[cfg(feature = "DeviceAccelerationInit")]
    #[doc = "Change the `acceleration` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceAccelerationInit`, `DeviceMotionEventInit`*"]
    pub fn acceleration(&mut self, val: &DeviceAccelerationInit) -> &mut Self {
        self.set_acceleration(val);
        self
    }
    #[cfg(feature = "DeviceAccelerationInit")]
    #[doc = "Change the `accelerationIncludingGravity` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceAccelerationInit`, `DeviceMotionEventInit`*"]
    pub fn acceleration_including_gravity(&mut self, val: &DeviceAccelerationInit) -> &mut Self {
        self.set_acceleration_including_gravity(val);
        self
    }
    #[doc = "Change the `interval` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceMotionEventInit`*"]
    pub fn interval(&mut self, val: Option<f64>) -> &mut Self {
        self.set_interval(val);
        self
    }
    #[cfg(feature = "DeviceRotationRateInit")]
    #[doc = "Change the `rotationRate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceMotionEventInit`, `DeviceRotationRateInit`*"]
    pub fn rotation_rate(&mut self, val: &DeviceRotationRateInit) -> &mut Self {
        self.set_rotation_rate(val);
        self
    }
}
impl Default for DeviceMotionEventInit {
    fn default() -> Self {
        Self::new()
    }
}
