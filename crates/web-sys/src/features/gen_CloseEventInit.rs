#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = CloseEventInit)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `CloseEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CloseEventInit`*"]
    pub type CloseEventInit;
    #[wasm_bindgen(method, getter = "bubbles")]
    fn bubbles_shim(this: &CloseEventInit) -> bool;
    #[wasm_bindgen(method, setter = "bubbles")]
    fn set_bubbles_shim(this: &CloseEventInit, val: bool);
    #[wasm_bindgen(method, getter = "cancelable")]
    fn cancelable_shim(this: &CloseEventInit) -> bool;
    #[wasm_bindgen(method, setter = "cancelable")]
    fn set_cancelable_shim(this: &CloseEventInit, val: bool);
    #[wasm_bindgen(method, getter = "composed")]
    fn composed_shim(this: &CloseEventInit) -> bool;
    #[wasm_bindgen(method, setter = "composed")]
    fn set_composed_shim(this: &CloseEventInit, val: bool);
    #[wasm_bindgen(method, getter = "code")]
    fn code_shim(this: &CloseEventInit) -> u16;
    #[wasm_bindgen(method, setter = "code")]
    fn set_code_shim(this: &CloseEventInit, val: u16);
    #[wasm_bindgen(method, getter = "reason")]
    fn reason_shim(this: &CloseEventInit) -> &str;
    #[wasm_bindgen(method, setter = "reason")]
    fn set_reason_shim(this: &CloseEventInit, val: &str);
    #[wasm_bindgen(method, getter = "wasClean")]
    fn was_clean_shim(this: &CloseEventInit) -> bool;
    #[wasm_bindgen(method, setter = "wasClean")]
    fn set_was_clean_shim(this: &CloseEventInit, val: bool);
}
#[doc = "The trait to access properties on the `CloseEventInit` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `CloseEventInit`*"]
pub trait CloseEventInitGetters {
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CloseEventInit`*"]
    fn bubbles(&self) -> bool;
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CloseEventInit`*"]
    fn cancelable(&self) -> bool;
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CloseEventInit`*"]
    fn composed(&self) -> bool;
    #[doc = "Get the `code` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CloseEventInit`*"]
    fn code(&self) -> u16;
    #[doc = "Get the `reason` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CloseEventInit`*"]
    fn reason(&self) -> &str;
    #[doc = "Get the `wasClean` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CloseEventInit`*"]
    fn was_clean(&self) -> bool;
}
impl CloseEventInitGetters for CloseEventInit {
    fn bubbles(&self) -> bool {
        self.bubbles_shim()
    }
    fn cancelable(&self) -> bool {
        self.cancelable_shim()
    }
    fn composed(&self) -> bool {
        self.composed_shim()
    }
    fn code(&self) -> u16 {
        self.code_shim()
    }
    fn reason(&self) -> &str {
        self.reason_shim()
    }
    fn was_clean(&self) -> bool {
        self.was_clean_shim()
    }
}
impl CloseEventInit {
    #[doc = "Construct a new `CloseEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CloseEventInit`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CloseEventInit`*"]
    pub fn bubbles(&mut self, val: bool) -> &mut Self {
        self.set_bubbles_shim(val);
        self
    }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CloseEventInit`*"]
    pub fn cancelable(&mut self, val: bool) -> &mut Self {
        self.set_cancelable_shim(val);
        self
    }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CloseEventInit`*"]
    pub fn composed(&mut self, val: bool) -> &mut Self {
        self.set_composed_shim(val);
        self
    }
    #[doc = "Change the `code` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CloseEventInit`*"]
    pub fn code(&mut self, val: u16) -> &mut Self {
        self.set_code_shim(val);
        self
    }
    #[doc = "Change the `reason` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CloseEventInit`*"]
    pub fn reason(&mut self, val: &str) -> &mut Self {
        self.set_reason_shim(val);
        self
    }
    #[doc = "Change the `wasClean` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CloseEventInit`*"]
    pub fn was_clean(&mut self, val: bool) -> &mut Self {
        self.set_was_clean_shim(val);
        self
    }
}
impl Default for CloseEventInit {
    fn default() -> Self {
        Self::new()
    }
}
