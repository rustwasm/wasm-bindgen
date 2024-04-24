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
    #[wasm_bindgen(method, getter = "bubbles")]
    fn bubbles_shim(this: &PerformanceEntryEventInit) -> bool;
    #[wasm_bindgen(method, setter = "bubbles")]
    fn set_bubbles_shim(this: &PerformanceEntryEventInit, val: bool);
    #[wasm_bindgen(method, getter = "cancelable")]
    fn cancelable_shim(this: &PerformanceEntryEventInit) -> bool;
    #[wasm_bindgen(method, setter = "cancelable")]
    fn set_cancelable_shim(this: &PerformanceEntryEventInit, val: bool);
    #[wasm_bindgen(method, getter = "composed")]
    fn composed_shim(this: &PerformanceEntryEventInit) -> bool;
    #[wasm_bindgen(method, setter = "composed")]
    fn set_composed_shim(this: &PerformanceEntryEventInit, val: bool);
    #[wasm_bindgen(method, getter = "duration")]
    fn duration_shim(this: &PerformanceEntryEventInit) -> f64;
    #[wasm_bindgen(method, setter = "duration")]
    fn set_duration_shim(this: &PerformanceEntryEventInit, val: f64);
    #[wasm_bindgen(method, getter = "entryType")]
    fn entry_type_shim(this: &PerformanceEntryEventInit) -> String;
    #[wasm_bindgen(method, setter = "entryType")]
    fn set_entry_type_shim(this: &PerformanceEntryEventInit, val: &str);
    #[wasm_bindgen(method, getter = "epoch")]
    fn epoch_shim(this: &PerformanceEntryEventInit) -> f64;
    #[wasm_bindgen(method, setter = "epoch")]
    fn set_epoch_shim(this: &PerformanceEntryEventInit, val: f64);
    #[wasm_bindgen(method, getter = "name")]
    fn name_shim(this: &PerformanceEntryEventInit) -> String;
    #[wasm_bindgen(method, setter = "name")]
    fn set_name_shim(this: &PerformanceEntryEventInit, val: &str);
    #[wasm_bindgen(method, getter = "origin")]
    fn origin_shim(this: &PerformanceEntryEventInit) -> String;
    #[wasm_bindgen(method, setter = "origin")]
    fn set_origin_shim(this: &PerformanceEntryEventInit, val: &str);
    #[wasm_bindgen(method, getter = "startTime")]
    fn start_time_shim(this: &PerformanceEntryEventInit) -> f64;
    #[wasm_bindgen(method, setter = "startTime")]
    fn set_start_time_shim(this: &PerformanceEntryEventInit, val: f64);
}
#[doc = "The trait to access properties on the `PerformanceEntryEventInit` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `PerformanceEntryEventInit`*"]
pub trait PerformanceEntryEventInitGetters {
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceEntryEventInit`*"]
    fn bubbles(&self) -> bool;
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceEntryEventInit`*"]
    fn cancelable(&self) -> bool;
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceEntryEventInit`*"]
    fn composed(&self) -> bool;
    #[doc = "Get the `duration` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceEntryEventInit`*"]
    fn duration(&self) -> f64;
    #[doc = "Get the `entryType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceEntryEventInit`*"]
    fn entry_type(&self) -> String;
    #[doc = "Get the `epoch` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceEntryEventInit`*"]
    fn epoch(&self) -> f64;
    #[doc = "Get the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceEntryEventInit`*"]
    fn name(&self) -> String;
    #[doc = "Get the `origin` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceEntryEventInit`*"]
    fn origin(&self) -> String;
    #[doc = "Get the `startTime` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceEntryEventInit`*"]
    fn start_time(&self) -> f64;
}
impl PerformanceEntryEventInitGetters for PerformanceEntryEventInit {
    fn bubbles(&self) -> bool {
        self.bubbles_shim()
    }
    fn cancelable(&self) -> bool {
        self.cancelable_shim()
    }
    fn composed(&self) -> bool {
        self.composed_shim()
    }
    fn duration(&self) -> f64 {
        self.duration_shim()
    }
    fn entry_type(&self) -> String {
        self.entry_type_shim()
    }
    fn epoch(&self) -> f64 {
        self.epoch_shim()
    }
    fn name(&self) -> String {
        self.name_shim()
    }
    fn origin(&self) -> String {
        self.origin_shim()
    }
    fn start_time(&self) -> f64 {
        self.start_time_shim()
    }
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
        self.set_bubbles_shim(val);
        self
    }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceEntryEventInit`*"]
    pub fn cancelable(&mut self, val: bool) -> &mut Self {
        self.set_cancelable_shim(val);
        self
    }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceEntryEventInit`*"]
    pub fn composed(&mut self, val: bool) -> &mut Self {
        self.set_composed_shim(val);
        self
    }
    #[doc = "Change the `duration` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceEntryEventInit`*"]
    pub fn duration(&mut self, val: f64) -> &mut Self {
        self.set_duration_shim(val);
        self
    }
    #[doc = "Change the `entryType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceEntryEventInit`*"]
    pub fn entry_type(&mut self, val: &str) -> &mut Self {
        self.set_entry_type_shim(val);
        self
    }
    #[doc = "Change the `epoch` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceEntryEventInit`*"]
    pub fn epoch(&mut self, val: f64) -> &mut Self {
        self.set_epoch_shim(val);
        self
    }
    #[doc = "Change the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceEntryEventInit`*"]
    pub fn name(&mut self, val: &str) -> &mut Self {
        self.set_name_shim(val);
        self
    }
    #[doc = "Change the `origin` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceEntryEventInit`*"]
    pub fn origin(&mut self, val: &str) -> &mut Self {
        self.set_origin_shim(val);
        self
    }
    #[doc = "Change the `startTime` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceEntryEventInit`*"]
    pub fn start_time(&mut self, val: f64) -> &mut Self {
        self.set_start_time_shim(val);
        self
    }
}
impl Default for PerformanceEntryEventInit {
    fn default() -> Self {
        Self::new()
    }
}
