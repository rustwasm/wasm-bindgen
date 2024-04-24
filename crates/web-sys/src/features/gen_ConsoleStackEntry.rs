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
    #[wasm_bindgen(method, getter = "asyncCause")]
    fn async_cause_shim(this: &ConsoleStackEntry) -> Option<&str>;
    #[wasm_bindgen(method, setter = "asyncCause")]
    fn set_async_cause_shim(this: &ConsoleStackEntry, val: Option<&str>);
    #[wasm_bindgen(method, getter = "columnNumber")]
    fn column_number_shim(this: &ConsoleStackEntry) -> u32;
    #[wasm_bindgen(method, setter = "columnNumber")]
    fn set_column_number_shim(this: &ConsoleStackEntry, val: u32);
    #[wasm_bindgen(method, getter = "filename")]
    fn filename_shim(this: &ConsoleStackEntry) -> &str;
    #[wasm_bindgen(method, setter = "filename")]
    fn set_filename_shim(this: &ConsoleStackEntry, val: &str);
    #[wasm_bindgen(method, getter = "functionName")]
    fn function_name_shim(this: &ConsoleStackEntry) -> &str;
    #[wasm_bindgen(method, setter = "functionName")]
    fn set_function_name_shim(this: &ConsoleStackEntry, val: &str);
    #[wasm_bindgen(method, getter = "lineNumber")]
    fn line_number_shim(this: &ConsoleStackEntry) -> u32;
    #[wasm_bindgen(method, setter = "lineNumber")]
    fn set_line_number_shim(this: &ConsoleStackEntry, val: u32);
}
#[doc = "The trait to access properties on the `ConsoleStackEntry` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `ConsoleStackEntry`*"]
pub trait ConsoleStackEntryGetters {
    #[doc = "Get the `asyncCause` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleStackEntry`*"]
    fn async_cause(&self) -> Option<&str>;
    #[doc = "Get the `columnNumber` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleStackEntry`*"]
    fn column_number(&self) -> u32;
    #[doc = "Get the `filename` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleStackEntry`*"]
    fn filename(&self) -> &str;
    #[doc = "Get the `functionName` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleStackEntry`*"]
    fn function_name(&self) -> &str;
    #[doc = "Get the `lineNumber` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleStackEntry`*"]
    fn line_number(&self) -> u32;
}
impl ConsoleStackEntryGetters for ConsoleStackEntry {
    fn async_cause(&self) -> Option<&str> {
        self.async_cause_shim()
    }
    fn column_number(&self) -> u32 {
        self.column_number_shim()
    }
    fn filename(&self) -> &str {
        self.filename_shim()
    }
    fn function_name(&self) -> &str {
        self.function_name_shim()
    }
    fn line_number(&self) -> u32 {
        self.line_number_shim()
    }
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
        self.set_async_cause_shim(val);
        self
    }
    #[doc = "Change the `columnNumber` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleStackEntry`*"]
    pub fn column_number(&mut self, val: u32) -> &mut Self {
        self.set_column_number_shim(val);
        self
    }
    #[doc = "Change the `filename` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleStackEntry`*"]
    pub fn filename(&mut self, val: &str) -> &mut Self {
        self.set_filename_shim(val);
        self
    }
    #[doc = "Change the `functionName` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleStackEntry`*"]
    pub fn function_name(&mut self, val: &str) -> &mut Self {
        self.set_function_name_shim(val);
        self
    }
    #[doc = "Change the `lineNumber` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleStackEntry`*"]
    pub fn line_number(&mut self, val: u32) -> &mut Self {
        self.set_line_number_shim(val);
        self
    }
}
impl Default for ConsoleStackEntry {
    fn default() -> Self {
        Self::new()
    }
}
