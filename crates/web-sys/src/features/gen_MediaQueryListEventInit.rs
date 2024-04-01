#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = MediaQueryListEventInit)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MediaQueryListEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaQueryListEventInit`*"]
    pub type MediaQueryListEventInit;
    #[wasm_bindgen(method, setter = "bubbles")]
    fn bubbles_shim(this: &MediaQueryListEventInit, val: bool);
    #[wasm_bindgen(method, setter = "cancelable")]
    fn cancelable_shim(this: &MediaQueryListEventInit, val: bool);
    #[wasm_bindgen(method, setter = "composed")]
    fn composed_shim(this: &MediaQueryListEventInit, val: bool);
    #[wasm_bindgen(method, setter = "matches")]
    fn matches_shim(this: &MediaQueryListEventInit, val: bool);
    #[wasm_bindgen(method, setter = "media")]
    fn media_shim(this: &MediaQueryListEventInit, val: &str);
}
impl MediaQueryListEventInit {
    #[doc = "Construct a new `MediaQueryListEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaQueryListEventInit`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaQueryListEventInit`*"]
    pub fn bubbles(&mut self, val: bool) -> &mut Self {
        self.bubbles_shim(val);
        self
    }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaQueryListEventInit`*"]
    pub fn cancelable(&mut self, val: bool) -> &mut Self {
        self.cancelable_shim(val);
        self
    }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaQueryListEventInit`*"]
    pub fn composed(&mut self, val: bool) -> &mut Self {
        self.composed_shim(val);
        self
    }
    #[doc = "Change the `matches` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaQueryListEventInit`*"]
    pub fn matches(&mut self, val: bool) -> &mut Self {
        self.matches_shim(val);
        self
    }
    #[doc = "Change the `media` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaQueryListEventInit`*"]
    pub fn media(&mut self, val: &str) -> &mut Self {
        self.media_shim(val);
        self
    }
}
impl Default for MediaQueryListEventInit {
    fn default() -> Self {
        Self::new()
    }
}
