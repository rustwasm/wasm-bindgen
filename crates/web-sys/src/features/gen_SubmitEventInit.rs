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
    #[wasm_bindgen(method, setter = "bubbles")]
    fn bubbles_shim(this: &SubmitEventInit, val: bool);
    #[wasm_bindgen(method, setter = "cancelable")]
    fn cancelable_shim(this: &SubmitEventInit, val: bool);
    #[wasm_bindgen(method, setter = "composed")]
    fn composed_shim(this: &SubmitEventInit, val: bool);
    #[cfg(feature = "HtmlElement")]
    #[wasm_bindgen(method, setter = "submitter")]
    fn submitter_shim(this: &SubmitEventInit, val: Option<&HtmlElement>);
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
        self.bubbles_shim(val);
        self
    }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SubmitEventInit`*"]
    pub fn cancelable(&mut self, val: bool) -> &mut Self {
        self.cancelable_shim(val);
        self
    }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SubmitEventInit`*"]
    pub fn composed(&mut self, val: bool) -> &mut Self {
        self.composed_shim(val);
        self
    }
    #[cfg(feature = "HtmlElement")]
    #[doc = "Change the `submitter` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlElement`, `SubmitEventInit`*"]
    pub fn submitter(&mut self, val: Option<&HtmlElement>) -> &mut Self {
        self.submitter_shim(val);
        self
    }
}
impl Default for SubmitEventInit {
    fn default() -> Self {
        Self::new()
    }
}
