#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = ProfileTimelineLayerRect)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ProfileTimelineLayerRect` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineLayerRect`*"]
    pub type ProfileTimelineLayerRect;
    #[wasm_bindgen(method, setter = "height")]
    fn height_shim(this: &ProfileTimelineLayerRect, val: i32);
    #[wasm_bindgen(method, setter = "width")]
    fn width_shim(this: &ProfileTimelineLayerRect, val: i32);
    #[wasm_bindgen(method, setter = "x")]
    fn x_shim(this: &ProfileTimelineLayerRect, val: i32);
    #[wasm_bindgen(method, setter = "y")]
    fn y_shim(this: &ProfileTimelineLayerRect, val: i32);
}
impl ProfileTimelineLayerRect {
    #[doc = "Construct a new `ProfileTimelineLayerRect`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineLayerRect`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `height` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineLayerRect`*"]
    pub fn height(&mut self, val: i32) -> &mut Self {
        self.height_shim(val);
        self
    }
    #[doc = "Change the `width` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineLayerRect`*"]
    pub fn width(&mut self, val: i32) -> &mut Self {
        self.width_shim(val);
        self
    }
    #[doc = "Change the `x` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineLayerRect`*"]
    pub fn x(&mut self, val: i32) -> &mut Self {
        self.x_shim(val);
        self
    }
    #[doc = "Change the `y` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineLayerRect`*"]
    pub fn y(&mut self, val: i32) -> &mut Self {
        self.y_shim(val);
        self
    }
}
impl Default for ProfileTimelineLayerRect {
    fn default() -> Self {
        Self::new()
    }
}
