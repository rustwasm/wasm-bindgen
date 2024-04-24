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
    #[wasm_bindgen(method, getter = "bubbles")]
    fn bubbles_shim(this: &DeviceMotionEventInit) -> bool;
    #[wasm_bindgen(method, setter = "bubbles")]
    fn set_bubbles_shim(this: &DeviceMotionEventInit, val: bool);
    #[wasm_bindgen(method, getter = "cancelable")]
    fn cancelable_shim(this: &DeviceMotionEventInit) -> bool;
    #[wasm_bindgen(method, setter = "cancelable")]
    fn set_cancelable_shim(this: &DeviceMotionEventInit, val: bool);
    #[wasm_bindgen(method, getter = "composed")]
    fn composed_shim(this: &DeviceMotionEventInit) -> bool;
    #[wasm_bindgen(method, setter = "composed")]
    fn set_composed_shim(this: &DeviceMotionEventInit, val: bool);
    #[cfg(feature = "DeviceAccelerationInit")]
    #[wasm_bindgen(method, getter = "acceleration")]
    fn acceleration_shim(this: &DeviceMotionEventInit) -> &DeviceAccelerationInit;
    #[cfg(feature = "DeviceAccelerationInit")]
    #[wasm_bindgen(method, setter = "acceleration")]
    fn set_acceleration_shim(this: &DeviceMotionEventInit, val: &DeviceAccelerationInit);
    #[cfg(feature = "DeviceAccelerationInit")]
    #[wasm_bindgen(method, getter = "accelerationIncludingGravity")]
    fn acceleration_including_gravity_shim(this: &DeviceMotionEventInit)
        -> &DeviceAccelerationInit;
    #[cfg(feature = "DeviceAccelerationInit")]
    #[wasm_bindgen(method, setter = "accelerationIncludingGravity")]
    fn set_acceleration_including_gravity_shim(
        this: &DeviceMotionEventInit,
        val: &DeviceAccelerationInit,
    );
    #[wasm_bindgen(method, getter = "interval")]
    fn interval_shim(this: &DeviceMotionEventInit) -> Option<f64>;
    #[wasm_bindgen(method, setter = "interval")]
    fn set_interval_shim(this: &DeviceMotionEventInit, val: Option<f64>);
    #[cfg(feature = "DeviceRotationRateInit")]
    #[wasm_bindgen(method, getter = "rotationRate")]
    fn rotation_rate_shim(this: &DeviceMotionEventInit) -> &DeviceRotationRateInit;
    #[cfg(feature = "DeviceRotationRateInit")]
    #[wasm_bindgen(method, setter = "rotationRate")]
    fn set_rotation_rate_shim(this: &DeviceMotionEventInit, val: &DeviceRotationRateInit);
}
#[doc = "The trait to access properties on the `DeviceMotionEventInit` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `DeviceMotionEventInit`*"]
pub trait DeviceMotionEventInitGetters {
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceMotionEventInit`*"]
    fn bubbles(&self) -> bool;
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceMotionEventInit`*"]
    fn cancelable(&self) -> bool;
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceMotionEventInit`*"]
    fn composed(&self) -> bool;
    #[cfg(feature = "DeviceAccelerationInit")]
    #[doc = "Get the `acceleration` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceAccelerationInit`, `DeviceMotionEventInit`*"]
    fn acceleration(&self) -> &DeviceAccelerationInit;
    #[cfg(feature = "DeviceAccelerationInit")]
    #[doc = "Get the `accelerationIncludingGravity` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceAccelerationInit`, `DeviceMotionEventInit`*"]
    fn acceleration_including_gravity(&self) -> &DeviceAccelerationInit;
    #[doc = "Get the `interval` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceMotionEventInit`*"]
    fn interval(&self) -> Option<f64>;
    #[cfg(feature = "DeviceRotationRateInit")]
    #[doc = "Get the `rotationRate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceMotionEventInit`, `DeviceRotationRateInit`*"]
    fn rotation_rate(&self) -> &DeviceRotationRateInit;
}
impl DeviceMotionEventInitGetters for DeviceMotionEventInit {
    fn bubbles(&self) -> bool {
        self.bubbles_shim()
    }
    fn cancelable(&self) -> bool {
        self.cancelable_shim()
    }
    fn composed(&self) -> bool {
        self.composed_shim()
    }
    #[cfg(feature = "DeviceAccelerationInit")]
    fn acceleration(&self) -> &DeviceAccelerationInit {
        self.acceleration_shim()
    }
    #[cfg(feature = "DeviceAccelerationInit")]
    fn acceleration_including_gravity(&self) -> &DeviceAccelerationInit {
        self.acceleration_including_gravity_shim()
    }
    fn interval(&self) -> Option<f64> {
        self.interval_shim()
    }
    #[cfg(feature = "DeviceRotationRateInit")]
    fn rotation_rate(&self) -> &DeviceRotationRateInit {
        self.rotation_rate_shim()
    }
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
        self.set_bubbles_shim(val);
        self
    }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceMotionEventInit`*"]
    pub fn cancelable(&mut self, val: bool) -> &mut Self {
        self.set_cancelable_shim(val);
        self
    }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceMotionEventInit`*"]
    pub fn composed(&mut self, val: bool) -> &mut Self {
        self.set_composed_shim(val);
        self
    }
    #[cfg(feature = "DeviceAccelerationInit")]
    #[doc = "Change the `acceleration` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceAccelerationInit`, `DeviceMotionEventInit`*"]
    pub fn acceleration(&mut self, val: &DeviceAccelerationInit) -> &mut Self {
        self.set_acceleration_shim(val);
        self
    }
    #[cfg(feature = "DeviceAccelerationInit")]
    #[doc = "Change the `accelerationIncludingGravity` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceAccelerationInit`, `DeviceMotionEventInit`*"]
    pub fn acceleration_including_gravity(&mut self, val: &DeviceAccelerationInit) -> &mut Self {
        self.set_acceleration_including_gravity_shim(val);
        self
    }
    #[doc = "Change the `interval` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceMotionEventInit`*"]
    pub fn interval(&mut self, val: Option<f64>) -> &mut Self {
        self.set_interval_shim(val);
        self
    }
    #[cfg(feature = "DeviceRotationRateInit")]
    #[doc = "Change the `rotationRate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceMotionEventInit`, `DeviceRotationRateInit`*"]
    pub fn rotation_rate(&mut self, val: &DeviceRotationRateInit) -> &mut Self {
        self.set_rotation_rate_shim(val);
        self
    }
}
impl Default for DeviceMotionEventInit {
    fn default() -> Self {
        Self::new()
    }
}
