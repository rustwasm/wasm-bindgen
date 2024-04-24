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
    #[wasm_bindgen(method, getter = "clipped")]
    fn clipped_shim(this: &SvgBoundingBoxOptions) -> bool;
    #[wasm_bindgen(method, setter = "clipped")]
    fn set_clipped_shim(this: &SvgBoundingBoxOptions, val: bool);
    #[wasm_bindgen(method, getter = "fill")]
    fn fill_shim(this: &SvgBoundingBoxOptions) -> bool;
    #[wasm_bindgen(method, setter = "fill")]
    fn set_fill_shim(this: &SvgBoundingBoxOptions, val: bool);
    #[wasm_bindgen(method, getter = "markers")]
    fn markers_shim(this: &SvgBoundingBoxOptions) -> bool;
    #[wasm_bindgen(method, setter = "markers")]
    fn set_markers_shim(this: &SvgBoundingBoxOptions, val: bool);
    #[wasm_bindgen(method, getter = "stroke")]
    fn stroke_shim(this: &SvgBoundingBoxOptions) -> bool;
    #[wasm_bindgen(method, setter = "stroke")]
    fn set_stroke_shim(this: &SvgBoundingBoxOptions, val: bool);
}
#[doc = "The trait to access properties on the `SvgBoundingBoxOptions` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `SvgBoundingBoxOptions`*"]
pub trait SvgBoundingBoxOptionsGetters {
    #[doc = "Get the `clipped` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgBoundingBoxOptions`*"]
    fn clipped(&self) -> bool;
    #[doc = "Get the `fill` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgBoundingBoxOptions`*"]
    fn fill(&self) -> bool;
    #[doc = "Get the `markers` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgBoundingBoxOptions`*"]
    fn markers(&self) -> bool;
    #[doc = "Get the `stroke` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgBoundingBoxOptions`*"]
    fn stroke(&self) -> bool;
}
impl SvgBoundingBoxOptionsGetters for SvgBoundingBoxOptions {
    fn clipped(&self) -> bool {
        self.clipped_shim()
    }
    fn fill(&self) -> bool {
        self.fill_shim()
    }
    fn markers(&self) -> bool {
        self.markers_shim()
    }
    fn stroke(&self) -> bool {
        self.stroke_shim()
    }
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
        self.set_clipped_shim(val);
        self
    }
    #[doc = "Change the `fill` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgBoundingBoxOptions`*"]
    pub fn fill(&mut self, val: bool) -> &mut Self {
        self.set_fill_shim(val);
        self
    }
    #[doc = "Change the `markers` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgBoundingBoxOptions`*"]
    pub fn markers(&mut self, val: bool) -> &mut Self {
        self.set_markers_shim(val);
        self
    }
    #[doc = "Change the `stroke` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgBoundingBoxOptions`*"]
    pub fn stroke(&mut self, val: bool) -> &mut Self {
        self.set_stroke_shim(val);
        self
    }
}
impl Default for SvgBoundingBoxOptions {
    fn default() -> Self {
        Self::new()
    }
}
