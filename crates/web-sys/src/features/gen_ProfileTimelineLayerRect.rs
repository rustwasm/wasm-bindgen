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
    #[wasm_bindgen(method, getter = "height")]
    fn height_shim(this: &ProfileTimelineLayerRect) -> i32;
    #[wasm_bindgen(method, setter = "height")]
    fn set_height_shim(this: &ProfileTimelineLayerRect, val: i32);
    #[wasm_bindgen(method, getter = "width")]
    fn width_shim(this: &ProfileTimelineLayerRect) -> i32;
    #[wasm_bindgen(method, setter = "width")]
    fn set_width_shim(this: &ProfileTimelineLayerRect, val: i32);
    #[wasm_bindgen(method, getter = "x")]
    fn x_shim(this: &ProfileTimelineLayerRect) -> i32;
    #[wasm_bindgen(method, setter = "x")]
    fn set_x_shim(this: &ProfileTimelineLayerRect, val: i32);
    #[wasm_bindgen(method, getter = "y")]
    fn y_shim(this: &ProfileTimelineLayerRect) -> i32;
    #[wasm_bindgen(method, setter = "y")]
    fn set_y_shim(this: &ProfileTimelineLayerRect, val: i32);
}
#[doc = "The trait to access properties on the `ProfileTimelineLayerRect` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `ProfileTimelineLayerRect`*"]
pub trait ProfileTimelineLayerRectGetters {
    #[doc = "Get the `height` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineLayerRect`*"]
    fn height(&self) -> i32;
    #[doc = "Get the `width` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineLayerRect`*"]
    fn width(&self) -> i32;
    #[doc = "Get the `x` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineLayerRect`*"]
    fn x(&self) -> i32;
    #[doc = "Get the `y` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineLayerRect`*"]
    fn y(&self) -> i32;
}
impl ProfileTimelineLayerRectGetters for ProfileTimelineLayerRect {
    fn height(&self) -> i32 {
        self.height_shim()
    }
    fn width(&self) -> i32 {
        self.width_shim()
    }
    fn x(&self) -> i32 {
        self.x_shim()
    }
    fn y(&self) -> i32 {
        self.y_shim()
    }
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
        self.set_height_shim(val);
        self
    }
    #[doc = "Change the `width` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineLayerRect`*"]
    pub fn width(&mut self, val: i32) -> &mut Self {
        self.set_width_shim(val);
        self
    }
    #[doc = "Change the `x` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineLayerRect`*"]
    pub fn x(&mut self, val: i32) -> &mut Self {
        self.set_x_shim(val);
        self
    }
    #[doc = "Change the `y` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineLayerRect`*"]
    pub fn y(&mut self, val: i32) -> &mut Self {
        self.set_y_shim(val);
        self
    }
}
impl Default for ProfileTimelineLayerRect {
    fn default() -> Self {
        Self::new()
    }
}
