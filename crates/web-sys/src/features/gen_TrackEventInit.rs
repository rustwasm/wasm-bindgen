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
    #[wasm_bindgen(method, getter = "bubbles")]
    fn bubbles_shim(this: &TrackEventInit) -> bool;
    #[wasm_bindgen(method, setter = "bubbles")]
    fn set_bubbles_shim(this: &TrackEventInit, val: bool);
    #[wasm_bindgen(method, getter = "cancelable")]
    fn cancelable_shim(this: &TrackEventInit) -> bool;
    #[wasm_bindgen(method, setter = "cancelable")]
    fn set_cancelable_shim(this: &TrackEventInit, val: bool);
    #[wasm_bindgen(method, getter = "composed")]
    fn composed_shim(this: &TrackEventInit) -> bool;
    #[wasm_bindgen(method, setter = "composed")]
    fn set_composed_shim(this: &TrackEventInit, val: bool);
    #[wasm_bindgen(method, getter = "track")]
    fn track_shim(this: &TrackEventInit) -> Option<&::js_sys::Object>;
    #[wasm_bindgen(method, setter = "track")]
    fn set_track_shim(this: &TrackEventInit, val: Option<&::js_sys::Object>);
}
#[doc = "The trait to access properties on the `TrackEventInit` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `TrackEventInit`*"]
pub trait TrackEventInitGetters {
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TrackEventInit`*"]
    fn bubbles(&self) -> bool;
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TrackEventInit`*"]
    fn cancelable(&self) -> bool;
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TrackEventInit`*"]
    fn composed(&self) -> bool;
    #[doc = "Get the `track` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TrackEventInit`*"]
    fn track(&self) -> Option<&::js_sys::Object>;
}
impl TrackEventInitGetters for TrackEventInit {
    fn bubbles(&self) -> bool {
        self.bubbles_shim()
    }
    fn cancelable(&self) -> bool {
        self.cancelable_shim()
    }
    fn composed(&self) -> bool {
        self.composed_shim()
    }
    fn track(&self) -> Option<&::js_sys::Object> {
        self.track_shim()
    }
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
        self.set_bubbles_shim(val);
        self
    }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TrackEventInit`*"]
    pub fn cancelable(&mut self, val: bool) -> &mut Self {
        self.set_cancelable_shim(val);
        self
    }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TrackEventInit`*"]
    pub fn composed(&mut self, val: bool) -> &mut Self {
        self.set_composed_shim(val);
        self
    }
    #[doc = "Change the `track` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TrackEventInit`*"]
    pub fn track(&mut self, val: Option<&::js_sys::Object>) -> &mut Self {
        self.set_track_shim(val);
        self
    }
}
impl Default for TrackEventInit {
    fn default() -> Self {
        Self::new()
    }
}
