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
    #[wasm_bindgen(method, getter = "bubbles")]
    fn bubbles_shim(this: &SubmitEventInit) -> bool;
    #[wasm_bindgen(method, setter = "bubbles")]
    fn set_bubbles_shim(this: &SubmitEventInit, val: bool);
    #[wasm_bindgen(method, getter = "cancelable")]
    fn cancelable_shim(this: &SubmitEventInit) -> bool;
    #[wasm_bindgen(method, setter = "cancelable")]
    fn set_cancelable_shim(this: &SubmitEventInit, val: bool);
    #[wasm_bindgen(method, getter = "composed")]
    fn composed_shim(this: &SubmitEventInit) -> bool;
    #[wasm_bindgen(method, setter = "composed")]
    fn set_composed_shim(this: &SubmitEventInit, val: bool);
    #[cfg(feature = "HtmlElement")]
    #[wasm_bindgen(method, getter = "submitter")]
    fn submitter_shim(this: &SubmitEventInit) -> Option<HtmlElement>;
    #[cfg(feature = "HtmlElement")]
    #[wasm_bindgen(method, setter = "submitter")]
    fn set_submitter_shim(this: &SubmitEventInit, val: Option<&HtmlElement>);
}
#[doc = "The trait to access properties on the `SubmitEventInit` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `SubmitEventInit`*"]
pub trait SubmitEventInitGetters {
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SubmitEventInit`*"]
    fn bubbles(&self) -> bool;
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SubmitEventInit`*"]
    fn cancelable(&self) -> bool;
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SubmitEventInit`*"]
    fn composed(&self) -> bool;
    #[cfg(feature = "HtmlElement")]
    #[doc = "Get the `submitter` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlElement`, `SubmitEventInit`*"]
    fn submitter(&self) -> Option<HtmlElement>;
}
impl SubmitEventInitGetters for SubmitEventInit {
    fn bubbles(&self) -> bool {
        self.bubbles_shim()
    }
    fn cancelable(&self) -> bool {
        self.cancelable_shim()
    }
    fn composed(&self) -> bool {
        self.composed_shim()
    }
    #[cfg(feature = "HtmlElement")]
    fn submitter(&self) -> Option<HtmlElement> {
        self.submitter_shim()
    }
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
        self.set_bubbles_shim(val);
        self
    }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SubmitEventInit`*"]
    pub fn cancelable(&mut self, val: bool) -> &mut Self {
        self.set_cancelable_shim(val);
        self
    }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SubmitEventInit`*"]
    pub fn composed(&mut self, val: bool) -> &mut Self {
        self.set_composed_shim(val);
        self
    }
    #[cfg(feature = "HtmlElement")]
    #[doc = "Change the `submitter` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlElement`, `SubmitEventInit`*"]
    pub fn submitter(&mut self, val: Option<&HtmlElement>) -> &mut Self {
        self.set_submitter_shim(val);
        self
    }
}
impl Default for SubmitEventInit {
    fn default() -> Self {
        Self::new()
    }
}
