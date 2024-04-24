#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = IDBVersionChangeEventInit)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `IdbVersionChangeEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbVersionChangeEventInit`*"]
    pub type IdbVersionChangeEventInit;
    #[wasm_bindgen(method, getter = "bubbles")]
    fn bubbles_shim(this: &IdbVersionChangeEventInit) -> bool;
    #[wasm_bindgen(method, setter = "bubbles")]
    fn set_bubbles_shim(this: &IdbVersionChangeEventInit, val: bool);
    #[wasm_bindgen(method, getter = "cancelable")]
    fn cancelable_shim(this: &IdbVersionChangeEventInit) -> bool;
    #[wasm_bindgen(method, setter = "cancelable")]
    fn set_cancelable_shim(this: &IdbVersionChangeEventInit, val: bool);
    #[wasm_bindgen(method, getter = "composed")]
    fn composed_shim(this: &IdbVersionChangeEventInit) -> bool;
    #[wasm_bindgen(method, setter = "composed")]
    fn set_composed_shim(this: &IdbVersionChangeEventInit, val: bool);
    #[wasm_bindgen(method, getter = "newVersion")]
    fn new_version_shim(this: &IdbVersionChangeEventInit) -> Option<f64>;
    #[wasm_bindgen(method, setter = "newVersion")]
    fn set_new_version_shim(this: &IdbVersionChangeEventInit, val: Option<f64>);
    #[wasm_bindgen(method, getter = "oldVersion")]
    fn old_version_shim(this: &IdbVersionChangeEventInit) -> f64;
    #[wasm_bindgen(method, setter = "oldVersion")]
    fn set_old_version_shim(this: &IdbVersionChangeEventInit, val: f64);
}
#[doc = "The trait to access properties on the `IdbVersionChangeEventInit` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `IdbVersionChangeEventInit`*"]
pub trait IdbVersionChangeEventInitGetters {
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbVersionChangeEventInit`*"]
    fn bubbles(&self) -> bool;
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbVersionChangeEventInit`*"]
    fn cancelable(&self) -> bool;
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbVersionChangeEventInit`*"]
    fn composed(&self) -> bool;
    #[doc = "Get the `newVersion` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbVersionChangeEventInit`*"]
    fn new_version(&self) -> Option<f64>;
    #[doc = "Get the `oldVersion` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbVersionChangeEventInit`*"]
    fn old_version(&self) -> f64;
}
impl IdbVersionChangeEventInitGetters for IdbVersionChangeEventInit {
    fn bubbles(&self) -> bool {
        self.bubbles_shim()
    }
    fn cancelable(&self) -> bool {
        self.cancelable_shim()
    }
    fn composed(&self) -> bool {
        self.composed_shim()
    }
    fn new_version(&self) -> Option<f64> {
        self.new_version_shim()
    }
    fn old_version(&self) -> f64 {
        self.old_version_shim()
    }
}
impl IdbVersionChangeEventInit {
    #[doc = "Construct a new `IdbVersionChangeEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbVersionChangeEventInit`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbVersionChangeEventInit`*"]
    pub fn bubbles(&mut self, val: bool) -> &mut Self {
        self.set_bubbles_shim(val);
        self
    }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbVersionChangeEventInit`*"]
    pub fn cancelable(&mut self, val: bool) -> &mut Self {
        self.set_cancelable_shim(val);
        self
    }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbVersionChangeEventInit`*"]
    pub fn composed(&mut self, val: bool) -> &mut Self {
        self.set_composed_shim(val);
        self
    }
    #[doc = "Change the `newVersion` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbVersionChangeEventInit`*"]
    pub fn new_version(&mut self, val: Option<f64>) -> &mut Self {
        self.set_new_version_shim(val);
        self
    }
    #[doc = "Change the `oldVersion` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbVersionChangeEventInit`*"]
    pub fn old_version(&mut self, val: f64) -> &mut Self {
        self.set_old_version_shim(val);
        self
    }
}
impl Default for IdbVersionChangeEventInit {
    fn default() -> Self {
        Self::new()
    }
}
