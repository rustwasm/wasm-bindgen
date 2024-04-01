#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = RegistrationOptions)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RegistrationOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RegistrationOptions`*"]
    pub type RegistrationOptions;
    #[wasm_bindgen(method, setter = "scope")]
    fn scope_shim(this: &RegistrationOptions, val: &str);
    #[wasm_bindgen(method, setter = "type")]
    fn type__shim(this: &RegistrationOptions, val: &str);
    #[cfg(feature = "ServiceWorkerUpdateViaCache")]
    #[wasm_bindgen(method, setter = "updateViaCache")]
    fn update_via_cache_shim(this: &RegistrationOptions, val: ServiceWorkerUpdateViaCache);
}
impl RegistrationOptions {
    #[doc = "Construct a new `RegistrationOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RegistrationOptions`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `scope` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RegistrationOptions`*"]
    pub fn scope(&mut self, val: &str) -> &mut Self {
        self.scope_shim(val);
        self
    }
    #[doc = "Change the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RegistrationOptions`*"]
    pub fn type_(&mut self, val: &str) -> &mut Self {
        self.type__shim(val);
        self
    }
    #[cfg(feature = "ServiceWorkerUpdateViaCache")]
    #[doc = "Change the `updateViaCache` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RegistrationOptions`, `ServiceWorkerUpdateViaCache`*"]
    pub fn update_via_cache(&mut self, val: ServiceWorkerUpdateViaCache) -> &mut Self {
        self.update_via_cache_shim(val);
        self
    }
}
impl Default for RegistrationOptions {
    fn default() -> Self {
        Self::new()
    }
}
