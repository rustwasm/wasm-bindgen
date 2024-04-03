#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = GroupedHistoryEventInit)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GroupedHistoryEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GroupedHistoryEventInit`*"]
    pub type GroupedHistoryEventInit;
    #[wasm_bindgen(method, setter = "bubbles")]
    fn bubbles_shim(this: &GroupedHistoryEventInit, val: bool);
    #[wasm_bindgen(method, setter = "cancelable")]
    fn cancelable_shim(this: &GroupedHistoryEventInit, val: bool);
    #[wasm_bindgen(method, setter = "composed")]
    fn composed_shim(this: &GroupedHistoryEventInit, val: bool);
    #[cfg(feature = "Element")]
    #[wasm_bindgen(method, setter = "otherBrowser")]
    fn other_browser_shim(this: &GroupedHistoryEventInit, val: Option<&Element>);
}
impl GroupedHistoryEventInit {
    #[doc = "Construct a new `GroupedHistoryEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GroupedHistoryEventInit`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GroupedHistoryEventInit`*"]
    pub fn bubbles(&mut self, val: bool) -> &mut Self {
        self.bubbles_shim(val);
        self
    }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GroupedHistoryEventInit`*"]
    pub fn cancelable(&mut self, val: bool) -> &mut Self {
        self.cancelable_shim(val);
        self
    }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GroupedHistoryEventInit`*"]
    pub fn composed(&mut self, val: bool) -> &mut Self {
        self.composed_shim(val);
        self
    }
    #[cfg(feature = "Element")]
    #[doc = "Change the `otherBrowser` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`, `GroupedHistoryEventInit`*"]
    pub fn other_browser(&mut self, val: Option<&Element>) -> &mut Self {
        self.other_browser_shim(val);
        self
    }
}
impl Default for GroupedHistoryEventInit {
    fn default() -> Self {
        Self::new()
    }
}
