#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = PerformanceEntryFilterOptions)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PerformanceEntryFilterOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceEntryFilterOptions`*"]
    pub type PerformanceEntryFilterOptions;
    #[wasm_bindgen(method, getter = "entryType")]
    fn entry_type_shim(this: &PerformanceEntryFilterOptions) -> String;
    #[wasm_bindgen(method, setter = "entryType")]
    fn set_entry_type_shim(this: &PerformanceEntryFilterOptions, val: &str);
    #[wasm_bindgen(method, getter = "initiatorType")]
    fn initiator_type_shim(this: &PerformanceEntryFilterOptions) -> String;
    #[wasm_bindgen(method, setter = "initiatorType")]
    fn set_initiator_type_shim(this: &PerformanceEntryFilterOptions, val: &str);
    #[wasm_bindgen(method, getter = "name")]
    fn name_shim(this: &PerformanceEntryFilterOptions) -> String;
    #[wasm_bindgen(method, setter = "name")]
    fn set_name_shim(this: &PerformanceEntryFilterOptions, val: &str);
}
#[doc = "The trait to access properties on the `PerformanceEntryFilterOptions` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `PerformanceEntryFilterOptions`*"]
pub trait PerformanceEntryFilterOptionsGetters {
    #[doc = "Get the `entryType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceEntryFilterOptions`*"]
    fn entry_type(&self) -> String;
    #[doc = "Get the `initiatorType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceEntryFilterOptions`*"]
    fn initiator_type(&self) -> String;
    #[doc = "Get the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceEntryFilterOptions`*"]
    fn name(&self) -> String;
}
impl PerformanceEntryFilterOptionsGetters for PerformanceEntryFilterOptions {
    fn entry_type(&self) -> String {
        self.entry_type_shim()
    }
    fn initiator_type(&self) -> String {
        self.initiator_type_shim()
    }
    fn name(&self) -> String {
        self.name_shim()
    }
}
impl PerformanceEntryFilterOptions {
    #[doc = "Construct a new `PerformanceEntryFilterOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceEntryFilterOptions`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `entryType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceEntryFilterOptions`*"]
    pub fn entry_type(&mut self, val: &str) -> &mut Self {
        self.set_entry_type_shim(val);
        self
    }
    #[doc = "Change the `initiatorType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceEntryFilterOptions`*"]
    pub fn initiator_type(&mut self, val: &str) -> &mut Self {
        self.set_initiator_type_shim(val);
        self
    }
    #[doc = "Change the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceEntryFilterOptions`*"]
    pub fn name(&mut self, val: &str) -> &mut Self {
        self.set_name_shim(val);
        self
    }
}
impl Default for PerformanceEntryFilterOptions {
    fn default() -> Self {
        Self::new()
    }
}
