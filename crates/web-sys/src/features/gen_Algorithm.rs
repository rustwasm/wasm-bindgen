#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = Algorithm)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `Algorithm` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Algorithm`*"]
    pub type Algorithm;
    #[wasm_bindgen(method, getter = "name")]
    fn name_shim(this: &Algorithm) -> String;
    #[wasm_bindgen(method, setter = "name")]
    fn set_name_shim(this: &Algorithm, val: &str);
}
#[doc = "The trait to access properties on the `Algorithm` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `Algorithm`*"]
pub trait AlgorithmGetters {
    #[doc = "Get the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Algorithm`*"]
    fn name(&self) -> String;
}
impl AlgorithmGetters for Algorithm {
    fn name(&self) -> String {
        self.name_shim()
    }
}
impl Algorithm {
    #[doc = "Construct a new `Algorithm`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Algorithm`*"]
    pub fn new(name: &str) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.name(name);
        ret
    }
    #[doc = "Change the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Algorithm`*"]
    pub fn name(&mut self, val: &str) -> &mut Self {
        self.set_name_shim(val);
        self
    }
}
