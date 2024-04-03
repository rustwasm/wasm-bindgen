#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = ImageEncodeOptions)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ImageEncodeOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageEncodeOptions`*"]
    pub type ImageEncodeOptions;
    #[wasm_bindgen(method, setter = "quality")]
    fn quality_shim(this: &ImageEncodeOptions, val: f64);
    #[wasm_bindgen(method, setter = "type")]
    fn type__shim(this: &ImageEncodeOptions, val: &str);
}
impl ImageEncodeOptions {
    #[doc = "Construct a new `ImageEncodeOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageEncodeOptions`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `quality` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageEncodeOptions`*"]
    pub fn quality(&mut self, val: f64) -> &mut Self {
        self.quality_shim(val);
        self
    }
    #[doc = "Change the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageEncodeOptions`*"]
    pub fn type_(&mut self, val: &str) -> &mut Self {
        self.type__shim(val);
        self
    }
}
impl Default for ImageEncodeOptions {
    fn default() -> Self {
        Self::new()
    }
}
