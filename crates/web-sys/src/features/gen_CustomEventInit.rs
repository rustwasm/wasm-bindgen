#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = CustomEventInit)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `CustomEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CustomEventInit`*"]
    pub type CustomEventInit;
    #[wasm_bindgen(method, getter = "bubbles")]
    fn bubbles_shim(this: &CustomEventInit) -> bool;
    #[wasm_bindgen(method, setter = "bubbles")]
    fn set_bubbles_shim(this: &CustomEventInit, val: bool);
    #[wasm_bindgen(method, getter = "cancelable")]
    fn cancelable_shim(this: &CustomEventInit) -> bool;
    #[wasm_bindgen(method, setter = "cancelable")]
    fn set_cancelable_shim(this: &CustomEventInit, val: bool);
    #[wasm_bindgen(method, getter = "composed")]
    fn composed_shim(this: &CustomEventInit) -> bool;
    #[wasm_bindgen(method, setter = "composed")]
    fn set_composed_shim(this: &CustomEventInit, val: bool);
    #[wasm_bindgen(method, getter = "detail")]
    fn detail_shim(this: &CustomEventInit) -> &::wasm_bindgen::JsValue;
    #[wasm_bindgen(method, setter = "detail")]
    fn set_detail_shim(this: &CustomEventInit, val: &::wasm_bindgen::JsValue);
}
#[doc = "The trait to access properties on the `CustomEventInit` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `CustomEventInit`*"]
pub trait CustomEventInitGetters {
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CustomEventInit`*"]
    fn bubbles(&self) -> bool;
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CustomEventInit`*"]
    fn cancelable(&self) -> bool;
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CustomEventInit`*"]
    fn composed(&self) -> bool;
    #[doc = "Get the `detail` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CustomEventInit`*"]
    fn detail(&self) -> &::wasm_bindgen::JsValue;
}
impl CustomEventInitGetters for CustomEventInit {
    fn bubbles(&self) -> bool {
        self.bubbles_shim()
    }
    fn cancelable(&self) -> bool {
        self.cancelable_shim()
    }
    fn composed(&self) -> bool {
        self.composed_shim()
    }
    fn detail(&self) -> &::wasm_bindgen::JsValue {
        self.detail_shim()
    }
}
impl CustomEventInit {
    #[doc = "Construct a new `CustomEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CustomEventInit`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CustomEventInit`*"]
    pub fn bubbles(&mut self, val: bool) -> &mut Self {
        self.set_bubbles_shim(val);
        self
    }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CustomEventInit`*"]
    pub fn cancelable(&mut self, val: bool) -> &mut Self {
        self.set_cancelable_shim(val);
        self
    }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CustomEventInit`*"]
    pub fn composed(&mut self, val: bool) -> &mut Self {
        self.set_composed_shim(val);
        self
    }
    #[doc = "Change the `detail` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CustomEventInit`*"]
    pub fn detail(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_detail_shim(val);
        self
    }
}
impl Default for CustomEventInit {
    fn default() -> Self {
        Self::new()
    }
}
