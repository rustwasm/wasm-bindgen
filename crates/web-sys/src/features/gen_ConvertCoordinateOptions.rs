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
    #[wasm_bindgen(method, getter = "fromBox")]
    fn from_box_shim(this: &ConvertCoordinateOptions) -> CssBoxType;
    #[cfg(feature = "CssBoxType")]
    #[wasm_bindgen(method, setter = "fromBox")]
    fn set_from_box_shim(this: &ConvertCoordinateOptions, val: CssBoxType);
    #[cfg(feature = "CssBoxType")]
    #[wasm_bindgen(method, getter = "toBox")]
    fn to_box_shim(this: &ConvertCoordinateOptions) -> CssBoxType;
    #[cfg(feature = "CssBoxType")]
    #[wasm_bindgen(method, setter = "toBox")]
    fn set_to_box_shim(this: &ConvertCoordinateOptions, val: CssBoxType);
}
#[doc = "The trait to access properties on the `ConvertCoordinateOptions` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `ConvertCoordinateOptions`*"]
pub trait ConvertCoordinateOptionsGetters {
    #[cfg(feature = "CssBoxType")]
    #[doc = "Get the `fromBox` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConvertCoordinateOptions`, `CssBoxType`*"]
    fn from_box(&self) -> CssBoxType;
    #[cfg(feature = "CssBoxType")]
    #[doc = "Get the `toBox` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConvertCoordinateOptions`, `CssBoxType`*"]
    fn to_box(&self) -> CssBoxType;
}
impl ConvertCoordinateOptionsGetters for ConvertCoordinateOptions {
    #[cfg(feature = "CssBoxType")]
    fn from_box(&self) -> CssBoxType {
        self.from_box_shim()
    }
    #[cfg(feature = "CssBoxType")]
    fn to_box(&self) -> CssBoxType {
        self.to_box_shim()
    }
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
        self.set_from_box_shim(val);
        self
    }
    #[cfg(feature = "CssBoxType")]
    #[doc = "Change the `toBox` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConvertCoordinateOptions`, `CssBoxType`*"]
    pub fn to_box(&mut self, val: CssBoxType) -> &mut Self {
        self.set_to_box_shim(val);
        self
    }
}
impl Default for ConvertCoordinateOptions {
    fn default() -> Self {
        Self::new()
    }
}
