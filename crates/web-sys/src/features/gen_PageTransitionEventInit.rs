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
    #[wasm_bindgen(method, getter = "bubbles")]
    fn bubbles_shim(this: &PageTransitionEventInit) -> bool;
    #[wasm_bindgen(method, setter = "bubbles")]
    fn set_bubbles_shim(this: &PageTransitionEventInit, val: bool);
    #[wasm_bindgen(method, getter = "cancelable")]
    fn cancelable_shim(this: &PageTransitionEventInit) -> bool;
    #[wasm_bindgen(method, setter = "cancelable")]
    fn set_cancelable_shim(this: &PageTransitionEventInit, val: bool);
    #[wasm_bindgen(method, getter = "composed")]
    fn composed_shim(this: &PageTransitionEventInit) -> bool;
    #[wasm_bindgen(method, setter = "composed")]
    fn set_composed_shim(this: &PageTransitionEventInit, val: bool);
    #[wasm_bindgen(method, getter = "inFrameSwap")]
    fn in_frame_swap_shim(this: &PageTransitionEventInit) -> bool;
    #[wasm_bindgen(method, setter = "inFrameSwap")]
    fn set_in_frame_swap_shim(this: &PageTransitionEventInit, val: bool);
    #[wasm_bindgen(method, getter = "persisted")]
    fn persisted_shim(this: &PageTransitionEventInit) -> bool;
    #[wasm_bindgen(method, setter = "persisted")]
    fn set_persisted_shim(this: &PageTransitionEventInit, val: bool);
}
#[doc = "The trait to access properties on the `PageTransitionEventInit` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `PageTransitionEventInit`*"]
pub trait PageTransitionEventInitGetters {
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PageTransitionEventInit`*"]
    fn bubbles(&self) -> bool;
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PageTransitionEventInit`*"]
    fn cancelable(&self) -> bool;
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PageTransitionEventInit`*"]
    fn composed(&self) -> bool;
    #[doc = "Get the `inFrameSwap` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PageTransitionEventInit`*"]
    fn in_frame_swap(&self) -> bool;
    #[doc = "Get the `persisted` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PageTransitionEventInit`*"]
    fn persisted(&self) -> bool;
}
impl PageTransitionEventInitGetters for PageTransitionEventInit {
    fn bubbles(&self) -> bool {
        self.bubbles_shim()
    }
    fn cancelable(&self) -> bool {
        self.cancelable_shim()
    }
    fn composed(&self) -> bool {
        self.composed_shim()
    }
    fn in_frame_swap(&self) -> bool {
        self.in_frame_swap_shim()
    }
    fn persisted(&self) -> bool {
        self.persisted_shim()
    }
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
        self.set_bubbles_shim(val);
        self
    }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PageTransitionEventInit`*"]
    pub fn cancelable(&mut self, val: bool) -> &mut Self {
        self.set_cancelable_shim(val);
        self
    }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PageTransitionEventInit`*"]
    pub fn composed(&mut self, val: bool) -> &mut Self {
        self.set_composed_shim(val);
        self
    }
    #[doc = "Change the `inFrameSwap` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PageTransitionEventInit`*"]
    pub fn in_frame_swap(&mut self, val: bool) -> &mut Self {
        self.set_in_frame_swap_shim(val);
        self
    }
    #[doc = "Change the `persisted` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PageTransitionEventInit`*"]
    pub fn persisted(&mut self, val: bool) -> &mut Self {
        self.set_persisted_shim(val);
        self
    }
}
impl Default for PageTransitionEventInit {
    fn default() -> Self {
        Self::new()
    }
}
