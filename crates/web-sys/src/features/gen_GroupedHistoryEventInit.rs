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
    #[wasm_bindgen(method, getter = "bubbles")]
    fn bubbles_shim(this: &GroupedHistoryEventInit) -> bool;
    #[wasm_bindgen(method, setter = "bubbles")]
    fn set_bubbles_shim(this: &GroupedHistoryEventInit, val: bool);
    #[wasm_bindgen(method, getter = "cancelable")]
    fn cancelable_shim(this: &GroupedHistoryEventInit) -> bool;
    #[wasm_bindgen(method, setter = "cancelable")]
    fn set_cancelable_shim(this: &GroupedHistoryEventInit, val: bool);
    #[wasm_bindgen(method, getter = "composed")]
    fn composed_shim(this: &GroupedHistoryEventInit) -> bool;
    #[wasm_bindgen(method, setter = "composed")]
    fn set_composed_shim(this: &GroupedHistoryEventInit, val: bool);
    #[cfg(feature = "Element")]
    #[wasm_bindgen(method, getter = "otherBrowser")]
    fn other_browser_shim(this: &GroupedHistoryEventInit) -> Option<Element>;
    #[cfg(feature = "Element")]
    #[wasm_bindgen(method, setter = "otherBrowser")]
    fn set_other_browser_shim(this: &GroupedHistoryEventInit, val: Option<&Element>);
}
#[doc = "The trait to access properties on the `GroupedHistoryEventInit` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `GroupedHistoryEventInit`*"]
pub trait GroupedHistoryEventInitGetters {
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GroupedHistoryEventInit`*"]
    fn bubbles(&self) -> bool;
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GroupedHistoryEventInit`*"]
    fn cancelable(&self) -> bool;
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GroupedHistoryEventInit`*"]
    fn composed(&self) -> bool;
    #[cfg(feature = "Element")]
    #[doc = "Get the `otherBrowser` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`, `GroupedHistoryEventInit`*"]
    fn other_browser(&self) -> Option<Element>;
}
impl GroupedHistoryEventInitGetters for GroupedHistoryEventInit {
    fn bubbles(&self) -> bool {
        self.bubbles_shim()
    }
    fn cancelable(&self) -> bool {
        self.cancelable_shim()
    }
    fn composed(&self) -> bool {
        self.composed_shim()
    }
    #[cfg(feature = "Element")]
    fn other_browser(&self) -> Option<Element> {
        self.other_browser_shim()
    }
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
        self.set_bubbles_shim(val);
        self
    }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GroupedHistoryEventInit`*"]
    pub fn cancelable(&mut self, val: bool) -> &mut Self {
        self.set_cancelable_shim(val);
        self
    }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GroupedHistoryEventInit`*"]
    pub fn composed(&mut self, val: bool) -> &mut Self {
        self.set_composed_shim(val);
        self
    }
    #[cfg(feature = "Element")]
    #[doc = "Change the `otherBrowser` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`, `GroupedHistoryEventInit`*"]
    pub fn other_browser(&mut self, val: Option<&Element>) -> &mut Self {
        self.set_other_browser_shim(val);
        self
    }
}
impl Default for GroupedHistoryEventInit {
    fn default() -> Self {
        Self::new()
    }
}
