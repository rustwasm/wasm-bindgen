#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = ElementDefinitionOptions)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ElementDefinitionOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ElementDefinitionOptions`*"]
    pub type ElementDefinitionOptions;
    #[wasm_bindgen(method, setter = "extends")]
    fn extends_shim(this: &ElementDefinitionOptions, val: &str);
}
impl ElementDefinitionOptions {
    #[doc = "Construct a new `ElementDefinitionOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ElementDefinitionOptions`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `extends` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ElementDefinitionOptions`*"]
    pub fn extends(&mut self, val: &str) -> &mut Self {
        self.extends_shim(val);
        self
    }
}
impl Default for ElementDefinitionOptions {
    fn default() -> Self {
        Self::new()
    }
}
