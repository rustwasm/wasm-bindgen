#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = ClientQueryOptions)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ClientQueryOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ClientQueryOptions`*"]
    pub type ClientQueryOptions;
    #[wasm_bindgen(method, setter = "includeUncontrolled")]
    fn include_uncontrolled_shim(this: &ClientQueryOptions, val: bool);
    #[cfg(feature = "ClientType")]
    #[wasm_bindgen(method, setter = "type")]
    fn type__shim(this: &ClientQueryOptions, val: ClientType);
}
impl ClientQueryOptions {
    #[doc = "Construct a new `ClientQueryOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ClientQueryOptions`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `includeUncontrolled` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ClientQueryOptions`*"]
    pub fn include_uncontrolled(&mut self, val: bool) -> &mut Self {
        self.include_uncontrolled_shim(val);
        self
    }
    #[cfg(feature = "ClientType")]
    #[doc = "Change the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ClientQueryOptions`, `ClientType`*"]
    pub fn type_(&mut self, val: ClientType) -> &mut Self {
        self.type__shim(val);
        self
    }
}
impl Default for ClientQueryOptions {
    fn default() -> Self {
        Self::new()
    }
}
