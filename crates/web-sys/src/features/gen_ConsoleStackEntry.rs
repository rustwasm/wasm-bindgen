#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = ConsoleStackEntry)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ConsoleStackEntry` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleStackEntry`*"]
    pub type ConsoleStackEntry;
    #[doc = "Get the `asyncCause` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleStackEntry`*"]
    #[wasm_bindgen(method, getter = "asyncCause")]
    pub fn get_async_cause(this: &ConsoleStackEntry) -> Option<String>;
    #[wasm_bindgen(method, setter = "asyncCause")]
    fn set_async_cause(this: &ConsoleStackEntry, val: Option<&str>);
    #[doc = "Get the `columnNumber` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleStackEntry`*"]
    #[wasm_bindgen(method, getter = "columnNumber")]
    pub fn get_column_number(this: &ConsoleStackEntry) -> Option<u32>;
    #[wasm_bindgen(method, setter = "columnNumber")]
    fn set_column_number(this: &ConsoleStackEntry, val: u32);
    #[doc = "Get the `filename` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleStackEntry`*"]
    #[wasm_bindgen(method, getter = "filename")]
    pub fn get_filename(this: &ConsoleStackEntry) -> Option<String>;
    #[wasm_bindgen(method, setter = "filename")]
    fn set_filename(this: &ConsoleStackEntry, val: &str);
    #[doc = "Get the `functionName` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleStackEntry`*"]
    #[wasm_bindgen(method, getter = "functionName")]
    pub fn get_function_name(this: &ConsoleStackEntry) -> Option<String>;
    #[wasm_bindgen(method, setter = "functionName")]
    fn set_function_name(this: &ConsoleStackEntry, val: &str);
    #[doc = "Get the `lineNumber` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleStackEntry`*"]
    #[wasm_bindgen(method, getter = "lineNumber")]
    pub fn get_line_number(this: &ConsoleStackEntry) -> Option<u32>;
    #[wasm_bindgen(method, setter = "lineNumber")]
    fn set_line_number(this: &ConsoleStackEntry, val: u32);
}
impl ConsoleStackEntry {
    #[doc = "Construct a new `ConsoleStackEntry`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleStackEntry`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `asyncCause` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleStackEntry`*"]
    pub fn async_cause(&mut self, val: Option<&str>) -> &mut Self {
        self.set_async_cause(val);
        self
    }
    #[doc = "Change the `columnNumber` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleStackEntry`*"]
    pub fn column_number(&mut self, val: u32) -> &mut Self {
        self.set_column_number(val);
        self
    }
    #[doc = "Change the `filename` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleStackEntry`*"]
    pub fn filename(&mut self, val: &str) -> &mut Self {
        self.set_filename(val);
        self
    }
    #[doc = "Change the `functionName` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleStackEntry`*"]
    pub fn function_name(&mut self, val: &str) -> &mut Self {
        self.set_function_name(val);
        self
    }
    #[doc = "Change the `lineNumber` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleStackEntry`*"]
    pub fn line_number(&mut self, val: u32) -> &mut Self {
        self.set_line_number(val);
        self
    }
}
impl Default for ConsoleStackEntry {
    fn default() -> Self {
        Self::new()
    }
}
