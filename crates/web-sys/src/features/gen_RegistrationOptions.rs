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
    #[wasm_bindgen(method, getter = "scope")]
    fn scope_shim(this: &RegistrationOptions) -> &str;
    #[wasm_bindgen(method, setter = "scope")]
    fn set_scope_shim(this: &RegistrationOptions, val: &str);
    #[wasm_bindgen(method, getter = "type")]
    fn type__shim(this: &RegistrationOptions) -> &str;
    #[wasm_bindgen(method, setter = "type")]
    fn set_type__shim(this: &RegistrationOptions, val: &str);
    #[cfg(feature = "ServiceWorkerUpdateViaCache")]
    #[wasm_bindgen(method, getter = "updateViaCache")]
    fn update_via_cache_shim(this: &RegistrationOptions) -> ServiceWorkerUpdateViaCache;
    #[cfg(feature = "ServiceWorkerUpdateViaCache")]
    #[wasm_bindgen(method, setter = "updateViaCache")]
    fn set_update_via_cache_shim(this: &RegistrationOptions, val: ServiceWorkerUpdateViaCache);
}
#[doc = "The trait to access properties on the `RegistrationOptions` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `RegistrationOptions`*"]
pub trait RegistrationOptionsGetters {
    #[doc = "Get the `scope` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RegistrationOptions`*"]
    fn scope(&self) -> &str;
    #[doc = "Get the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RegistrationOptions`*"]
    fn type_(&self) -> &str;
    #[cfg(feature = "ServiceWorkerUpdateViaCache")]
    #[doc = "Get the `updateViaCache` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RegistrationOptions`, `ServiceWorkerUpdateViaCache`*"]
    fn update_via_cache(&self) -> ServiceWorkerUpdateViaCache;
}
impl RegistrationOptionsGetters for RegistrationOptions {
    fn scope(&self) -> &str {
        self.scope_shim()
    }
    fn type_(&self) -> &str {
        self.type__shim()
    }
    #[cfg(feature = "ServiceWorkerUpdateViaCache")]
    fn update_via_cache(&self) -> ServiceWorkerUpdateViaCache {
        self.update_via_cache_shim()
    }
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
        self.set_scope_shim(val);
        self
    }
    #[doc = "Change the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RegistrationOptions`*"]
    pub fn type_(&mut self, val: &str) -> &mut Self {
        self.set_type__shim(val);
        self
    }
    #[cfg(feature = "ServiceWorkerUpdateViaCache")]
    #[doc = "Change the `updateViaCache` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RegistrationOptions`, `ServiceWorkerUpdateViaCache`*"]
    pub fn update_via_cache(&mut self, val: ServiceWorkerUpdateViaCache) -> &mut Self {
        self.set_update_via_cache_shim(val);
        self
    }
}
impl Default for RegistrationOptions {
    fn default() -> Self {
        Self::new()
    }
}
