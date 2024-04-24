#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = MediaRecorderErrorEventInit)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MediaRecorderErrorEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaRecorderErrorEventInit`*"]
    pub type MediaRecorderErrorEventInit;
    #[wasm_bindgen(method, getter = "bubbles")]
    fn bubbles_shim(this: &MediaRecorderErrorEventInit) -> bool;
    #[wasm_bindgen(method, setter = "bubbles")]
    fn set_bubbles_shim(this: &MediaRecorderErrorEventInit, val: bool);
    #[wasm_bindgen(method, getter = "cancelable")]
    fn cancelable_shim(this: &MediaRecorderErrorEventInit) -> bool;
    #[wasm_bindgen(method, setter = "cancelable")]
    fn set_cancelable_shim(this: &MediaRecorderErrorEventInit, val: bool);
    #[wasm_bindgen(method, getter = "composed")]
    fn composed_shim(this: &MediaRecorderErrorEventInit) -> bool;
    #[wasm_bindgen(method, setter = "composed")]
    fn set_composed_shim(this: &MediaRecorderErrorEventInit, val: bool);
    #[cfg(feature = "DomException")]
    #[wasm_bindgen(method, getter = "error")]
    fn error_shim(this: &MediaRecorderErrorEventInit) -> DomException;
    #[cfg(feature = "DomException")]
    #[wasm_bindgen(method, setter = "error")]
    fn set_error_shim(this: &MediaRecorderErrorEventInit, val: &DomException);
}
#[doc = "The trait to access properties on the `MediaRecorderErrorEventInit` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `MediaRecorderErrorEventInit`*"]
pub trait MediaRecorderErrorEventInitGetters {
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaRecorderErrorEventInit`*"]
    fn bubbles(&self) -> bool;
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaRecorderErrorEventInit`*"]
    fn cancelable(&self) -> bool;
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaRecorderErrorEventInit`*"]
    fn composed(&self) -> bool;
    #[cfg(feature = "DomException")]
    #[doc = "Get the `error` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomException`, `MediaRecorderErrorEventInit`*"]
    fn error(&self) -> DomException;
}
impl MediaRecorderErrorEventInitGetters for MediaRecorderErrorEventInit {
    fn bubbles(&self) -> bool {
        self.bubbles_shim()
    }
    fn cancelable(&self) -> bool {
        self.cancelable_shim()
    }
    fn composed(&self) -> bool {
        self.composed_shim()
    }
    #[cfg(feature = "DomException")]
    fn error(&self) -> DomException {
        self.error_shim()
    }
}
impl MediaRecorderErrorEventInit {
    #[cfg(feature = "DomException")]
    #[doc = "Construct a new `MediaRecorderErrorEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomException`, `MediaRecorderErrorEventInit`*"]
    pub fn new(error: &DomException) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        Self::error(&mut ret, error);
        ret
    }
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaRecorderErrorEventInit`*"]
    pub fn bubbles(&mut self, val: bool) -> &mut Self {
        self.set_bubbles_shim(val);
        self
    }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaRecorderErrorEventInit`*"]
    pub fn cancelable(&mut self, val: bool) -> &mut Self {
        self.set_cancelable_shim(val);
        self
    }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaRecorderErrorEventInit`*"]
    pub fn composed(&mut self, val: bool) -> &mut Self {
        self.set_composed_shim(val);
        self
    }
    #[cfg(feature = "DomException")]
    #[doc = "Change the `error` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomException`, `MediaRecorderErrorEventInit`*"]
    pub fn error(&mut self, val: &DomException) -> &mut Self {
        self.set_error_shim(val);
        self
    }
}
