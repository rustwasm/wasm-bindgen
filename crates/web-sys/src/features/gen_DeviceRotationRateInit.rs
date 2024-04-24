#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = DeviceRotationRateInit)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DeviceRotationRateInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceRotationRateInit`*"]
    pub type DeviceRotationRateInit;
    #[wasm_bindgen(method, getter = "alpha")]
    fn alpha_shim(this: &DeviceRotationRateInit) -> Option<f64>;
    #[wasm_bindgen(method, setter = "alpha")]
    fn set_alpha_shim(this: &DeviceRotationRateInit, val: Option<f64>);
    #[wasm_bindgen(method, getter = "beta")]
    fn beta_shim(this: &DeviceRotationRateInit) -> Option<f64>;
    #[wasm_bindgen(method, setter = "beta")]
    fn set_beta_shim(this: &DeviceRotationRateInit, val: Option<f64>);
    #[wasm_bindgen(method, getter = "gamma")]
    fn gamma_shim(this: &DeviceRotationRateInit) -> Option<f64>;
    #[wasm_bindgen(method, setter = "gamma")]
    fn set_gamma_shim(this: &DeviceRotationRateInit, val: Option<f64>);
}
#[doc = "The trait to access properties on the `DeviceRotationRateInit` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `DeviceRotationRateInit`*"]
pub trait DeviceRotationRateInitGetters {
    #[doc = "Get the `alpha` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceRotationRateInit`*"]
    fn alpha(&self) -> Option<f64>;
    #[doc = "Get the `beta` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceRotationRateInit`*"]
    fn beta(&self) -> Option<f64>;
    #[doc = "Get the `gamma` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceRotationRateInit`*"]
    fn gamma(&self) -> Option<f64>;
}
impl DeviceRotationRateInitGetters for DeviceRotationRateInit {
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
impl DeviceRotationRateInit {
    #[doc = "Construct a new `DeviceRotationRateInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceRotationRateInit`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `alpha` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceRotationRateInit`*"]
    pub fn alpha(&mut self, val: Option<f64>) -> &mut Self {
        self.set_alpha_shim(val);
        self
    }
    #[doc = "Change the `beta` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceRotationRateInit`*"]
    pub fn beta(&mut self, val: Option<f64>) -> &mut Self {
        self.set_beta_shim(val);
        self
    }
    #[doc = "Change the `gamma` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceRotationRateInit`*"]
    pub fn gamma(&mut self, val: Option<f64>) -> &mut Self {
        self.set_gamma_shim(val);
        self
    }
}
impl Default for DeviceRotationRateInit {
    fn default() -> Self {
        Self::new()
    }
}
