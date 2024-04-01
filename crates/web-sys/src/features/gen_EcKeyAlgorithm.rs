#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = EcKeyAlgorithm)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `EcKeyAlgorithm` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EcKeyAlgorithm`*"]
    pub type EcKeyAlgorithm;
    #[wasm_bindgen(method, setter = "name")]
    fn name_shim(this: &EcKeyAlgorithm, val: &str);
    #[wasm_bindgen(method, setter = "namedCurve")]
    fn named_curve_shim(this: &EcKeyAlgorithm, val: &str);
}
impl EcKeyAlgorithm {
    #[doc = "Construct a new `EcKeyAlgorithm`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EcKeyAlgorithm`*"]
    pub fn new(name: &str, named_curve: &str) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.name(name);
        ret.named_curve(named_curve);
        ret
    }
    #[doc = "Change the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EcKeyAlgorithm`*"]
    pub fn name(&mut self, val: &str) -> &mut Self {
        self.name_shim(val);
        self
    }
    #[doc = "Change the `namedCurve` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EcKeyAlgorithm`*"]
    pub fn named_curve(&mut self, val: &str) -> &mut Self {
        self.named_curve_shim(val);
        self
    }
}
