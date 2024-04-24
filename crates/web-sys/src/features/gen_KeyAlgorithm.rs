#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = KeyAlgorithm)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `KeyAlgorithm` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyAlgorithm`*"]
    pub type KeyAlgorithm;
    #[wasm_bindgen(method, getter = "name")]
    fn name_shim(this: &KeyAlgorithm) -> String;
    #[wasm_bindgen(method, setter = "name")]
    fn set_name_shim(this: &KeyAlgorithm, val: &str);
}
#[doc = "The trait to access properties on the `KeyAlgorithm` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `KeyAlgorithm`*"]
pub trait KeyAlgorithmGetters {
    #[doc = "Get the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyAlgorithm`*"]
    fn name(&self) -> String;
}
impl KeyAlgorithmGetters for KeyAlgorithm {
    fn name(&self) -> String {
        self.name_shim()
    }
}
impl KeyAlgorithm {
    #[doc = "Construct a new `KeyAlgorithm`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyAlgorithm`*"]
    pub fn new(name: &str) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.name(name);
        ret
    }
    #[doc = "Change the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyAlgorithm`*"]
    pub fn name(&mut self, val: &str) -> &mut Self {
        self.set_name_shim(val);
        self
    }
}
