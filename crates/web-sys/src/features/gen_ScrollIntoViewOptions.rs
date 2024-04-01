#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = ScrollIntoViewOptions)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ScrollIntoViewOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ScrollIntoViewOptions`*"]
    pub type ScrollIntoViewOptions;
    #[cfg(feature = "ScrollBehavior")]
    #[wasm_bindgen(method, setter = "behavior")]
    fn behavior_shim(this: &ScrollIntoViewOptions, val: ScrollBehavior);
    #[cfg(feature = "ScrollLogicalPosition")]
    #[wasm_bindgen(method, setter = "block")]
    fn block_shim(this: &ScrollIntoViewOptions, val: ScrollLogicalPosition);
    #[cfg(feature = "ScrollLogicalPosition")]
    #[wasm_bindgen(method, setter = "inline")]
    fn inline_shim(this: &ScrollIntoViewOptions, val: ScrollLogicalPosition);
}
impl ScrollIntoViewOptions {
    #[doc = "Construct a new `ScrollIntoViewOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ScrollIntoViewOptions`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(feature = "ScrollBehavior")]
    #[doc = "Change the `behavior` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ScrollBehavior`, `ScrollIntoViewOptions`*"]
    pub fn behavior(&mut self, val: ScrollBehavior) -> &mut Self {
        self.behavior_shim(val);
        self
    }
    #[cfg(feature = "ScrollLogicalPosition")]
    #[doc = "Change the `block` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ScrollIntoViewOptions`, `ScrollLogicalPosition`*"]
    pub fn block(&mut self, val: ScrollLogicalPosition) -> &mut Self {
        self.block_shim(val);
        self
    }
    #[cfg(feature = "ScrollLogicalPosition")]
    #[doc = "Change the `inline` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ScrollIntoViewOptions`, `ScrollLogicalPosition`*"]
    pub fn inline(&mut self, val: ScrollLogicalPosition) -> &mut Self {
        self.inline_shim(val);
        self
    }
}
impl Default for ScrollIntoViewOptions {
    fn default() -> Self {
        Self::new()
    }
}
