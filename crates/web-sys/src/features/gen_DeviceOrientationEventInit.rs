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
    #[wasm_bindgen(method, setter = "bubbles")]
    fn bubbles_shim(this: &DeviceOrientationEventInit, val: bool);
    #[wasm_bindgen(method, setter = "cancelable")]
    fn cancelable_shim(this: &DeviceOrientationEventInit, val: bool);
    #[wasm_bindgen(method, setter = "composed")]
    fn composed_shim(this: &DeviceOrientationEventInit, val: bool);
    #[wasm_bindgen(method, setter = "absolute")]
    fn absolute_shim(this: &DeviceOrientationEventInit, val: bool);
    #[wasm_bindgen(method, setter = "alpha")]
    fn alpha_shim(this: &DeviceOrientationEventInit, val: Option<f64>);
    #[wasm_bindgen(method, setter = "beta")]
    fn beta_shim(this: &DeviceOrientationEventInit, val: Option<f64>);
    #[wasm_bindgen(method, setter = "gamma")]
    fn gamma_shim(this: &DeviceOrientationEventInit, val: Option<f64>);
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
        self.bubbles_shim(val);
        self
    }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceOrientationEventInit`*"]
    pub fn cancelable(&mut self, val: bool) -> &mut Self {
        self.cancelable_shim(val);
        self
    }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceOrientationEventInit`*"]
    pub fn composed(&mut self, val: bool) -> &mut Self {
        self.composed_shim(val);
        self
    }
    #[doc = "Change the `absolute` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceOrientationEventInit`*"]
    pub fn absolute(&mut self, val: bool) -> &mut Self {
        self.absolute_shim(val);
        self
    }
    #[doc = "Change the `alpha` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceOrientationEventInit`*"]
    pub fn alpha(&mut self, val: Option<f64>) -> &mut Self {
        self.alpha_shim(val);
        self
    }
    #[doc = "Change the `beta` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceOrientationEventInit`*"]
    pub fn beta(&mut self, val: Option<f64>) -> &mut Self {
        self.beta_shim(val);
        self
    }
    #[doc = "Change the `gamma` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceOrientationEventInit`*"]
    pub fn gamma(&mut self, val: Option<f64>) -> &mut Self {
        self.gamma_shim(val);
        self
    }
}
impl Default for DeviceOrientationEventInit {
    fn default() -> Self {
        Self::new()
    }
}
