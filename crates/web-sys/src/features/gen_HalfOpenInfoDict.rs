#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = HalfOpenInfoDict)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HalfOpenInfoDict` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HalfOpenInfoDict`*"]
    pub type HalfOpenInfoDict;
    #[wasm_bindgen(method, getter = "speculative")]
    fn speculative_shim(this: &HalfOpenInfoDict) -> bool;
    #[wasm_bindgen(method, setter = "speculative")]
    fn set_speculative_shim(this: &HalfOpenInfoDict, val: bool);
}
#[doc = "The trait to access properties on the `HalfOpenInfoDict` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `HalfOpenInfoDict`*"]
pub trait HalfOpenInfoDictGetters {
    #[doc = "Get the `speculative` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HalfOpenInfoDict`*"]
    fn speculative(&self) -> bool;
}
impl HalfOpenInfoDictGetters for HalfOpenInfoDict {
    fn speculative(&self) -> bool {
        self.speculative_shim()
    }
}
impl HalfOpenInfoDict {
    #[doc = "Construct a new `HalfOpenInfoDict`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HalfOpenInfoDict`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `speculative` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HalfOpenInfoDict`*"]
    pub fn speculative(&mut self, val: bool) -> &mut Self {
        self.set_speculative_shim(val);
        self
    }
}
impl Default for HalfOpenInfoDict {
    fn default() -> Self {
        Self::new()
    }
}
