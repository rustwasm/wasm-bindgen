#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = AnimationPlaybackEventInit)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AnimationPlaybackEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationPlaybackEventInit`*"]
    pub type AnimationPlaybackEventInit;
    #[wasm_bindgen(method, getter = "bubbles")]
    fn bubbles_shim(this: &AnimationPlaybackEventInit) -> bool;
    #[wasm_bindgen(method, setter = "bubbles")]
    fn set_bubbles_shim(this: &AnimationPlaybackEventInit, val: bool);
    #[wasm_bindgen(method, getter = "cancelable")]
    fn cancelable_shim(this: &AnimationPlaybackEventInit) -> bool;
    #[wasm_bindgen(method, setter = "cancelable")]
    fn set_cancelable_shim(this: &AnimationPlaybackEventInit, val: bool);
    #[wasm_bindgen(method, getter = "composed")]
    fn composed_shim(this: &AnimationPlaybackEventInit) -> bool;
    #[wasm_bindgen(method, setter = "composed")]
    fn set_composed_shim(this: &AnimationPlaybackEventInit, val: bool);
    #[wasm_bindgen(method, getter = "currentTime")]
    fn current_time_shim(this: &AnimationPlaybackEventInit) -> Option<f64>;
    #[wasm_bindgen(method, setter = "currentTime")]
    fn set_current_time_shim(this: &AnimationPlaybackEventInit, val: Option<f64>);
    #[wasm_bindgen(method, getter = "timelineTime")]
    fn timeline_time_shim(this: &AnimationPlaybackEventInit) -> Option<f64>;
    #[wasm_bindgen(method, setter = "timelineTime")]
    fn set_timeline_time_shim(this: &AnimationPlaybackEventInit, val: Option<f64>);
}
#[doc = "The trait to access properties on the `AnimationPlaybackEventInit` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `AnimationPlaybackEventInit`*"]
pub trait AnimationPlaybackEventInitGetters {
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationPlaybackEventInit`*"]
    fn bubbles(&self) -> bool;
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationPlaybackEventInit`*"]
    fn cancelable(&self) -> bool;
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationPlaybackEventInit`*"]
    fn composed(&self) -> bool;
    #[doc = "Get the `currentTime` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationPlaybackEventInit`*"]
    fn current_time(&self) -> Option<f64>;
    #[doc = "Get the `timelineTime` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationPlaybackEventInit`*"]
    fn timeline_time(&self) -> Option<f64>;
}
impl AnimationPlaybackEventInitGetters for AnimationPlaybackEventInit {
    fn bubbles(&self) -> bool {
        self.bubbles_shim()
    }
    fn cancelable(&self) -> bool {
        self.cancelable_shim()
    }
    fn composed(&self) -> bool {
        self.composed_shim()
    }
    fn current_time(&self) -> Option<f64> {
        self.current_time_shim()
    }
    fn timeline_time(&self) -> Option<f64> {
        self.timeline_time_shim()
    }
}
impl AnimationPlaybackEventInit {
    #[doc = "Construct a new `AnimationPlaybackEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationPlaybackEventInit`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationPlaybackEventInit`*"]
    pub fn bubbles(&mut self, val: bool) -> &mut Self {
        self.set_bubbles_shim(val);
        self
    }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationPlaybackEventInit`*"]
    pub fn cancelable(&mut self, val: bool) -> &mut Self {
        self.set_cancelable_shim(val);
        self
    }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationPlaybackEventInit`*"]
    pub fn composed(&mut self, val: bool) -> &mut Self {
        self.set_composed_shim(val);
        self
    }
    #[doc = "Change the `currentTime` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationPlaybackEventInit`*"]
    pub fn current_time(&mut self, val: Option<f64>) -> &mut Self {
        self.set_current_time_shim(val);
        self
    }
    #[doc = "Change the `timelineTime` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationPlaybackEventInit`*"]
    pub fn timeline_time(&mut self, val: Option<f64>) -> &mut Self {
        self.set_timeline_time_shim(val);
        self
    }
}
impl Default for AnimationPlaybackEventInit {
    fn default() -> Self {
        Self::new()
    }
}
