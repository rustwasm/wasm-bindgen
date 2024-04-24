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
    #[wasm_bindgen(method, getter = "bubbles")]
    fn bubbles_shim(this: &AnimationEventInit) -> bool;
    #[wasm_bindgen(method, setter = "bubbles")]
    fn set_bubbles_shim(this: &AnimationEventInit, val: bool);
    #[wasm_bindgen(method, getter = "cancelable")]
    fn cancelable_shim(this: &AnimationEventInit) -> bool;
    #[wasm_bindgen(method, setter = "cancelable")]
    fn set_cancelable_shim(this: &AnimationEventInit, val: bool);
    #[wasm_bindgen(method, getter = "composed")]
    fn composed_shim(this: &AnimationEventInit) -> bool;
    #[wasm_bindgen(method, setter = "composed")]
    fn set_composed_shim(this: &AnimationEventInit, val: bool);
    #[wasm_bindgen(method, getter = "animationName")]
    fn animation_name_shim(this: &AnimationEventInit) -> &str;
    #[wasm_bindgen(method, setter = "animationName")]
    fn set_animation_name_shim(this: &AnimationEventInit, val: &str);
    #[wasm_bindgen(method, getter = "elapsedTime")]
    fn elapsed_time_shim(this: &AnimationEventInit) -> f32;
    #[wasm_bindgen(method, setter = "elapsedTime")]
    fn set_elapsed_time_shim(this: &AnimationEventInit, val: f32);
    #[wasm_bindgen(method, getter = "pseudoElement")]
    fn pseudo_element_shim(this: &AnimationEventInit) -> &str;
    #[wasm_bindgen(method, setter = "pseudoElement")]
    fn set_pseudo_element_shim(this: &AnimationEventInit, val: &str);
}
#[doc = "The trait to access properties on the `AnimationEventInit` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `AnimationEventInit`*"]
pub trait AnimationEventInitGetters {
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationEventInit`*"]
    fn bubbles(&self) -> bool;
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationEventInit`*"]
    fn cancelable(&self) -> bool;
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationEventInit`*"]
    fn composed(&self) -> bool;
    #[doc = "Get the `animationName` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationEventInit`*"]
    fn animation_name(&self) -> &str;
    #[doc = "Get the `elapsedTime` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationEventInit`*"]
    fn elapsed_time(&self) -> f32;
    #[doc = "Get the `pseudoElement` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationEventInit`*"]
    fn pseudo_element(&self) -> &str;
}
impl AnimationEventInitGetters for AnimationEventInit {
    fn bubbles(&self) -> bool {
        self.bubbles_shim()
    }
    fn cancelable(&self) -> bool {
        self.cancelable_shim()
    }
    fn composed(&self) -> bool {
        self.composed_shim()
    }
    fn animation_name(&self) -> &str {
        self.animation_name_shim()
    }
    fn elapsed_time(&self) -> f32 {
        self.elapsed_time_shim()
    }
    fn pseudo_element(&self) -> &str {
        self.pseudo_element_shim()
    }
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
        self.set_bubbles_shim(val);
        self
    }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationEventInit`*"]
    pub fn cancelable(&mut self, val: bool) -> &mut Self {
        self.set_cancelable_shim(val);
        self
    }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationEventInit`*"]
    pub fn composed(&mut self, val: bool) -> &mut Self {
        self.set_composed_shim(val);
        self
    }
    #[doc = "Change the `animationName` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationEventInit`*"]
    pub fn animation_name(&mut self, val: &str) -> &mut Self {
        self.set_animation_name_shim(val);
        self
    }
    #[doc = "Change the `elapsedTime` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationEventInit`*"]
    pub fn elapsed_time(&mut self, val: f32) -> &mut Self {
        self.set_elapsed_time_shim(val);
        self
    }
    #[doc = "Change the `pseudoElement` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationEventInit`*"]
    pub fn pseudo_element(&mut self, val: &str) -> &mut Self {
        self.set_pseudo_element_shim(val);
        self
    }
}
impl Default for AnimationEventInit {
    fn default() -> Self {
        Self::new()
    }
}
