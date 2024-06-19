#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = EcKeyGenParams)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `EcKeyGenParams` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EcKeyGenParams`*"]
    pub type EcKeyGenParams;
    #[doc = "Get the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EcKeyGenParams`*"]
    #[wasm_bindgen(method, getter = "name")]
    pub fn get_name(this: &EcKeyGenParams) -> String;
    #[wasm_bindgen(method, setter = "name")]
    fn set_name(this: &EcKeyGenParams, val: &str);
    #[doc = "Get the `namedCurve` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EcKeyGenParams`*"]
    #[wasm_bindgen(method, getter = "namedCurve")]
    pub fn get_named_curve(this: &EcKeyGenParams) -> String;
    #[wasm_bindgen(method, setter = "namedCurve")]
    fn set_named_curve(this: &EcKeyGenParams, val: &str);
}
impl EcKeyGenParams {
    #[doc = "Construct a new `EcKeyGenParams`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EcKeyGenParams`*"]
    pub fn new(name: &str, named_curve: &str) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.name(name);
        ret.named_curve(named_curve);
        ret
    }
    #[doc = "Change the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EcKeyGenParams`*"]
    pub fn name(&mut self, val: &str) -> &mut Self {
        self.set_name(val);
        self
    }
    #[doc = "Change the `namedCurve` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EcKeyGenParams`*"]
    pub fn named_curve(&mut self, val: &str) -> &mut Self {
        self.set_named_curve(val);
        self
    }
}
