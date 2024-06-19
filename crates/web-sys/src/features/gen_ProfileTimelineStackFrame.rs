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
    #[doc = "Get the `asyncCause` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineStackFrame`*"]
    #[wasm_bindgen(method, getter = "asyncCause")]
    pub fn get_async_cause(this: &ProfileTimelineStackFrame) -> Option<String>;
    #[wasm_bindgen(method, setter = "asyncCause")]
    fn set_async_cause(this: &ProfileTimelineStackFrame, val: &str);
    #[doc = "Get the `asyncParent` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineStackFrame`*"]
    #[wasm_bindgen(method, getter = "asyncParent")]
    pub fn get_async_parent(this: &ProfileTimelineStackFrame) -> Option<::js_sys::Object>;
    #[wasm_bindgen(method, setter = "asyncParent")]
    fn set_async_parent(this: &ProfileTimelineStackFrame, val: Option<&::js_sys::Object>);
    #[doc = "Get the `column` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineStackFrame`*"]
    #[wasm_bindgen(method, getter = "column")]
    pub fn get_column(this: &ProfileTimelineStackFrame) -> Option<i32>;
    #[wasm_bindgen(method, setter = "column")]
    fn set_column(this: &ProfileTimelineStackFrame, val: i32);
    #[doc = "Get the `functionDisplayName` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineStackFrame`*"]
    #[wasm_bindgen(method, getter = "functionDisplayName")]
    pub fn get_function_display_name(this: &ProfileTimelineStackFrame) -> Option<String>;
    #[wasm_bindgen(method, setter = "functionDisplayName")]
    fn set_function_display_name(this: &ProfileTimelineStackFrame, val: &str);
    #[doc = "Get the `line` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineStackFrame`*"]
    #[wasm_bindgen(method, getter = "line")]
    pub fn get_line(this: &ProfileTimelineStackFrame) -> Option<i32>;
    #[wasm_bindgen(method, setter = "line")]
    fn set_line(this: &ProfileTimelineStackFrame, val: i32);
    #[doc = "Get the `parent` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineStackFrame`*"]
    #[wasm_bindgen(method, getter = "parent")]
    pub fn get_parent(this: &ProfileTimelineStackFrame) -> Option<::js_sys::Object>;
    #[wasm_bindgen(method, setter = "parent")]
    fn set_parent(this: &ProfileTimelineStackFrame, val: Option<&::js_sys::Object>);
    #[doc = "Get the `source` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineStackFrame`*"]
    #[wasm_bindgen(method, getter = "source")]
    pub fn get_source(this: &ProfileTimelineStackFrame) -> Option<String>;
    #[wasm_bindgen(method, setter = "source")]
    fn set_source(this: &ProfileTimelineStackFrame, val: &str);
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
        self.set_async_cause(val);
        self
    }
    #[doc = "Change the `asyncParent` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineStackFrame`*"]
    pub fn async_parent(&mut self, val: Option<&::js_sys::Object>) -> &mut Self {
        self.set_async_parent(val);
        self
    }
    #[doc = "Change the `column` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineStackFrame`*"]
    pub fn column(&mut self, val: i32) -> &mut Self {
        self.set_column(val);
        self
    }
    #[doc = "Change the `functionDisplayName` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineStackFrame`*"]
    pub fn function_display_name(&mut self, val: &str) -> &mut Self {
        self.set_function_display_name(val);
        self
    }
    #[doc = "Change the `line` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineStackFrame`*"]
    pub fn line(&mut self, val: i32) -> &mut Self {
        self.set_line(val);
        self
    }
    #[doc = "Change the `parent` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineStackFrame`*"]
    pub fn parent(&mut self, val: Option<&::js_sys::Object>) -> &mut Self {
        self.set_parent(val);
        self
    }
    #[doc = "Change the `source` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineStackFrame`*"]
    pub fn source(&mut self, val: &str) -> &mut Self {
        self.set_source(val);
        self
    }
}
impl Default for ProfileTimelineStackFrame {
    fn default() -> Self {
        Self::new()
    }
}
