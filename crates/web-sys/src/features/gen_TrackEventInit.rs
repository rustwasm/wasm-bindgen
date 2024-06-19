#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = TrackEventInit)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `TrackEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TrackEventInit`*"]
    pub type TrackEventInit;
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TrackEventInit`*"]
    #[wasm_bindgen(method, getter = "bubbles")]
    pub fn get_bubbles(this: &TrackEventInit) -> Option<bool>;
    #[wasm_bindgen(method, setter = "bubbles")]
    fn set_bubbles(this: &TrackEventInit, val: bool);
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TrackEventInit`*"]
    #[wasm_bindgen(method, getter = "cancelable")]
    pub fn get_cancelable(this: &TrackEventInit) -> Option<bool>;
    #[wasm_bindgen(method, setter = "cancelable")]
    fn set_cancelable(this: &TrackEventInit, val: bool);
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TrackEventInit`*"]
    #[wasm_bindgen(method, getter = "composed")]
    pub fn get_composed(this: &TrackEventInit) -> Option<bool>;
    #[wasm_bindgen(method, setter = "composed")]
    fn set_composed(this: &TrackEventInit, val: bool);
    #[doc = "Get the `track` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TrackEventInit`*"]
    #[wasm_bindgen(method, getter = "track")]
    pub fn get_track(this: &TrackEventInit) -> Option<::js_sys::Object>;
    #[wasm_bindgen(method, setter = "track")]
    fn set_track(this: &TrackEventInit, val: Option<&::js_sys::Object>);
}
impl TrackEventInit {
    #[doc = "Construct a new `TrackEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TrackEventInit`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TrackEventInit`*"]
    pub fn bubbles(&mut self, val: bool) -> &mut Self {
        self.set_bubbles(val);
        self
    }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TrackEventInit`*"]
    pub fn cancelable(&mut self, val: bool) -> &mut Self {
        self.set_cancelable(val);
        self
    }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TrackEventInit`*"]
    pub fn composed(&mut self, val: bool) -> &mut Self {
        self.set_composed(val);
        self
    }
    #[doc = "Change the `track` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TrackEventInit`*"]
    pub fn track(&mut self, val: Option<&::js_sys::Object>) -> &mut Self {
        self.set_track(val);
        self
    }
}
impl Default for TrackEventInit {
    fn default() -> Self {
        Self::new()
    }
}
