#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = ProfileTimelineStackFrame)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ProfileTimelineStackFrame` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineStackFrame`*"]
    pub type ProfileTimelineStackFrame;
    #[wasm_bindgen(method, getter = "asyncCause")]
    fn async_cause_shim(this: &ProfileTimelineStackFrame) -> &str;
    #[wasm_bindgen(method, setter = "asyncCause")]
    fn set_async_cause_shim(this: &ProfileTimelineStackFrame, val: &str);
    #[wasm_bindgen(method, getter = "asyncParent")]
    fn async_parent_shim(this: &ProfileTimelineStackFrame) -> Option<&::js_sys::Object>;
    #[wasm_bindgen(method, setter = "asyncParent")]
    fn set_async_parent_shim(this: &ProfileTimelineStackFrame, val: Option<&::js_sys::Object>);
    #[wasm_bindgen(method, getter = "column")]
    fn column_shim(this: &ProfileTimelineStackFrame) -> i32;
    #[wasm_bindgen(method, setter = "column")]
    fn set_column_shim(this: &ProfileTimelineStackFrame, val: i32);
    #[wasm_bindgen(method, getter = "functionDisplayName")]
    fn function_display_name_shim(this: &ProfileTimelineStackFrame) -> &str;
    #[wasm_bindgen(method, setter = "functionDisplayName")]
    fn set_function_display_name_shim(this: &ProfileTimelineStackFrame, val: &str);
    #[wasm_bindgen(method, getter = "line")]
    fn line_shim(this: &ProfileTimelineStackFrame) -> i32;
    #[wasm_bindgen(method, setter = "line")]
    fn set_line_shim(this: &ProfileTimelineStackFrame, val: i32);
    #[wasm_bindgen(method, getter = "parent")]
    fn parent_shim(this: &ProfileTimelineStackFrame) -> Option<&::js_sys::Object>;
    #[wasm_bindgen(method, setter = "parent")]
    fn set_parent_shim(this: &ProfileTimelineStackFrame, val: Option<&::js_sys::Object>);
    #[wasm_bindgen(method, getter = "source")]
    fn source_shim(this: &ProfileTimelineStackFrame) -> &str;
    #[wasm_bindgen(method, setter = "source")]
    fn set_source_shim(this: &ProfileTimelineStackFrame, val: &str);
}
#[doc = "The trait to access properties on the `ProfileTimelineStackFrame` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `ProfileTimelineStackFrame`*"]
pub trait ProfileTimelineStackFrameGetters {
    #[doc = "Get the `asyncCause` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineStackFrame`*"]
    fn async_cause(&self) -> &str;
    #[doc = "Get the `asyncParent` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineStackFrame`*"]
    fn async_parent(&self) -> Option<&::js_sys::Object>;
    #[doc = "Get the `column` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineStackFrame`*"]
    fn column(&self) -> i32;
    #[doc = "Get the `functionDisplayName` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineStackFrame`*"]
    fn function_display_name(&self) -> &str;
    #[doc = "Get the `line` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineStackFrame`*"]
    fn line(&self) -> i32;
    #[doc = "Get the `parent` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineStackFrame`*"]
    fn parent(&self) -> Option<&::js_sys::Object>;
    #[doc = "Get the `source` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineStackFrame`*"]
    fn source(&self) -> &str;
}
impl ProfileTimelineStackFrameGetters for ProfileTimelineStackFrame {
    fn async_cause(&self) -> &str {
        self.async_cause_shim()
    }
    fn async_parent(&self) -> Option<&::js_sys::Object> {
        self.async_parent_shim()
    }
    fn column(&self) -> i32 {
        self.column_shim()
    }
    fn function_display_name(&self) -> &str {
        self.function_display_name_shim()
    }
    fn line(&self) -> i32 {
        self.line_shim()
    }
    fn parent(&self) -> Option<&::js_sys::Object> {
        self.parent_shim()
    }
    fn source(&self) -> &str {
        self.source_shim()
    }
}
impl ProfileTimelineStackFrame {
    #[doc = "Construct a new `ProfileTimelineStackFrame`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineStackFrame`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `asyncCause` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineStackFrame`*"]
    pub fn async_cause(&mut self, val: &str) -> &mut Self {
        self.set_async_cause_shim(val);
        self
    }
    #[doc = "Change the `asyncParent` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineStackFrame`*"]
    pub fn async_parent(&mut self, val: Option<&::js_sys::Object>) -> &mut Self {
        self.set_async_parent_shim(val);
        self
    }
    #[doc = "Change the `column` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineStackFrame`*"]
    pub fn column(&mut self, val: i32) -> &mut Self {
        self.set_column_shim(val);
        self
    }
    #[doc = "Change the `functionDisplayName` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineStackFrame`*"]
    pub fn function_display_name(&mut self, val: &str) -> &mut Self {
        self.set_function_display_name_shim(val);
        self
    }
    #[doc = "Change the `line` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineStackFrame`*"]
    pub fn line(&mut self, val: i32) -> &mut Self {
        self.set_line_shim(val);
        self
    }
    #[doc = "Change the `parent` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineStackFrame`*"]
    pub fn parent(&mut self, val: Option<&::js_sys::Object>) -> &mut Self {
        self.set_parent_shim(val);
        self
    }
    #[doc = "Change the `source` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineStackFrame`*"]
    pub fn source(&mut self, val: &str) -> &mut Self {
        self.set_source_shim(val);
        self
    }
}
impl Default for ProfileTimelineStackFrame {
    fn default() -> Self {
        Self::new()
    }
}
