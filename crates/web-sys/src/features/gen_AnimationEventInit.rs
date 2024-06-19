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
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationEventInit`*"]
    #[wasm_bindgen(method, getter = "bubbles")]
    pub fn get_bubbles(this: &AnimationEventInit) -> Option<bool>;
    #[wasm_bindgen(method, setter = "bubbles")]
    fn set_bubbles(this: &AnimationEventInit, val: bool);
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationEventInit`*"]
    #[wasm_bindgen(method, getter = "cancelable")]
    pub fn get_cancelable(this: &AnimationEventInit) -> Option<bool>;
    #[wasm_bindgen(method, setter = "cancelable")]
    fn set_cancelable(this: &AnimationEventInit, val: bool);
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationEventInit`*"]
    #[wasm_bindgen(method, getter = "composed")]
    pub fn get_composed(this: &AnimationEventInit) -> Option<bool>;
    #[wasm_bindgen(method, setter = "composed")]
    fn set_composed(this: &AnimationEventInit, val: bool);
    #[doc = "Get the `animationName` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationEventInit`*"]
    #[wasm_bindgen(method, getter = "animationName")]
    pub fn get_animation_name(this: &AnimationEventInit) -> Option<String>;
    #[wasm_bindgen(method, setter = "animationName")]
    fn set_animation_name(this: &AnimationEventInit, val: &str);
    #[doc = "Get the `elapsedTime` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationEventInit`*"]
    #[wasm_bindgen(method, getter = "elapsedTime")]
    pub fn get_elapsed_time(this: &AnimationEventInit) -> Option<f32>;
    #[wasm_bindgen(method, setter = "elapsedTime")]
    fn set_elapsed_time(this: &AnimationEventInit, val: f32);
    #[doc = "Get the `pseudoElement` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationEventInit`*"]
    #[wasm_bindgen(method, getter = "pseudoElement")]
    pub fn get_pseudo_element(this: &AnimationEventInit) -> Option<String>;
    #[wasm_bindgen(method, setter = "pseudoElement")]
    fn set_pseudo_element(this: &AnimationEventInit, val: &str);
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
        self.set_bubbles(val);
        self
    }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationEventInit`*"]
    pub fn cancelable(&mut self, val: bool) -> &mut Self {
        self.set_cancelable(val);
        self
    }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationEventInit`*"]
    pub fn composed(&mut self, val: bool) -> &mut Self {
        self.set_composed(val);
        self
    }
    #[doc = "Change the `animationName` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationEventInit`*"]
    pub fn animation_name(&mut self, val: &str) -> &mut Self {
        self.set_animation_name(val);
        self
    }
    #[doc = "Change the `elapsedTime` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationEventInit`*"]
    pub fn elapsed_time(&mut self, val: f32) -> &mut Self {
        self.set_elapsed_time(val);
        self
    }
    #[doc = "Change the `pseudoElement` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationEventInit`*"]
    pub fn pseudo_element(&mut self, val: &str) -> &mut Self {
        self.set_pseudo_element(val);
        self
    }
}
impl Default for AnimationEventInit {
    fn default() -> Self {
        Self::new()
    }
}
