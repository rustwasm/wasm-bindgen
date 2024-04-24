#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = DeviceLightEventInit)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DeviceLightEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceLightEventInit`*"]
    pub type DeviceLightEventInit;
    #[wasm_bindgen(method, getter = "bubbles")]
    fn bubbles_shim(this: &DeviceLightEventInit) -> bool;
    #[wasm_bindgen(method, setter = "bubbles")]
    fn set_bubbles_shim(this: &DeviceLightEventInit, val: bool);
    #[wasm_bindgen(method, getter = "cancelable")]
    fn cancelable_shim(this: &DeviceLightEventInit) -> bool;
    #[wasm_bindgen(method, setter = "cancelable")]
    fn set_cancelable_shim(this: &DeviceLightEventInit, val: bool);
    #[wasm_bindgen(method, getter = "composed")]
    fn composed_shim(this: &DeviceLightEventInit) -> bool;
    #[wasm_bindgen(method, setter = "composed")]
    fn set_composed_shim(this: &DeviceLightEventInit, val: bool);
    #[wasm_bindgen(method, getter = "value")]
    fn value_shim(this: &DeviceLightEventInit) -> f64;
    #[wasm_bindgen(method, setter = "value")]
    fn set_value_shim(this: &DeviceLightEventInit, val: f64);
}
#[doc = "The trait to access properties on the `DeviceLightEventInit` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `DeviceLightEventInit`*"]
pub trait DeviceLightEventInitGetters {
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceLightEventInit`*"]
    fn bubbles(&self) -> bool;
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceLightEventInit`*"]
    fn cancelable(&self) -> bool;
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceLightEventInit`*"]
    fn composed(&self) -> bool;
    #[doc = "Get the `value` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceLightEventInit`*"]
    fn value(&self) -> f64;
}
impl DeviceLightEventInitGetters for DeviceLightEventInit {
    fn bubbles(&self) -> bool {
        self.bubbles_shim()
    }
    fn cancelable(&self) -> bool {
        self.cancelable_shim()
    }
    fn composed(&self) -> bool {
        self.composed_shim()
    }
    fn value(&self) -> f64 {
        self.value_shim()
    }
}
impl DeviceLightEventInit {
    #[doc = "Construct a new `DeviceLightEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceLightEventInit`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceLightEventInit`*"]
    pub fn bubbles(&mut self, val: bool) -> &mut Self {
        self.set_bubbles_shim(val);
        self
    }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceLightEventInit`*"]
    pub fn cancelable(&mut self, val: bool) -> &mut Self {
        self.set_cancelable_shim(val);
        self
    }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceLightEventInit`*"]
    pub fn composed(&mut self, val: bool) -> &mut Self {
        self.set_composed_shim(val);
        self
    }
    #[doc = "Change the `value` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceLightEventInit`*"]
    pub fn value(&mut self, val: f64) -> &mut Self {
        self.set_value_shim(val);
        self
    }
}
impl Default for DeviceLightEventInit {
    fn default() -> Self {
        Self::new()
    }
}
