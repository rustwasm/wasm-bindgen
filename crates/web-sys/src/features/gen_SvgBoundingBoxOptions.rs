#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = SVGBoundingBoxOptions)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgBoundingBoxOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgBoundingBoxOptions`*"]
    pub type SvgBoundingBoxOptions;
    #[doc = "Get the `clipped` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgBoundingBoxOptions`*"]
    #[wasm_bindgen(method, getter = "clipped")]
    pub fn get_clipped(this: &SvgBoundingBoxOptions) -> Option<bool>;
    #[wasm_bindgen(method, setter = "clipped")]
    fn set_clipped(this: &SvgBoundingBoxOptions, val: bool);
    #[doc = "Get the `fill` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgBoundingBoxOptions`*"]
    #[wasm_bindgen(method, getter = "fill")]
    pub fn get_fill(this: &SvgBoundingBoxOptions) -> Option<bool>;
    #[wasm_bindgen(method, setter = "fill")]
    fn set_fill(this: &SvgBoundingBoxOptions, val: bool);
    #[doc = "Get the `markers` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgBoundingBoxOptions`*"]
    #[wasm_bindgen(method, getter = "markers")]
    pub fn get_markers(this: &SvgBoundingBoxOptions) -> Option<bool>;
    #[wasm_bindgen(method, setter = "markers")]
    fn set_markers(this: &SvgBoundingBoxOptions, val: bool);
    #[doc = "Get the `stroke` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgBoundingBoxOptions`*"]
    #[wasm_bindgen(method, getter = "stroke")]
    pub fn get_stroke(this: &SvgBoundingBoxOptions) -> Option<bool>;
    #[wasm_bindgen(method, setter = "stroke")]
    fn set_stroke(this: &SvgBoundingBoxOptions, val: bool);
}
impl SvgBoundingBoxOptions {
    #[doc = "Construct a new `SvgBoundingBoxOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgBoundingBoxOptions`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `clipped` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgBoundingBoxOptions`*"]
    pub fn clipped(&mut self, val: bool) -> &mut Self {
        self.set_clipped(val);
        self
    }
    #[doc = "Change the `fill` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgBoundingBoxOptions`*"]
    pub fn fill(&mut self, val: bool) -> &mut Self {
        self.set_fill(val);
        self
    }
    #[doc = "Change the `markers` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgBoundingBoxOptions`*"]
    pub fn markers(&mut self, val: bool) -> &mut Self {
        self.set_markers(val);
        self
    }
    #[doc = "Change the `stroke` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgBoundingBoxOptions`*"]
    pub fn stroke(&mut self, val: bool) -> &mut Self {
        self.set_stroke(val);
        self
    }
}
impl Default for SvgBoundingBoxOptions {
    fn default() -> Self {
        Self::new()
    }
}
