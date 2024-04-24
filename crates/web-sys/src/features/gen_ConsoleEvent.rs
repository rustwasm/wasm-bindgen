#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = ConsoleEvent)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ConsoleEvent` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleEvent`*"]
    pub type ConsoleEvent;
    #[wasm_bindgen(method, getter = "ID")]
    fn id_shim(this: &ConsoleEvent) -> &::wasm_bindgen::JsValue;
    #[wasm_bindgen(method, setter = "ID")]
    fn set_id_shim(this: &ConsoleEvent, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, getter = "addonId")]
    fn addon_id_shim(this: &ConsoleEvent) -> &str;
    #[wasm_bindgen(method, setter = "addonId")]
    fn set_addon_id_shim(this: &ConsoleEvent, val: &str);
    #[wasm_bindgen(method, getter = "arguments")]
    fn arguments_shim(this: &ConsoleEvent) -> &::wasm_bindgen::JsValue;
    #[wasm_bindgen(method, setter = "arguments")]
    fn set_arguments_shim(this: &ConsoleEvent, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, getter = "columnNumber")]
    fn column_number_shim(this: &ConsoleEvent) -> u32;
    #[wasm_bindgen(method, setter = "columnNumber")]
    fn set_column_number_shim(this: &ConsoleEvent, val: u32);
    #[wasm_bindgen(method, getter = "consoleID")]
    fn console_id_shim(this: &ConsoleEvent) -> &str;
    #[wasm_bindgen(method, setter = "consoleID")]
    fn set_console_id_shim(this: &ConsoleEvent, val: &str);
    #[wasm_bindgen(method, getter = "counter")]
    fn counter_shim(this: &ConsoleEvent) -> &::wasm_bindgen::JsValue;
    #[wasm_bindgen(method, setter = "counter")]
    fn set_counter_shim(this: &ConsoleEvent, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, getter = "filename")]
    fn filename_shim(this: &ConsoleEvent) -> &str;
    #[wasm_bindgen(method, setter = "filename")]
    fn set_filename_shim(this: &ConsoleEvent, val: &str);
    #[wasm_bindgen(method, getter = "functionName")]
    fn function_name_shim(this: &ConsoleEvent) -> &str;
    #[wasm_bindgen(method, setter = "functionName")]
    fn set_function_name_shim(this: &ConsoleEvent, val: &str);
    #[wasm_bindgen(method, getter = "groupName")]
    fn group_name_shim(this: &ConsoleEvent) -> &str;
    #[wasm_bindgen(method, setter = "groupName")]
    fn set_group_name_shim(this: &ConsoleEvent, val: &str);
    #[wasm_bindgen(method, getter = "innerID")]
    fn inner_id_shim(this: &ConsoleEvent) -> &::wasm_bindgen::JsValue;
    #[wasm_bindgen(method, setter = "innerID")]
    fn set_inner_id_shim(this: &ConsoleEvent, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, getter = "level")]
    fn level_shim(this: &ConsoleEvent) -> &str;
    #[wasm_bindgen(method, setter = "level")]
    fn set_level_shim(this: &ConsoleEvent, val: &str);
    #[wasm_bindgen(method, getter = "lineNumber")]
    fn line_number_shim(this: &ConsoleEvent) -> u32;
    #[wasm_bindgen(method, setter = "lineNumber")]
    fn set_line_number_shim(this: &ConsoleEvent, val: u32);
    #[wasm_bindgen(method, getter = "prefix")]
    fn prefix_shim(this: &ConsoleEvent) -> &str;
    #[wasm_bindgen(method, setter = "prefix")]
    fn set_prefix_shim(this: &ConsoleEvent, val: &str);
    #[wasm_bindgen(method, getter = "private")]
    fn private_shim(this: &ConsoleEvent) -> bool;
    #[wasm_bindgen(method, setter = "private")]
    fn set_private_shim(this: &ConsoleEvent, val: bool);
    #[wasm_bindgen(method, getter = "styles")]
    fn styles_shim(this: &ConsoleEvent) -> &::wasm_bindgen::JsValue;
    #[wasm_bindgen(method, setter = "styles")]
    fn set_styles_shim(this: &ConsoleEvent, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, getter = "timeStamp")]
    fn time_stamp_shim(this: &ConsoleEvent) -> f64;
    #[wasm_bindgen(method, setter = "timeStamp")]
    fn set_time_stamp_shim(this: &ConsoleEvent, val: f64);
    #[wasm_bindgen(method, getter = "timer")]
    fn timer_shim(this: &ConsoleEvent) -> &::wasm_bindgen::JsValue;
    #[wasm_bindgen(method, setter = "timer")]
    fn set_timer_shim(this: &ConsoleEvent, val: &::wasm_bindgen::JsValue);
}
#[doc = "The trait to access properties on the `ConsoleEvent` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `ConsoleEvent`*"]
pub trait ConsoleEventGetters {
    #[doc = "Get the `ID` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleEvent`*"]
    fn id(&self) -> &::wasm_bindgen::JsValue;
    #[doc = "Get the `addonId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleEvent`*"]
    fn addon_id(&self) -> &str;
    #[doc = "Get the `arguments` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleEvent`*"]
    fn arguments(&self) -> &::wasm_bindgen::JsValue;
    #[doc = "Get the `columnNumber` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleEvent`*"]
    fn column_number(&self) -> u32;
    #[doc = "Get the `consoleID` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleEvent`*"]
    fn console_id(&self) -> &str;
    #[doc = "Get the `counter` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleEvent`*"]
    fn counter(&self) -> &::wasm_bindgen::JsValue;
    #[doc = "Get the `filename` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleEvent`*"]
    fn filename(&self) -> &str;
    #[doc = "Get the `functionName` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleEvent`*"]
    fn function_name(&self) -> &str;
    #[doc = "Get the `groupName` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleEvent`*"]
    fn group_name(&self) -> &str;
    #[doc = "Get the `innerID` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleEvent`*"]
    fn inner_id(&self) -> &::wasm_bindgen::JsValue;
    #[doc = "Get the `level` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleEvent`*"]
    fn level(&self) -> &str;
    #[doc = "Get the `lineNumber` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleEvent`*"]
    fn line_number(&self) -> u32;
    #[doc = "Get the `prefix` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleEvent`*"]
    fn prefix(&self) -> &str;
    #[doc = "Get the `private` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleEvent`*"]
    fn private(&self) -> bool;
    #[doc = "Get the `styles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleEvent`*"]
    fn styles(&self) -> &::wasm_bindgen::JsValue;
    #[doc = "Get the `timeStamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleEvent`*"]
    fn time_stamp(&self) -> f64;
    #[doc = "Get the `timer` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleEvent`*"]
    fn timer(&self) -> &::wasm_bindgen::JsValue;
}
impl ConsoleEventGetters for ConsoleEvent {
    fn id(&self) -> &::wasm_bindgen::JsValue {
        self.id_shim()
    }
    fn addon_id(&self) -> &str {
        self.addon_id_shim()
    }
    fn arguments(&self) -> &::wasm_bindgen::JsValue {
        self.arguments_shim()
    }
    fn column_number(&self) -> u32 {
        self.column_number_shim()
    }
    fn console_id(&self) -> &str {
        self.console_id_shim()
    }
    fn counter(&self) -> &::wasm_bindgen::JsValue {
        self.counter_shim()
    }
    fn filename(&self) -> &str {
        self.filename_shim()
    }
    fn function_name(&self) -> &str {
        self.function_name_shim()
    }
    fn group_name(&self) -> &str {
        self.group_name_shim()
    }
    fn inner_id(&self) -> &::wasm_bindgen::JsValue {
        self.inner_id_shim()
    }
    fn level(&self) -> &str {
        self.level_shim()
    }
    fn line_number(&self) -> u32 {
        self.line_number_shim()
    }
    fn prefix(&self) -> &str {
        self.prefix_shim()
    }
    fn private(&self) -> bool {
        self.private_shim()
    }
    fn styles(&self) -> &::wasm_bindgen::JsValue {
        self.styles_shim()
    }
    fn time_stamp(&self) -> f64 {
        self.time_stamp_shim()
    }
    fn timer(&self) -> &::wasm_bindgen::JsValue {
        self.timer_shim()
    }
}
impl ConsoleEvent {
    #[doc = "Construct a new `ConsoleEvent`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleEvent`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `ID` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleEvent`*"]
    pub fn id(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_id_shim(val);
        self
    }
    #[doc = "Change the `addonId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleEvent`*"]
    pub fn addon_id(&mut self, val: &str) -> &mut Self {
        self.set_addon_id_shim(val);
        self
    }
    #[doc = "Change the `arguments` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleEvent`*"]
    pub fn arguments(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_arguments_shim(val);
        self
    }
    #[doc = "Change the `columnNumber` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleEvent`*"]
    pub fn column_number(&mut self, val: u32) -> &mut Self {
        self.set_column_number_shim(val);
        self
    }
    #[doc = "Change the `consoleID` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleEvent`*"]
    pub fn console_id(&mut self, val: &str) -> &mut Self {
        self.set_console_id_shim(val);
        self
    }
    #[doc = "Change the `counter` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleEvent`*"]
    pub fn counter(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_counter_shim(val);
        self
    }
    #[doc = "Change the `filename` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleEvent`*"]
    pub fn filename(&mut self, val: &str) -> &mut Self {
        self.set_filename_shim(val);
        self
    }
    #[doc = "Change the `functionName` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleEvent`*"]
    pub fn function_name(&mut self, val: &str) -> &mut Self {
        self.set_function_name_shim(val);
        self
    }
    #[doc = "Change the `groupName` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleEvent`*"]
    pub fn group_name(&mut self, val: &str) -> &mut Self {
        self.set_group_name_shim(val);
        self
    }
    #[doc = "Change the `innerID` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleEvent`*"]
    pub fn inner_id(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_inner_id_shim(val);
        self
    }
    #[doc = "Change the `level` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleEvent`*"]
    pub fn level(&mut self, val: &str) -> &mut Self {
        self.set_level_shim(val);
        self
    }
    #[doc = "Change the `lineNumber` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleEvent`*"]
    pub fn line_number(&mut self, val: u32) -> &mut Self {
        self.set_line_number_shim(val);
        self
    }
    #[doc = "Change the `prefix` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleEvent`*"]
    pub fn prefix(&mut self, val: &str) -> &mut Self {
        self.set_prefix_shim(val);
        self
    }
    #[doc = "Change the `private` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleEvent`*"]
    pub fn private(&mut self, val: bool) -> &mut Self {
        self.set_private_shim(val);
        self
    }
    #[doc = "Change the `styles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleEvent`*"]
    pub fn styles(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_styles_shim(val);
        self
    }
    #[doc = "Change the `timeStamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleEvent`*"]
    pub fn time_stamp(&mut self, val: f64) -> &mut Self {
        self.set_time_stamp_shim(val);
        self
    }
    #[doc = "Change the `timer` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleEvent`*"]
    pub fn timer(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_timer_shim(val);
        self
    }
}
impl Default for ConsoleEvent {
    fn default() -> Self {
        Self::new()
    }
}
