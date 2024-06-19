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
    #[doc = "Get the `height` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineLayerRect`*"]
    #[wasm_bindgen(method, getter = "height")]
    pub fn get_height(this: &ProfileTimelineLayerRect) -> Option<i32>;
    #[wasm_bindgen(method, setter = "height")]
    fn set_height(this: &ProfileTimelineLayerRect, val: i32);
    #[doc = "Get the `width` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineLayerRect`*"]
    #[wasm_bindgen(method, getter = "width")]
    pub fn get_width(this: &ProfileTimelineLayerRect) -> Option<i32>;
    #[wasm_bindgen(method, setter = "width")]
    fn set_width(this: &ProfileTimelineLayerRect, val: i32);
    #[doc = "Get the `x` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineLayerRect`*"]
    #[wasm_bindgen(method, getter = "x")]
    pub fn get_x(this: &ProfileTimelineLayerRect) -> Option<i32>;
    #[wasm_bindgen(method, setter = "x")]
    fn set_x(this: &ProfileTimelineLayerRect, val: i32);
    #[doc = "Get the `y` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineLayerRect`*"]
    #[wasm_bindgen(method, getter = "y")]
    pub fn get_y(this: &ProfileTimelineLayerRect) -> Option<i32>;
    #[wasm_bindgen(method, setter = "y")]
    fn set_y(this: &ProfileTimelineLayerRect, val: i32);
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
        self.set_height(val);
        self
    }
    #[doc = "Change the `width` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineLayerRect`*"]
    pub fn width(&mut self, val: i32) -> &mut Self {
        self.set_width(val);
        self
    }
    #[doc = "Change the `x` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineLayerRect`*"]
    pub fn x(&mut self, val: i32) -> &mut Self {
        self.set_x(val);
        self
    }
    #[doc = "Change the `y` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineLayerRect`*"]
    pub fn y(&mut self, val: i32) -> &mut Self {
        self.set_y(val);
        self
    }
}
impl Default for ProfileTimelineLayerRect {
    fn default() -> Self {
        Self::new()
    }
}
