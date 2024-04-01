#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = PageTransitionEventInit)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PageTransitionEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PageTransitionEventInit`*"]
    pub type PageTransitionEventInit;
    #[wasm_bindgen(method, setter = "bubbles")]
    fn bubbles_shim(this: &PageTransitionEventInit, val: bool);
    #[wasm_bindgen(method, setter = "cancelable")]
    fn cancelable_shim(this: &PageTransitionEventInit, val: bool);
    #[wasm_bindgen(method, setter = "composed")]
    fn composed_shim(this: &PageTransitionEventInit, val: bool);
    #[wasm_bindgen(method, setter = "inFrameSwap")]
    fn in_frame_swap_shim(this: &PageTransitionEventInit, val: bool);
    #[wasm_bindgen(method, setter = "persisted")]
    fn persisted_shim(this: &PageTransitionEventInit, val: bool);
}
impl PageTransitionEventInit {
    #[doc = "Construct a new `PageTransitionEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PageTransitionEventInit`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PageTransitionEventInit`*"]
    pub fn bubbles(&mut self, val: bool) -> &mut Self {
        self.bubbles_shim(val);
        self
    }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PageTransitionEventInit`*"]
    pub fn cancelable(&mut self, val: bool) -> &mut Self {
        self.cancelable_shim(val);
        self
    }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PageTransitionEventInit`*"]
    pub fn composed(&mut self, val: bool) -> &mut Self {
        self.composed_shim(val);
        self
    }
    #[doc = "Change the `inFrameSwap` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PageTransitionEventInit`*"]
    pub fn in_frame_swap(&mut self, val: bool) -> &mut Self {
        self.in_frame_swap_shim(val);
        self
    }
    #[doc = "Change the `persisted` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PageTransitionEventInit`*"]
    pub fn persisted(&mut self, val: bool) -> &mut Self {
        self.persisted_shim(val);
        self
    }
}
impl Default for PageTransitionEventInit {
    fn default() -> Self {
        Self::new()
    }
}
