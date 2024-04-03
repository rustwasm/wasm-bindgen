#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = ScrollToOptions)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ScrollToOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ScrollToOptions`*"]
    pub type ScrollToOptions;
    #[cfg(feature = "ScrollBehavior")]
    #[wasm_bindgen(method, setter = "behavior")]
    fn behavior_shim(this: &ScrollToOptions, val: ScrollBehavior);
    #[wasm_bindgen(method, setter = "left")]
    fn left_shim(this: &ScrollToOptions, val: f64);
    #[wasm_bindgen(method, setter = "top")]
    fn top_shim(this: &ScrollToOptions, val: f64);
}
impl ScrollToOptions {
    #[doc = "Construct a new `ScrollToOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ScrollToOptions`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(feature = "ScrollBehavior")]
    #[doc = "Change the `behavior` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ScrollBehavior`, `ScrollToOptions`*"]
    pub fn behavior(&mut self, val: ScrollBehavior) -> &mut Self {
        self.behavior_shim(val);
        self
    }
    #[doc = "Change the `left` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ScrollToOptions`*"]
    pub fn left(&mut self, val: f64) -> &mut Self {
        self.left_shim(val);
        self
    }
    #[doc = "Change the `top` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ScrollToOptions`*"]
    pub fn top(&mut self, val: f64) -> &mut Self {
        self.top_shim(val);
        self
    }
}
impl Default for ScrollToOptions {
    fn default() -> Self {
        Self::new()
    }
}
