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
    #[wasm_bindgen(method, getter = "includeUncontrolled")]
    fn include_uncontrolled_shim(this: &ClientQueryOptions) -> bool;
    #[wasm_bindgen(method, setter = "includeUncontrolled")]
    fn set_include_uncontrolled_shim(this: &ClientQueryOptions, val: bool);
    #[cfg(feature = "ClientType")]
    #[wasm_bindgen(method, getter = "type")]
    fn type__shim(this: &ClientQueryOptions) -> ClientType;
    #[cfg(feature = "ClientType")]
    #[wasm_bindgen(method, setter = "type")]
    fn set_type__shim(this: &ClientQueryOptions, val: ClientType);
}
#[doc = "The trait to access properties on the `ClientQueryOptions` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `ClientQueryOptions`*"]
pub trait ClientQueryOptionsGetters {
    #[doc = "Get the `includeUncontrolled` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ClientQueryOptions`*"]
    fn include_uncontrolled(&self) -> bool;
    #[cfg(feature = "ClientType")]
    #[doc = "Get the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ClientQueryOptions`, `ClientType`*"]
    fn type_(&self) -> ClientType;
}
impl ClientQueryOptionsGetters for ClientQueryOptions {
    fn include_uncontrolled(&self) -> bool {
        self.include_uncontrolled_shim()
    }
    #[cfg(feature = "ClientType")]
    fn type_(&self) -> ClientType {
        self.type__shim()
    }
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
        self.set_include_uncontrolled_shim(val);
        self
    }
    #[cfg(feature = "ClientType")]
    #[doc = "Change the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ClientQueryOptions`, `ClientType`*"]
    pub fn type_(&mut self, val: ClientType) -> &mut Self {
        self.set_type__shim(val);
        self
    }
}
impl Default for ClientQueryOptions {
    fn default() -> Self {
        Self::new()
    }
}
