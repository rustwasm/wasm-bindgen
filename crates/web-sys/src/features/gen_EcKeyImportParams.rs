#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = EcKeyImportParams)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `EcKeyImportParams` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EcKeyImportParams`*"]
    pub type EcKeyImportParams;
    #[wasm_bindgen(method, getter = "name")]
    fn name_shim(this: &EcKeyImportParams) -> String;
    #[wasm_bindgen(method, setter = "name")]
    fn set_name_shim(this: &EcKeyImportParams, val: &str);
    #[wasm_bindgen(method, getter = "namedCurve")]
    fn named_curve_shim(this: &EcKeyImportParams) -> String;
    #[wasm_bindgen(method, setter = "namedCurve")]
    fn set_named_curve_shim(this: &EcKeyImportParams, val: &str);
}
#[doc = "The trait to access properties on the `EcKeyImportParams` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `EcKeyImportParams`*"]
pub trait EcKeyImportParamsGetters {
    #[doc = "Get the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EcKeyImportParams`*"]
    fn name(&self) -> String;
    #[doc = "Get the `namedCurve` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EcKeyImportParams`*"]
    fn named_curve(&self) -> String;
}
impl EcKeyImportParamsGetters for EcKeyImportParams {
    fn name(&self) -> String {
        self.name_shim()
    }
    fn named_curve(&self) -> String {
        self.named_curve_shim()
    }
}
impl EcKeyImportParams {
    #[doc = "Construct a new `EcKeyImportParams`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EcKeyImportParams`*"]
    pub fn new(name: &str) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        Self::name(&mut ret, name);
        ret
    }
    #[doc = "Change the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EcKeyImportParams`*"]
    pub fn name(&mut self, val: &str) -> &mut Self {
        self.set_name_shim(val);
        self
    }
    #[doc = "Change the `namedCurve` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EcKeyImportParams`*"]
    pub fn named_curve(&mut self, val: &str) -> &mut Self {
        self.set_named_curve_shim(val);
        self
    }
}
