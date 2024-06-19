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
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceOrientationEventInit`*"]
    #[wasm_bindgen(method, getter = "bubbles")]
    pub fn get_bubbles(this: &DeviceOrientationEventInit) -> Option<bool>;
    #[wasm_bindgen(method, setter = "bubbles")]
    fn set_bubbles(this: &DeviceOrientationEventInit, val: bool);
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceOrientationEventInit`*"]
    #[wasm_bindgen(method, getter = "cancelable")]
    pub fn get_cancelable(this: &DeviceOrientationEventInit) -> Option<bool>;
    #[wasm_bindgen(method, setter = "cancelable")]
    fn set_cancelable(this: &DeviceOrientationEventInit, val: bool);
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceOrientationEventInit`*"]
    #[wasm_bindgen(method, getter = "composed")]
    pub fn get_composed(this: &DeviceOrientationEventInit) -> Option<bool>;
    #[wasm_bindgen(method, setter = "composed")]
    fn set_composed(this: &DeviceOrientationEventInit, val: bool);
    #[doc = "Get the `absolute` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceOrientationEventInit`*"]
    #[wasm_bindgen(method, getter = "absolute")]
    pub fn get_absolute(this: &DeviceOrientationEventInit) -> Option<bool>;
    #[wasm_bindgen(method, setter = "absolute")]
    fn set_absolute(this: &DeviceOrientationEventInit, val: bool);
    #[doc = "Get the `alpha` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceOrientationEventInit`*"]
    #[wasm_bindgen(method, getter = "alpha")]
    pub fn get_alpha(this: &DeviceOrientationEventInit) -> Option<f64>;
    #[wasm_bindgen(method, setter = "alpha")]
    fn set_alpha(this: &DeviceOrientationEventInit, val: Option<f64>);
    #[doc = "Get the `beta` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceOrientationEventInit`*"]
    #[wasm_bindgen(method, getter = "beta")]
    pub fn get_beta(this: &DeviceOrientationEventInit) -> Option<f64>;
    #[wasm_bindgen(method, setter = "beta")]
    fn set_beta(this: &DeviceOrientationEventInit, val: Option<f64>);
    #[doc = "Get the `gamma` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceOrientationEventInit`*"]
    #[wasm_bindgen(method, getter = "gamma")]
    pub fn get_gamma(this: &DeviceOrientationEventInit) -> Option<f64>;
    #[wasm_bindgen(method, setter = "gamma")]
    fn set_gamma(this: &DeviceOrientationEventInit, val: Option<f64>);
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
        self.set_bubbles(val);
        self
    }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceOrientationEventInit`*"]
    pub fn cancelable(&mut self, val: bool) -> &mut Self {
        self.set_cancelable(val);
        self
    }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceOrientationEventInit`*"]
    pub fn composed(&mut self, val: bool) -> &mut Self {
        self.set_composed(val);
        self
    }
    #[doc = "Change the `absolute` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceOrientationEventInit`*"]
    pub fn absolute(&mut self, val: bool) -> &mut Self {
        self.set_absolute(val);
        self
    }
    #[doc = "Change the `alpha` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceOrientationEventInit`*"]
    pub fn alpha(&mut self, val: Option<f64>) -> &mut Self {
        self.set_alpha(val);
        self
    }
    #[doc = "Change the `beta` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceOrientationEventInit`*"]
    pub fn beta(&mut self, val: Option<f64>) -> &mut Self {
        self.set_beta(val);
        self
    }
    #[doc = "Change the `gamma` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceOrientationEventInit`*"]
    pub fn gamma(&mut self, val: Option<f64>) -> &mut Self {
        self.set_gamma(val);
        self
    }
}
impl Default for DeviceOrientationEventInit {
    fn default() -> Self {
        Self::new()
    }
}
