#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = ScrollOptions)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ScrollOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ScrollOptions`*"]
    pub type ScrollOptions;
    #[cfg(feature = "ScrollBehavior")]
    #[wasm_bindgen(method, getter = "behavior")]
    fn behavior_shim(this: &ScrollOptions) -> ScrollBehavior;
    #[cfg(feature = "ScrollBehavior")]
    #[wasm_bindgen(method, setter = "behavior")]
    fn set_behavior_shim(this: &ScrollOptions, val: ScrollBehavior);
}
#[doc = "The trait to access properties on the `ScrollOptions` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `ScrollOptions`*"]
pub trait ScrollOptionsGetters {
    #[cfg(feature = "ScrollBehavior")]
    #[doc = "Get the `behavior` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ScrollBehavior`, `ScrollOptions`*"]
    fn behavior(&self) -> ScrollBehavior;
}
impl ScrollOptionsGetters for ScrollOptions {
    #[cfg(feature = "ScrollBehavior")]
    fn behavior(&self) -> ScrollBehavior {
        self.behavior_shim()
    }
}
impl ScrollOptions {
    #[doc = "Construct a new `ScrollOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ScrollOptions`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(feature = "ScrollBehavior")]
    #[doc = "Change the `behavior` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ScrollBehavior`, `ScrollOptions`*"]
    pub fn behavior(&mut self, val: ScrollBehavior) -> &mut Self {
        self.set_behavior_shim(val);
        self
    }
}
impl Default for ScrollOptions {
    fn default() -> Self {
        Self::new()
    }
}
