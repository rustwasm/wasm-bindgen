#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = MediaStreamTrackEventInit)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MediaStreamTrackEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamTrackEventInit`*"]
    pub type MediaStreamTrackEventInit;
    #[wasm_bindgen(method, getter = "bubbles")]
    fn bubbles_shim(this: &MediaStreamTrackEventInit) -> bool;
    #[wasm_bindgen(method, setter = "bubbles")]
    fn set_bubbles_shim(this: &MediaStreamTrackEventInit, val: bool);
    #[wasm_bindgen(method, getter = "cancelable")]
    fn cancelable_shim(this: &MediaStreamTrackEventInit) -> bool;
    #[wasm_bindgen(method, setter = "cancelable")]
    fn set_cancelable_shim(this: &MediaStreamTrackEventInit, val: bool);
    #[wasm_bindgen(method, getter = "composed")]
    fn composed_shim(this: &MediaStreamTrackEventInit) -> bool;
    #[wasm_bindgen(method, setter = "composed")]
    fn set_composed_shim(this: &MediaStreamTrackEventInit, val: bool);
    #[cfg(feature = "MediaStreamTrack")]
    #[wasm_bindgen(method, getter = "track")]
    fn track_shim(this: &MediaStreamTrackEventInit) -> MediaStreamTrack;
    #[cfg(feature = "MediaStreamTrack")]
    #[wasm_bindgen(method, setter = "track")]
    fn set_track_shim(this: &MediaStreamTrackEventInit, val: &MediaStreamTrack);
}
#[doc = "The trait to access properties on the `MediaStreamTrackEventInit` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `MediaStreamTrackEventInit`*"]
pub trait MediaStreamTrackEventInitGetters {
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamTrackEventInit`*"]
    fn bubbles(&self) -> bool;
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamTrackEventInit`*"]
    fn cancelable(&self) -> bool;
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamTrackEventInit`*"]
    fn composed(&self) -> bool;
    #[cfg(feature = "MediaStreamTrack")]
    #[doc = "Get the `track` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamTrack`, `MediaStreamTrackEventInit`*"]
    fn track(&self) -> MediaStreamTrack;
}
impl MediaStreamTrackEventInitGetters for MediaStreamTrackEventInit {
    fn bubbles(&self) -> bool {
        self.bubbles_shim()
    }
    fn cancelable(&self) -> bool {
        self.cancelable_shim()
    }
    fn composed(&self) -> bool {
        self.composed_shim()
    }
    #[cfg(feature = "MediaStreamTrack")]
    fn track(&self) -> MediaStreamTrack {
        self.track_shim()
    }
}
impl MediaStreamTrackEventInit {
    #[cfg(feature = "MediaStreamTrack")]
    #[doc = "Construct a new `MediaStreamTrackEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamTrack`, `MediaStreamTrackEventInit`*"]
    pub fn new(track: &MediaStreamTrack) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        Self::track(&mut ret, track);
        ret
    }
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamTrackEventInit`*"]
    pub fn bubbles(&mut self, val: bool) -> &mut Self {
        self.set_bubbles_shim(val);
        self
    }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamTrackEventInit`*"]
    pub fn cancelable(&mut self, val: bool) -> &mut Self {
        self.set_cancelable_shim(val);
        self
    }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamTrackEventInit`*"]
    pub fn composed(&mut self, val: bool) -> &mut Self {
        self.set_composed_shim(val);
        self
    }
    #[cfg(feature = "MediaStreamTrack")]
    #[doc = "Change the `track` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamTrack`, `MediaStreamTrackEventInit`*"]
    pub fn track(&mut self, val: &MediaStreamTrack) -> &mut Self {
        self.set_track_shim(val);
        self
    }
}
