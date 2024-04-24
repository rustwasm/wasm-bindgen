#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = ElementCreationOptions)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ElementCreationOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ElementCreationOptions`*"]
    pub type ElementCreationOptions;
    #[wasm_bindgen(method, getter = "is")]
    fn is_shim(this: &ElementCreationOptions) -> String;
    #[wasm_bindgen(method, setter = "is")]
    fn set_is_shim(this: &ElementCreationOptions, val: &str);
    #[wasm_bindgen(method, getter = "pseudo")]
    fn pseudo_shim(this: &ElementCreationOptions) -> String;
    #[wasm_bindgen(method, setter = "pseudo")]
    fn set_pseudo_shim(this: &ElementCreationOptions, val: &str);
}
#[doc = "The trait to access properties on the `ElementCreationOptions` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `ElementCreationOptions`*"]
pub trait ElementCreationOptionsGetters {
    #[doc = "Get the `is` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ElementCreationOptions`*"]
    fn is(&self) -> String;
    #[doc = "Get the `pseudo` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ElementCreationOptions`*"]
    fn pseudo(&self) -> String;
}
impl ElementCreationOptionsGetters for ElementCreationOptions {
    fn is(&self) -> String {
        self.is_shim()
    }
    fn pseudo(&self) -> String {
        self.pseudo_shim()
    }
}
impl ElementCreationOptions {
    #[doc = "Construct a new `ElementCreationOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ElementCreationOptions`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `is` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ElementCreationOptions`*"]
    pub fn is(&mut self, val: &str) -> &mut Self {
        self.set_is_shim(val);
        self
    }
    #[doc = "Change the `pseudo` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ElementCreationOptions`*"]
    pub fn pseudo(&mut self, val: &str) -> &mut Self {
        self.set_pseudo_shim(val);
        self
    }
}
impl Default for ElementCreationOptions {
    fn default() -> Self {
        Self::new()
    }
}
