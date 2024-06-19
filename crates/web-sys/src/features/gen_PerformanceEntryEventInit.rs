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
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceEntryEventInit`*"]
    #[wasm_bindgen(method, getter = "bubbles")]
    pub fn get_bubbles(this: &PerformanceEntryEventInit) -> Option<bool>;
    #[wasm_bindgen(method, setter = "bubbles")]
    fn set_bubbles(this: &PerformanceEntryEventInit, val: bool);
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceEntryEventInit`*"]
    #[wasm_bindgen(method, getter = "cancelable")]
    pub fn get_cancelable(this: &PerformanceEntryEventInit) -> Option<bool>;
    #[wasm_bindgen(method, setter = "cancelable")]
    fn set_cancelable(this: &PerformanceEntryEventInit, val: bool);
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceEntryEventInit`*"]
    #[wasm_bindgen(method, getter = "composed")]
    pub fn get_composed(this: &PerformanceEntryEventInit) -> Option<bool>;
    #[wasm_bindgen(method, setter = "composed")]
    fn set_composed(this: &PerformanceEntryEventInit, val: bool);
    #[doc = "Get the `duration` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceEntryEventInit`*"]
    #[wasm_bindgen(method, getter = "duration")]
    pub fn get_duration(this: &PerformanceEntryEventInit) -> Option<f64>;
    #[wasm_bindgen(method, setter = "duration")]
    fn set_duration(this: &PerformanceEntryEventInit, val: f64);
    #[doc = "Get the `entryType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceEntryEventInit`*"]
    #[wasm_bindgen(method, getter = "entryType")]
    pub fn get_entry_type(this: &PerformanceEntryEventInit) -> Option<String>;
    #[wasm_bindgen(method, setter = "entryType")]
    fn set_entry_type(this: &PerformanceEntryEventInit, val: &str);
    #[doc = "Get the `epoch` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceEntryEventInit`*"]
    #[wasm_bindgen(method, getter = "epoch")]
    pub fn get_epoch(this: &PerformanceEntryEventInit) -> Option<f64>;
    #[wasm_bindgen(method, setter = "epoch")]
    fn set_epoch(this: &PerformanceEntryEventInit, val: f64);
    #[doc = "Get the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceEntryEventInit`*"]
    #[wasm_bindgen(method, getter = "name")]
    pub fn get_name(this: &PerformanceEntryEventInit) -> Option<String>;
    #[wasm_bindgen(method, setter = "name")]
    fn set_name(this: &PerformanceEntryEventInit, val: &str);
    #[doc = "Get the `origin` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceEntryEventInit`*"]
    #[wasm_bindgen(method, getter = "origin")]
    pub fn get_origin(this: &PerformanceEntryEventInit) -> Option<String>;
    #[wasm_bindgen(method, setter = "origin")]
    fn set_origin(this: &PerformanceEntryEventInit, val: &str);
    #[doc = "Get the `startTime` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceEntryEventInit`*"]
    #[wasm_bindgen(method, getter = "startTime")]
    pub fn get_start_time(this: &PerformanceEntryEventInit) -> Option<f64>;
    #[wasm_bindgen(method, setter = "startTime")]
    fn set_start_time(this: &PerformanceEntryEventInit, val: f64);
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
        self.set_bubbles(val);
        self
    }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceEntryEventInit`*"]
    pub fn cancelable(&mut self, val: bool) -> &mut Self {
        self.set_cancelable(val);
        self
    }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceEntryEventInit`*"]
    pub fn composed(&mut self, val: bool) -> &mut Self {
        self.set_composed(val);
        self
    }
    #[doc = "Change the `duration` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceEntryEventInit`*"]
    pub fn duration(&mut self, val: f64) -> &mut Self {
        self.set_duration(val);
        self
    }
    #[doc = "Change the `entryType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceEntryEventInit`*"]
    pub fn entry_type(&mut self, val: &str) -> &mut Self {
        self.set_entry_type(val);
        self
    }
    #[doc = "Change the `epoch` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceEntryEventInit`*"]
    pub fn epoch(&mut self, val: f64) -> &mut Self {
        self.set_epoch(val);
        self
    }
    #[doc = "Change the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceEntryEventInit`*"]
    pub fn name(&mut self, val: &str) -> &mut Self {
        self.set_name(val);
        self
    }
    #[doc = "Change the `origin` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceEntryEventInit`*"]
    pub fn origin(&mut self, val: &str) -> &mut Self {
        self.set_origin(val);
        self
    }
    #[doc = "Change the `startTime` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceEntryEventInit`*"]
    pub fn start_time(&mut self, val: f64) -> &mut Self {
        self.set_start_time(val);
        self
    }
}
impl Default for PerformanceEntryEventInit {
    fn default() -> Self {
        Self::new()
    }
}
