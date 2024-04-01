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
    #[wasm_bindgen(method, setter = "ID")]
    fn id_shim(this: &ConsoleEvent, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, setter = "addonId")]
    fn addon_id_shim(this: &ConsoleEvent, val: &str);
    #[wasm_bindgen(method, setter = "arguments")]
    fn arguments_shim(this: &ConsoleEvent, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, setter = "columnNumber")]
    fn column_number_shim(this: &ConsoleEvent, val: u32);
    #[wasm_bindgen(method, setter = "consoleID")]
    fn console_id_shim(this: &ConsoleEvent, val: &str);
    #[wasm_bindgen(method, setter = "counter")]
    fn counter_shim(this: &ConsoleEvent, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, setter = "filename")]
    fn filename_shim(this: &ConsoleEvent, val: &str);
    #[wasm_bindgen(method, setter = "functionName")]
    fn function_name_shim(this: &ConsoleEvent, val: &str);
    #[wasm_bindgen(method, setter = "groupName")]
    fn group_name_shim(this: &ConsoleEvent, val: &str);
    #[wasm_bindgen(method, setter = "innerID")]
    fn inner_id_shim(this: &ConsoleEvent, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, setter = "level")]
    fn level_shim(this: &ConsoleEvent, val: &str);
    #[wasm_bindgen(method, setter = "lineNumber")]
    fn line_number_shim(this: &ConsoleEvent, val: u32);
    #[wasm_bindgen(method, setter = "prefix")]
    fn prefix_shim(this: &ConsoleEvent, val: &str);
    #[wasm_bindgen(method, setter = "private")]
    fn private_shim(this: &ConsoleEvent, val: bool);
    #[wasm_bindgen(method, setter = "styles")]
    fn styles_shim(this: &ConsoleEvent, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, setter = "timeStamp")]
    fn time_stamp_shim(this: &ConsoleEvent, val: f64);
    #[wasm_bindgen(method, setter = "timer")]
    fn timer_shim(this: &ConsoleEvent, val: &::wasm_bindgen::JsValue);
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
        self.id_shim(val);
        self
    }
    #[doc = "Change the `addonId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleEvent`*"]
    pub fn addon_id(&mut self, val: &str) -> &mut Self {
        self.addon_id_shim(val);
        self
    }
    #[doc = "Change the `arguments` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleEvent`*"]
    pub fn arguments(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.arguments_shim(val);
        self
    }
    #[doc = "Change the `columnNumber` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleEvent`*"]
    pub fn column_number(&mut self, val: u32) -> &mut Self {
        self.column_number_shim(val);
        self
    }
    #[doc = "Change the `consoleID` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleEvent`*"]
    pub fn console_id(&mut self, val: &str) -> &mut Self {
        self.console_id_shim(val);
        self
    }
    #[doc = "Change the `counter` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleEvent`*"]
    pub fn counter(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.counter_shim(val);
        self
    }
    #[doc = "Change the `filename` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleEvent`*"]
    pub fn filename(&mut self, val: &str) -> &mut Self {
        self.filename_shim(val);
        self
    }
    #[doc = "Change the `functionName` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleEvent`*"]
    pub fn function_name(&mut self, val: &str) -> &mut Self {
        self.function_name_shim(val);
        self
    }
    #[doc = "Change the `groupName` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleEvent`*"]
    pub fn group_name(&mut self, val: &str) -> &mut Self {
        self.group_name_shim(val);
        self
    }
    #[doc = "Change the `innerID` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleEvent`*"]
    pub fn inner_id(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.inner_id_shim(val);
        self
    }
    #[doc = "Change the `level` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleEvent`*"]
    pub fn level(&mut self, val: &str) -> &mut Self {
        self.level_shim(val);
        self
    }
    #[doc = "Change the `lineNumber` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleEvent`*"]
    pub fn line_number(&mut self, val: u32) -> &mut Self {
        self.line_number_shim(val);
        self
    }
    #[doc = "Change the `prefix` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleEvent`*"]
    pub fn prefix(&mut self, val: &str) -> &mut Self {
        self.prefix_shim(val);
        self
    }
    #[doc = "Change the `private` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleEvent`*"]
    pub fn private(&mut self, val: bool) -> &mut Self {
        self.private_shim(val);
        self
    }
    #[doc = "Change the `styles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleEvent`*"]
    pub fn styles(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.styles_shim(val);
        self
    }
    #[doc = "Change the `timeStamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleEvent`*"]
    pub fn time_stamp(&mut self, val: f64) -> &mut Self {
        self.time_stamp_shim(val);
        self
    }
    #[doc = "Change the `timer` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleEvent`*"]
    pub fn timer(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.timer_shim(val);
        self
    }
}
impl Default for ConsoleEvent {
    fn default() -> Self {
        Self::new()
    }
}
