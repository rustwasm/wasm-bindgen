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
    #[wasm_bindgen(method, getter = "bubbles")]
    fn bubbles_shim(this: &MediaQueryListEventInit) -> bool;
    #[wasm_bindgen(method, setter = "bubbles")]
    fn set_bubbles_shim(this: &MediaQueryListEventInit, val: bool);
    #[wasm_bindgen(method, getter = "cancelable")]
    fn cancelable_shim(this: &MediaQueryListEventInit) -> bool;
    #[wasm_bindgen(method, setter = "cancelable")]
    fn set_cancelable_shim(this: &MediaQueryListEventInit, val: bool);
    #[wasm_bindgen(method, getter = "composed")]
    fn composed_shim(this: &MediaQueryListEventInit) -> bool;
    #[wasm_bindgen(method, setter = "composed")]
    fn set_composed_shim(this: &MediaQueryListEventInit, val: bool);
    #[wasm_bindgen(method, getter = "matches")]
    fn matches_shim(this: &MediaQueryListEventInit) -> bool;
    #[wasm_bindgen(method, setter = "matches")]
    fn set_matches_shim(this: &MediaQueryListEventInit, val: bool);
    #[wasm_bindgen(method, getter = "media")]
    fn media_shim(this: &MediaQueryListEventInit) -> &str;
    #[wasm_bindgen(method, setter = "media")]
    fn set_media_shim(this: &MediaQueryListEventInit, val: &str);
}
#[doc = "The trait to access properties on the `MediaQueryListEventInit` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `MediaQueryListEventInit`*"]
pub trait MediaQueryListEventInitGetters {
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaQueryListEventInit`*"]
    fn bubbles(&self) -> bool;
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaQueryListEventInit`*"]
    fn cancelable(&self) -> bool;
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaQueryListEventInit`*"]
    fn composed(&self) -> bool;
    #[doc = "Get the `matches` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaQueryListEventInit`*"]
    fn matches(&self) -> bool;
    #[doc = "Get the `media` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaQueryListEventInit`*"]
    fn media(&self) -> &str;
}
impl MediaQueryListEventInitGetters for MediaQueryListEventInit {
    fn bubbles(&self) -> bool {
        self.bubbles_shim()
    }
    fn cancelable(&self) -> bool {
        self.cancelable_shim()
    }
    fn composed(&self) -> bool {
        self.composed_shim()
    }
    fn matches(&self) -> bool {
        self.matches_shim()
    }
    fn media(&self) -> &str {
        self.media_shim()
    }
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
        self.set_bubbles_shim(val);
        self
    }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaQueryListEventInit`*"]
    pub fn cancelable(&mut self, val: bool) -> &mut Self {
        self.set_cancelable_shim(val);
        self
    }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaQueryListEventInit`*"]
    pub fn composed(&mut self, val: bool) -> &mut Self {
        self.set_composed_shim(val);
        self
    }
    #[doc = "Change the `matches` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaQueryListEventInit`*"]
    pub fn matches(&mut self, val: bool) -> &mut Self {
        self.set_matches_shim(val);
        self
    }
    #[doc = "Change the `media` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaQueryListEventInit`*"]
    pub fn media(&mut self, val: &str) -> &mut Self {
        self.set_media_shim(val);
        self
    }
}
impl Default for MediaQueryListEventInit {
    fn default() -> Self {
        Self::new()
    }
}
