#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = WorkerOptions)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `WorkerOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WorkerOptions`*"]
    pub type WorkerOptions;
    #[cfg(feature = "RequestCredentials")]
    #[wasm_bindgen(method, getter = "credentials")]
    fn credentials_shim(this: &WorkerOptions) -> RequestCredentials;
    #[cfg(feature = "RequestCredentials")]
    #[wasm_bindgen(method, setter = "credentials")]
    fn set_credentials_shim(this: &WorkerOptions, val: RequestCredentials);
    #[wasm_bindgen(method, getter = "name")]
    fn name_shim(this: &WorkerOptions) -> &str;
    #[wasm_bindgen(method, setter = "name")]
    fn set_name_shim(this: &WorkerOptions, val: &str);
    #[cfg(feature = "WorkerType")]
    #[wasm_bindgen(method, getter = "type")]
    fn type__shim(this: &WorkerOptions) -> WorkerType;
    #[cfg(feature = "WorkerType")]
    #[wasm_bindgen(method, setter = "type")]
    fn set_type__shim(this: &WorkerOptions, val: WorkerType);
}
#[doc = "The trait to access properties on the `WorkerOptions` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `WorkerOptions`*"]
pub trait WorkerOptionsGetters {
    #[cfg(feature = "RequestCredentials")]
    #[doc = "Get the `credentials` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RequestCredentials`, `WorkerOptions`*"]
    fn credentials(&self) -> RequestCredentials;
    #[doc = "Get the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WorkerOptions`*"]
    fn name(&self) -> &str;
    #[cfg(feature = "WorkerType")]
    #[doc = "Get the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WorkerOptions`, `WorkerType`*"]
    fn type_(&self) -> WorkerType;
}
impl WorkerOptionsGetters for WorkerOptions {
    #[cfg(feature = "RequestCredentials")]
    fn credentials(&self) -> RequestCredentials {
        self.credentials_shim()
    }
    fn name(&self) -> &str {
        self.name_shim()
    }
    #[cfg(feature = "WorkerType")]
    fn type_(&self) -> WorkerType {
        self.type__shim()
    }
}
impl WorkerOptions {
    #[doc = "Construct a new `WorkerOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WorkerOptions`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(feature = "RequestCredentials")]
    #[doc = "Change the `credentials` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RequestCredentials`, `WorkerOptions`*"]
    pub fn credentials(&mut self, val: RequestCredentials) -> &mut Self {
        self.set_credentials_shim(val);
        self
    }
    #[doc = "Change the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WorkerOptions`*"]
    pub fn name(&mut self, val: &str) -> &mut Self {
        self.set_name_shim(val);
        self
    }
    #[cfg(feature = "WorkerType")]
    #[doc = "Change the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WorkerOptions`, `WorkerType`*"]
    pub fn type_(&mut self, val: WorkerType) -> &mut Self {
        self.set_type__shim(val);
        self
    }
}
impl Default for WorkerOptions {
    fn default() -> Self {
        Self::new()
    }
}
