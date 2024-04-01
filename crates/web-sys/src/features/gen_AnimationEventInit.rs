#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = AnimationEventInit)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AnimationEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationEventInit`*"]
    pub type AnimationEventInit;
    #[wasm_bindgen(method, setter = "bubbles")]
    fn bubbles_shim(this: &AnimationEventInit, val: bool);
    #[wasm_bindgen(method, setter = "cancelable")]
    fn cancelable_shim(this: &AnimationEventInit, val: bool);
    #[wasm_bindgen(method, setter = "composed")]
    fn composed_shim(this: &AnimationEventInit, val: bool);
    #[wasm_bindgen(method, setter = "animationName")]
    fn animation_name_shim(this: &AnimationEventInit, val: &str);
    #[wasm_bindgen(method, setter = "elapsedTime")]
    fn elapsed_time_shim(this: &AnimationEventInit, val: f32);
    #[wasm_bindgen(method, setter = "pseudoElement")]
    fn pseudo_element_shim(this: &AnimationEventInit, val: &str);
}
impl AnimationEventInit {
    #[doc = "Construct a new `AnimationEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationEventInit`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationEventInit`*"]
    pub fn bubbles(&mut self, val: bool) -> &mut Self {
        self.bubbles_shim(val);
        self
    }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationEventInit`*"]
    pub fn cancelable(&mut self, val: bool) -> &mut Self {
        self.cancelable_shim(val);
        self
    }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationEventInit`*"]
    pub fn composed(&mut self, val: bool) -> &mut Self {
        self.composed_shim(val);
        self
    }
    #[doc = "Change the `animationName` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationEventInit`*"]
    pub fn animation_name(&mut self, val: &str) -> &mut Self {
        self.animation_name_shim(val);
        self
    }
    #[doc = "Change the `elapsedTime` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationEventInit`*"]
    pub fn elapsed_time(&mut self, val: f32) -> &mut Self {
        self.elapsed_time_shim(val);
        self
    }
    #[doc = "Change the `pseudoElement` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationEventInit`*"]
    pub fn pseudo_element(&mut self, val: &str) -> &mut Self {
        self.pseudo_element_shim(val);
        self
    }
}
impl Default for AnimationEventInit {
    fn default() -> Self {
        Self::new()
    }
}
