#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = BoxQuadOptions)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `BoxQuadOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BoxQuadOptions`*"]
    pub type BoxQuadOptions;
    #[cfg(feature = "CssBoxType")]
    #[wasm_bindgen(method, getter = "box")]
    fn box__shim(this: &BoxQuadOptions) -> CssBoxType;
    #[cfg(feature = "CssBoxType")]
    #[wasm_bindgen(method, setter = "box")]
    fn set_box__shim(this: &BoxQuadOptions, val: CssBoxType);
    #[wasm_bindgen(method, getter = "relativeTo")]
    fn relative_to_shim(this: &BoxQuadOptions) -> &::js_sys::Object;
    #[wasm_bindgen(method, setter = "relativeTo")]
    fn set_relative_to_shim(this: &BoxQuadOptions, val: &::js_sys::Object);
}
#[doc = "The trait to access properties on the `BoxQuadOptions` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `BoxQuadOptions`*"]
pub trait BoxQuadOptionsGetters {
    #[cfg(feature = "CssBoxType")]
    #[doc = "Get the `box` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BoxQuadOptions`, `CssBoxType`*"]
    fn box_(&self) -> CssBoxType;
    #[doc = "Get the `relativeTo` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BoxQuadOptions`*"]
    fn relative_to(&self) -> &::js_sys::Object;
}
impl BoxQuadOptionsGetters for BoxQuadOptions {
    #[cfg(feature = "CssBoxType")]
    fn box_(&self) -> CssBoxType {
        self.box__shim()
    }
    fn relative_to(&self) -> &::js_sys::Object {
        self.relative_to_shim()
    }
}
impl BoxQuadOptions {
    #[doc = "Construct a new `BoxQuadOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BoxQuadOptions`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(feature = "CssBoxType")]
    #[doc = "Change the `box` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BoxQuadOptions`, `CssBoxType`*"]
    pub fn box_(&mut self, val: CssBoxType) -> &mut Self {
        self.set_box__shim(val);
        self
    }
    #[doc = "Change the `relativeTo` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BoxQuadOptions`*"]
    pub fn relative_to(&mut self, val: &::js_sys::Object) -> &mut Self {
        self.set_relative_to_shim(val);
        self
    }
}
impl Default for BoxQuadOptions {
    fn default() -> Self {
        Self::new()
    }
}
