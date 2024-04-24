#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = WorkletOptions)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `WorkletOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WorkletOptions`*"]
    pub type WorkletOptions;
    #[cfg(feature = "RequestCredentials")]
    #[wasm_bindgen(method, getter = "credentials")]
    fn credentials_shim(this: &WorkletOptions) -> RequestCredentials;
    #[cfg(feature = "RequestCredentials")]
    #[wasm_bindgen(method, setter = "credentials")]
    fn set_credentials_shim(this: &WorkletOptions, val: RequestCredentials);
}
#[doc = "The trait to access properties on the `WorkletOptions` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `WorkletOptions`*"]
pub trait WorkletOptionsGetters {
    #[cfg(feature = "RequestCredentials")]
    #[doc = "Get the `credentials` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RequestCredentials`, `WorkletOptions`*"]
    fn credentials(&self) -> RequestCredentials;
}
impl WorkletOptionsGetters for WorkletOptions {
    #[cfg(feature = "RequestCredentials")]
    fn credentials(&self) -> RequestCredentials {
        self.credentials_shim()
    }
}
impl WorkletOptions {
    #[doc = "Construct a new `WorkletOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WorkletOptions`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(feature = "RequestCredentials")]
    #[doc = "Change the `credentials` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RequestCredentials`, `WorkletOptions`*"]
    pub fn credentials(&mut self, val: RequestCredentials) -> &mut Self {
        self.set_credentials_shim(val);
        self
    }
}
impl Default for WorkletOptions {
    fn default() -> Self {
        Self::new()
    }
}
