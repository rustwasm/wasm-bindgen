#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = PushEventInit)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PushEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushEventInit`*"]
    pub type PushEventInit;
    #[wasm_bindgen(method, getter = "bubbles")]
    fn bubbles_shim(this: &PushEventInit) -> bool;
    #[wasm_bindgen(method, setter = "bubbles")]
    fn set_bubbles_shim(this: &PushEventInit, val: bool);
    #[wasm_bindgen(method, getter = "cancelable")]
    fn cancelable_shim(this: &PushEventInit) -> bool;
    #[wasm_bindgen(method, setter = "cancelable")]
    fn set_cancelable_shim(this: &PushEventInit, val: bool);
    #[wasm_bindgen(method, getter = "composed")]
    fn composed_shim(this: &PushEventInit) -> bool;
    #[wasm_bindgen(method, setter = "composed")]
    fn set_composed_shim(this: &PushEventInit, val: bool);
    #[wasm_bindgen(method, getter = "data")]
    fn data_shim(this: &PushEventInit) -> &::wasm_bindgen::JsValue;
    #[wasm_bindgen(method, setter = "data")]
    fn set_data_shim(this: &PushEventInit, val: &::wasm_bindgen::JsValue);
}
#[doc = "The trait to access properties on the `PushEventInit` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `PushEventInit`*"]
pub trait PushEventInitGetters {
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushEventInit`*"]
    fn bubbles(&self) -> bool;
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushEventInit`*"]
    fn cancelable(&self) -> bool;
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushEventInit`*"]
    fn composed(&self) -> bool;
    #[doc = "Get the `data` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushEventInit`*"]
    fn data(&self) -> &::wasm_bindgen::JsValue;
}
impl PushEventInitGetters for PushEventInit {
    fn bubbles(&self) -> bool {
        self.bubbles_shim()
    }
    fn cancelable(&self) -> bool {
        self.cancelable_shim()
    }
    fn composed(&self) -> bool {
        self.composed_shim()
    }
    fn data(&self) -> &::wasm_bindgen::JsValue {
        self.data_shim()
    }
}
impl PushEventInit {
    #[doc = "Construct a new `PushEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushEventInit`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushEventInit`*"]
    pub fn bubbles(&mut self, val: bool) -> &mut Self {
        self.set_bubbles_shim(val);
        self
    }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushEventInit`*"]
    pub fn cancelable(&mut self, val: bool) -> &mut Self {
        self.set_cancelable_shim(val);
        self
    }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushEventInit`*"]
    pub fn composed(&mut self, val: bool) -> &mut Self {
        self.set_composed_shim(val);
        self
    }
    #[doc = "Change the `data` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushEventInit`*"]
    pub fn data(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_data_shim(val);
        self
    }
}
impl Default for PushEventInit {
    fn default() -> Self {
        Self::new()
    }
}
