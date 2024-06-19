#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = SubmitEventInit)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SubmitEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SubmitEventInit`*"]
    pub type SubmitEventInit;
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SubmitEventInit`*"]
    #[wasm_bindgen(method, getter = "bubbles")]
    pub fn get_bubbles(this: &SubmitEventInit) -> Option<bool>;
    #[wasm_bindgen(method, setter = "bubbles")]
    fn set_bubbles(this: &SubmitEventInit, val: bool);
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SubmitEventInit`*"]
    #[wasm_bindgen(method, getter = "cancelable")]
    pub fn get_cancelable(this: &SubmitEventInit) -> Option<bool>;
    #[wasm_bindgen(method, setter = "cancelable")]
    fn set_cancelable(this: &SubmitEventInit, val: bool);
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SubmitEventInit`*"]
    #[wasm_bindgen(method, getter = "composed")]
    pub fn get_composed(this: &SubmitEventInit) -> Option<bool>;
    #[wasm_bindgen(method, setter = "composed")]
    fn set_composed(this: &SubmitEventInit, val: bool);
    #[cfg(feature = "HtmlElement")]
    #[doc = "Get the `submitter` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlElement`, `SubmitEventInit`*"]
    #[wasm_bindgen(method, getter = "submitter")]
    pub fn get_submitter(this: &SubmitEventInit) -> Option<HtmlElement>;
    #[cfg(feature = "HtmlElement")]
    #[wasm_bindgen(method, setter = "submitter")]
    fn set_submitter(this: &SubmitEventInit, val: Option<&HtmlElement>);
}
impl SubmitEventInit {
    #[doc = "Construct a new `SubmitEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SubmitEventInit`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SubmitEventInit`*"]
    pub fn bubbles(&mut self, val: bool) -> &mut Self {
        self.set_bubbles(val);
        self
    }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SubmitEventInit`*"]
    pub fn cancelable(&mut self, val: bool) -> &mut Self {
        self.set_cancelable(val);
        self
    }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SubmitEventInit`*"]
    pub fn composed(&mut self, val: bool) -> &mut Self {
        self.set_composed(val);
        self
    }
    #[cfg(feature = "HtmlElement")]
    #[doc = "Change the `submitter` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlElement`, `SubmitEventInit`*"]
    pub fn submitter(&mut self, val: Option<&HtmlElement>) -> &mut Self {
        self.set_submitter(val);
        self
    }
}
impl Default for SubmitEventInit {
    fn default() -> Self {
        Self::new()
    }
}
