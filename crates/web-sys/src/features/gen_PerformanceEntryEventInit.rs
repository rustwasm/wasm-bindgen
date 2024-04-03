#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = PerformanceEntryEventInit)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PerformanceEntryEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceEntryEventInit`*"]
    pub type PerformanceEntryEventInit;
    #[wasm_bindgen(method, setter = "bubbles")]
    fn bubbles_shim(this: &PerformanceEntryEventInit, val: bool);
    #[wasm_bindgen(method, setter = "cancelable")]
    fn cancelable_shim(this: &PerformanceEntryEventInit, val: bool);
    #[wasm_bindgen(method, setter = "composed")]
    fn composed_shim(this: &PerformanceEntryEventInit, val: bool);
    #[wasm_bindgen(method, setter = "duration")]
    fn duration_shim(this: &PerformanceEntryEventInit, val: f64);
    #[wasm_bindgen(method, setter = "entryType")]
    fn entry_type_shim(this: &PerformanceEntryEventInit, val: &str);
    #[wasm_bindgen(method, setter = "epoch")]
    fn epoch_shim(this: &PerformanceEntryEventInit, val: f64);
    #[wasm_bindgen(method, setter = "name")]
    fn name_shim(this: &PerformanceEntryEventInit, val: &str);
    #[wasm_bindgen(method, setter = "origin")]
    fn origin_shim(this: &PerformanceEntryEventInit, val: &str);
    #[wasm_bindgen(method, setter = "startTime")]
    fn start_time_shim(this: &PerformanceEntryEventInit, val: f64);
}
impl PerformanceEntryEventInit {
    #[doc = "Construct a new `PerformanceEntryEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceEntryEventInit`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceEntryEventInit`*"]
    pub fn bubbles(&mut self, val: bool) -> &mut Self {
        self.bubbles_shim(val);
        self
    }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceEntryEventInit`*"]
    pub fn cancelable(&mut self, val: bool) -> &mut Self {
        self.cancelable_shim(val);
        self
    }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceEntryEventInit`*"]
    pub fn composed(&mut self, val: bool) -> &mut Self {
        self.composed_shim(val);
        self
    }
    #[doc = "Change the `duration` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceEntryEventInit`*"]
    pub fn duration(&mut self, val: f64) -> &mut Self {
        self.duration_shim(val);
        self
    }
    #[doc = "Change the `entryType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceEntryEventInit`*"]
    pub fn entry_type(&mut self, val: &str) -> &mut Self {
        self.entry_type_shim(val);
        self
    }
    #[doc = "Change the `epoch` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceEntryEventInit`*"]
    pub fn epoch(&mut self, val: f64) -> &mut Self {
        self.epoch_shim(val);
        self
    }
    #[doc = "Change the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceEntryEventInit`*"]
    pub fn name(&mut self, val: &str) -> &mut Self {
        self.name_shim(val);
        self
    }
    #[doc = "Change the `origin` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceEntryEventInit`*"]
    pub fn origin(&mut self, val: &str) -> &mut Self {
        self.origin_shim(val);
        self
    }
    #[doc = "Change the `startTime` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceEntryEventInit`*"]
    pub fn start_time(&mut self, val: f64) -> &mut Self {
        self.start_time_shim(val);
        self
    }
}
impl Default for PerformanceEntryEventInit {
    fn default() -> Self {
        Self::new()
    }
}
