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
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaQueryListEventInit`*"]
    #[wasm_bindgen(method, getter = "bubbles")]
    pub fn get_bubbles(this: &MediaQueryListEventInit) -> Option<bool>;
    #[wasm_bindgen(method, setter = "bubbles")]
    fn set_bubbles(this: &MediaQueryListEventInit, val: bool);
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaQueryListEventInit`*"]
    #[wasm_bindgen(method, getter = "cancelable")]
    pub fn get_cancelable(this: &MediaQueryListEventInit) -> Option<bool>;
    #[wasm_bindgen(method, setter = "cancelable")]
    fn set_cancelable(this: &MediaQueryListEventInit, val: bool);
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaQueryListEventInit`*"]
    #[wasm_bindgen(method, getter = "composed")]
    pub fn get_composed(this: &MediaQueryListEventInit) -> Option<bool>;
    #[wasm_bindgen(method, setter = "composed")]
    fn set_composed(this: &MediaQueryListEventInit, val: bool);
    #[doc = "Get the `matches` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaQueryListEventInit`*"]
    #[wasm_bindgen(method, getter = "matches")]
    pub fn get_matches(this: &MediaQueryListEventInit) -> Option<bool>;
    #[wasm_bindgen(method, setter = "matches")]
    fn set_matches(this: &MediaQueryListEventInit, val: bool);
    #[doc = "Get the `media` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaQueryListEventInit`*"]
    #[wasm_bindgen(method, getter = "media")]
    pub fn get_media(this: &MediaQueryListEventInit) -> Option<String>;
    #[wasm_bindgen(method, setter = "media")]
    fn set_media(this: &MediaQueryListEventInit, val: &str);
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
        self.set_bubbles(val);
        self
    }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaQueryListEventInit`*"]
    pub fn cancelable(&mut self, val: bool) -> &mut Self {
        self.set_cancelable(val);
        self
    }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaQueryListEventInit`*"]
    pub fn composed(&mut self, val: bool) -> &mut Self {
        self.set_composed(val);
        self
    }
    #[doc = "Change the `matches` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaQueryListEventInit`*"]
    pub fn matches(&mut self, val: bool) -> &mut Self {
        self.set_matches(val);
        self
    }
    #[doc = "Change the `media` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaQueryListEventInit`*"]
    pub fn media(&mut self, val: &str) -> &mut Self {
        self.set_media(val);
        self
    }
}
impl Default for MediaQueryListEventInit {
    fn default() -> Self {
        Self::new()
    }
}
