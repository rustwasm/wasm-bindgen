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
    #[wasm_bindgen(method, setter = "asyncCause")]
    fn async_cause_shim(this: &ConsoleStackEntry, val: Option<&str>);
    #[wasm_bindgen(method, setter = "columnNumber")]
    fn column_number_shim(this: &ConsoleStackEntry, val: u32);
    #[wasm_bindgen(method, setter = "filename")]
    fn filename_shim(this: &ConsoleStackEntry, val: &str);
    #[wasm_bindgen(method, setter = "functionName")]
    fn function_name_shim(this: &ConsoleStackEntry, val: &str);
    #[wasm_bindgen(method, setter = "lineNumber")]
    fn line_number_shim(this: &ConsoleStackEntry, val: u32);
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
        self.async_cause_shim(val);
        self
    }
    #[doc = "Change the `columnNumber` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleStackEntry`*"]
    pub fn column_number(&mut self, val: u32) -> &mut Self {
        self.column_number_shim(val);
        self
    }
    #[doc = "Change the `filename` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleStackEntry`*"]
    pub fn filename(&mut self, val: &str) -> &mut Self {
        self.filename_shim(val);
        self
    }
    #[doc = "Change the `functionName` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleStackEntry`*"]
    pub fn function_name(&mut self, val: &str) -> &mut Self {
        self.function_name_shim(val);
        self
    }
    #[doc = "Change the `lineNumber` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleStackEntry`*"]
    pub fn line_number(&mut self, val: u32) -> &mut Self {
        self.line_number_shim(val);
        self
    }
}
impl Default for ConsoleStackEntry {
    fn default() -> Self {
        Self::new()
    }
}
