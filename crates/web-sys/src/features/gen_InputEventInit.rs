#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = InputEventInit)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `InputEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `InputEventInit`*"]
    pub type InputEventInit;
    #[wasm_bindgen(method, setter = "bubbles")]
    fn bubbles_shim(this: &InputEventInit, val: bool);
    #[wasm_bindgen(method, setter = "cancelable")]
    fn cancelable_shim(this: &InputEventInit, val: bool);
    #[wasm_bindgen(method, setter = "composed")]
    fn composed_shim(this: &InputEventInit, val: bool);
    #[wasm_bindgen(method, setter = "detail")]
    fn detail_shim(this: &InputEventInit, val: i32);
    #[cfg(feature = "Window")]
    #[wasm_bindgen(method, setter = "view")]
    fn view_shim(this: &InputEventInit, val: Option<&Window>);
    #[wasm_bindgen(method, setter = "data")]
    fn data_shim(this: &InputEventInit, val: Option<&str>);
    #[cfg(feature = "DataTransfer")]
    #[wasm_bindgen(method, setter = "dataTransfer")]
    fn data_transfer_shim(this: &InputEventInit, val: Option<&DataTransfer>);
    #[wasm_bindgen(method, setter = "inputType")]
    fn input_type_shim(this: &InputEventInit, val: &str);
    #[wasm_bindgen(method, setter = "isComposing")]
    fn is_composing_shim(this: &InputEventInit, val: bool);
    #[wasm_bindgen(method, setter = "targetRanges")]
    fn target_ranges_shim(this: &InputEventInit, val: &::wasm_bindgen::JsValue);
}
impl InputEventInit {
    #[doc = "Construct a new `InputEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `InputEventInit`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `InputEventInit`*"]
    pub fn bubbles(&mut self, val: bool) -> &mut Self {
        self.bubbles_shim(val);
        self
    }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `InputEventInit`*"]
    pub fn cancelable(&mut self, val: bool) -> &mut Self {
        self.cancelable_shim(val);
        self
    }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `InputEventInit`*"]
    pub fn composed(&mut self, val: bool) -> &mut Self {
        self.composed_shim(val);
        self
    }
    #[doc = "Change the `detail` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `InputEventInit`*"]
    pub fn detail(&mut self, val: i32) -> &mut Self {
        self.detail_shim(val);
        self
    }
    #[cfg(feature = "Window")]
    #[doc = "Change the `view` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `InputEventInit`, `Window`*"]
    pub fn view(&mut self, val: Option<&Window>) -> &mut Self {
        self.view_shim(val);
        self
    }
    #[doc = "Change the `data` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `InputEventInit`*"]
    pub fn data(&mut self, val: Option<&str>) -> &mut Self {
        self.data_shim(val);
        self
    }
    #[cfg(feature = "DataTransfer")]
    #[doc = "Change the `dataTransfer` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DataTransfer`, `InputEventInit`*"]
    pub fn data_transfer(&mut self, val: Option<&DataTransfer>) -> &mut Self {
        self.data_transfer_shim(val);
        self
    }
    #[doc = "Change the `inputType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `InputEventInit`*"]
    pub fn input_type(&mut self, val: &str) -> &mut Self {
        self.input_type_shim(val);
        self
    }
    #[doc = "Change the `isComposing` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `InputEventInit`*"]
    pub fn is_composing(&mut self, val: bool) -> &mut Self {
        self.is_composing_shim(val);
        self
    }
    #[doc = "Change the `targetRanges` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `InputEventInit`*"]
    pub fn target_ranges(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.target_ranges_shim(val);
        self
    }
}
impl Default for InputEventInit {
    fn default() -> Self {
        Self::new()
    }
}
