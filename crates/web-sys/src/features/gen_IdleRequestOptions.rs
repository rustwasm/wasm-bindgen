#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = IdleRequestOptions)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `IdleRequestOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdleRequestOptions`*"]
    pub type IdleRequestOptions;
    #[wasm_bindgen(method, setter = "timeout")]
    fn timeout_shim(this: &IdleRequestOptions, val: u32);
}
impl IdleRequestOptions {
    #[doc = "Construct a new `IdleRequestOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdleRequestOptions`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `timeout` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdleRequestOptions`*"]
    pub fn timeout(&mut self, val: u32) -> &mut Self {
        self.timeout_shim(val);
        self
    }
}
impl Default for IdleRequestOptions {
    fn default() -> Self {
        Self::new()
    }
}
