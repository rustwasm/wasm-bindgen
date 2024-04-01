#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = ConvertCoordinateOptions)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ConvertCoordinateOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConvertCoordinateOptions`*"]
    pub type ConvertCoordinateOptions;
    #[cfg(feature = "CssBoxType")]
    #[wasm_bindgen(method, setter = "fromBox")]
    fn from_box_shim(this: &ConvertCoordinateOptions, val: CssBoxType);
    #[cfg(feature = "CssBoxType")]
    #[wasm_bindgen(method, setter = "toBox")]
    fn to_box_shim(this: &ConvertCoordinateOptions, val: CssBoxType);
}
impl ConvertCoordinateOptions {
    #[doc = "Construct a new `ConvertCoordinateOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConvertCoordinateOptions`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(feature = "CssBoxType")]
    #[doc = "Change the `fromBox` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConvertCoordinateOptions`, `CssBoxType`*"]
    pub fn from_box(&mut self, val: CssBoxType) -> &mut Self {
        self.from_box_shim(val);
        self
    }
    #[cfg(feature = "CssBoxType")]
    #[doc = "Change the `toBox` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConvertCoordinateOptions`, `CssBoxType`*"]
    pub fn to_box(&mut self, val: CssBoxType) -> &mut Self {
        self.to_box_shim(val);
        self
    }
}
impl Default for ConvertCoordinateOptions {
    fn default() -> Self {
        Self::new()
    }
}
