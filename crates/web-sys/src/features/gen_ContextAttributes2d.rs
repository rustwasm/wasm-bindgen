#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = ContextAttributes2D)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ContextAttributes2d` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ContextAttributes2d`*"]
    pub type ContextAttributes2d;
    #[wasm_bindgen(method, getter = "alpha")]
    fn alpha_shim(this: &ContextAttributes2d) -> bool;
    #[wasm_bindgen(method, setter = "alpha")]
    fn set_alpha_shim(this: &ContextAttributes2d, val: bool);
    #[wasm_bindgen(method, getter = "willReadFrequently")]
    fn will_read_frequently_shim(this: &ContextAttributes2d) -> bool;
    #[wasm_bindgen(method, setter = "willReadFrequently")]
    fn set_will_read_frequently_shim(this: &ContextAttributes2d, val: bool);
}
#[doc = "The trait to access properties on the `ContextAttributes2d` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `ContextAttributes2d`*"]
pub trait ContextAttributes2dGetters {
    #[doc = "Get the `alpha` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ContextAttributes2d`*"]
    fn alpha(&self) -> bool;
    #[doc = "Get the `willReadFrequently` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ContextAttributes2d`*"]
    fn will_read_frequently(&self) -> bool;
}
impl ContextAttributes2dGetters for ContextAttributes2d {
    fn alpha(&self) -> bool {
        self.alpha_shim()
    }
    fn will_read_frequently(&self) -> bool {
        self.will_read_frequently_shim()
    }
}
impl ContextAttributes2d {
    #[doc = "Construct a new `ContextAttributes2d`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ContextAttributes2d`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `alpha` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ContextAttributes2d`*"]
    pub fn alpha(&mut self, val: bool) -> &mut Self {
        self.set_alpha_shim(val);
        self
    }
    #[doc = "Change the `willReadFrequently` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ContextAttributes2d`*"]
    pub fn will_read_frequently(&mut self, val: bool) -> &mut Self {
        self.set_will_read_frequently_shim(val);
        self
    }
}
impl Default for ContextAttributes2d {
    fn default() -> Self {
        Self::new()
    }
}
