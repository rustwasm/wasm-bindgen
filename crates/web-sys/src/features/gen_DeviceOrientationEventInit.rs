#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = DeviceOrientationEventInit)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DeviceOrientationEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceOrientationEventInit`*"]
    pub type DeviceOrientationEventInit;
    #[wasm_bindgen(method, getter = "bubbles")]
    fn bubbles_shim(this: &DeviceOrientationEventInit) -> bool;
    #[wasm_bindgen(method, setter = "bubbles")]
    fn set_bubbles_shim(this: &DeviceOrientationEventInit, val: bool);
    #[wasm_bindgen(method, getter = "cancelable")]
    fn cancelable_shim(this: &DeviceOrientationEventInit) -> bool;
    #[wasm_bindgen(method, setter = "cancelable")]
    fn set_cancelable_shim(this: &DeviceOrientationEventInit, val: bool);
    #[wasm_bindgen(method, getter = "composed")]
    fn composed_shim(this: &DeviceOrientationEventInit) -> bool;
    #[wasm_bindgen(method, setter = "composed")]
    fn set_composed_shim(this: &DeviceOrientationEventInit, val: bool);
    #[wasm_bindgen(method, getter = "absolute")]
    fn absolute_shim(this: &DeviceOrientationEventInit) -> bool;
    #[wasm_bindgen(method, setter = "absolute")]
    fn set_absolute_shim(this: &DeviceOrientationEventInit, val: bool);
    #[wasm_bindgen(method, getter = "alpha")]
    fn alpha_shim(this: &DeviceOrientationEventInit) -> Option<f64>;
    #[wasm_bindgen(method, setter = "alpha")]
    fn set_alpha_shim(this: &DeviceOrientationEventInit, val: Option<f64>);
    #[wasm_bindgen(method, getter = "beta")]
    fn beta_shim(this: &DeviceOrientationEventInit) -> Option<f64>;
    #[wasm_bindgen(method, setter = "beta")]
    fn set_beta_shim(this: &DeviceOrientationEventInit, val: Option<f64>);
    #[wasm_bindgen(method, getter = "gamma")]
    fn gamma_shim(this: &DeviceOrientationEventInit) -> Option<f64>;
    #[wasm_bindgen(method, setter = "gamma")]
    fn set_gamma_shim(this: &DeviceOrientationEventInit, val: Option<f64>);
}
#[doc = "The trait to access properties on the `DeviceOrientationEventInit` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `DeviceOrientationEventInit`*"]
pub trait DeviceOrientationEventInitGetters {
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceOrientationEventInit`*"]
    fn bubbles(&self) -> bool;
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceOrientationEventInit`*"]
    fn cancelable(&self) -> bool;
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceOrientationEventInit`*"]
    fn composed(&self) -> bool;
    #[doc = "Get the `absolute` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceOrientationEventInit`*"]
    fn absolute(&self) -> bool;
    #[doc = "Get the `alpha` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceOrientationEventInit`*"]
    fn alpha(&self) -> Option<f64>;
    #[doc = "Get the `beta` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceOrientationEventInit`*"]
    fn beta(&self) -> Option<f64>;
    #[doc = "Get the `gamma` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceOrientationEventInit`*"]
    fn gamma(&self) -> Option<f64>;
}
impl DeviceOrientationEventInitGetters for DeviceOrientationEventInit {
    fn bubbles(&self) -> bool {
        self.bubbles_shim()
    }
    fn cancelable(&self) -> bool {
        self.cancelable_shim()
    }
    fn composed(&self) -> bool {
        self.composed_shim()
    }
    fn absolute(&self) -> bool {
        self.absolute_shim()
    }
    fn alpha(&self) -> Option<f64> {
        self.alpha_shim()
    }
    fn beta(&self) -> Option<f64> {
        self.beta_shim()
    }
    fn gamma(&self) -> Option<f64> {
        self.gamma_shim()
    }
}
impl DeviceOrientationEventInit {
    #[doc = "Construct a new `DeviceOrientationEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceOrientationEventInit`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceOrientationEventInit`*"]
    pub fn bubbles(&mut self, val: bool) -> &mut Self {
        self.set_bubbles_shim(val);
        self
    }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceOrientationEventInit`*"]
    pub fn cancelable(&mut self, val: bool) -> &mut Self {
        self.set_cancelable_shim(val);
        self
    }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceOrientationEventInit`*"]
    pub fn composed(&mut self, val: bool) -> &mut Self {
        self.set_composed_shim(val);
        self
    }
    #[doc = "Change the `absolute` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceOrientationEventInit`*"]
    pub fn absolute(&mut self, val: bool) -> &mut Self {
        self.set_absolute_shim(val);
        self
    }
    #[doc = "Change the `alpha` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceOrientationEventInit`*"]
    pub fn alpha(&mut self, val: Option<f64>) -> &mut Self {
        self.set_alpha_shim(val);
        self
    }
    #[doc = "Change the `beta` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceOrientationEventInit`*"]
    pub fn beta(&mut self, val: Option<f64>) -> &mut Self {
        self.set_beta_shim(val);
        self
    }
    #[doc = "Change the `gamma` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceOrientationEventInit`*"]
    pub fn gamma(&mut self, val: Option<f64>) -> &mut Self {
        self.set_gamma_shim(val);
        self
    }
}
impl Default for DeviceOrientationEventInit {
    fn default() -> Self {
        Self::new()
    }
}
