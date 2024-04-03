#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = ConstrainBooleanParameters)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ConstrainBooleanParameters` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstrainBooleanParameters`*"]
    pub type ConstrainBooleanParameters;
    #[wasm_bindgen(method, setter = "exact")]
    fn exact_shim(this: &ConstrainBooleanParameters, val: bool);
    #[wasm_bindgen(method, setter = "ideal")]
    fn ideal_shim(this: &ConstrainBooleanParameters, val: bool);
}
impl ConstrainBooleanParameters {
    #[doc = "Construct a new `ConstrainBooleanParameters`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstrainBooleanParameters`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `exact` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstrainBooleanParameters`*"]
    pub fn exact(&mut self, val: bool) -> &mut Self {
        self.exact_shim(val);
        self
    }
    #[doc = "Change the `ideal` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstrainBooleanParameters`*"]
    pub fn ideal(&mut self, val: bool) -> &mut Self {
        self.ideal_shim(val);
        self
    }
}
impl Default for ConstrainBooleanParameters {
    fn default() -> Self {
        Self::new()
    }
}
