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
    #[wasm_bindgen(method, setter = "name")]
    fn name_shim(this: &Algorithm, val: &str);
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
        self.name_shim(val);
        self
    }
}
