#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = RTCIdentityAssertionResult)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcIdentityAssertionResult` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIdentityAssertionResult`*"]
    pub type RtcIdentityAssertionResult;
    #[wasm_bindgen(method, getter = "assertion")]
    fn assertion_shim(this: &RtcIdentityAssertionResult) -> String;
    #[wasm_bindgen(method, setter = "assertion")]
    fn set_assertion_shim(this: &RtcIdentityAssertionResult, val: &str);
    #[cfg(feature = "RtcIdentityProviderDetails")]
    #[wasm_bindgen(method, getter = "idp")]
    fn idp_shim(this: &RtcIdentityAssertionResult) -> RtcIdentityProviderDetails;
    #[cfg(feature = "RtcIdentityProviderDetails")]
    #[wasm_bindgen(method, setter = "idp")]
    fn set_idp_shim(this: &RtcIdentityAssertionResult, val: &RtcIdentityProviderDetails);
}
#[doc = "The trait to access properties on the `RtcIdentityAssertionResult` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `RtcIdentityAssertionResult`*"]
pub trait RtcIdentityAssertionResultGetters {
    #[doc = "Get the `assertion` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIdentityAssertionResult`*"]
    fn assertion(&self) -> String;
    #[cfg(feature = "RtcIdentityProviderDetails")]
    #[doc = "Get the `idp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIdentityAssertionResult`, `RtcIdentityProviderDetails`*"]
    fn idp(&self) -> RtcIdentityProviderDetails;
}
impl RtcIdentityAssertionResultGetters for RtcIdentityAssertionResult {
    fn assertion(&self) -> String {
        self.assertion_shim()
    }
    #[cfg(feature = "RtcIdentityProviderDetails")]
    fn idp(&self) -> RtcIdentityProviderDetails {
        self.idp_shim()
    }
}
impl RtcIdentityAssertionResult {
    #[cfg(feature = "RtcIdentityProviderDetails")]
    #[doc = "Construct a new `RtcIdentityAssertionResult`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIdentityAssertionResult`, `RtcIdentityProviderDetails`*"]
    pub fn new(assertion: &str, idp: &RtcIdentityProviderDetails) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        Self::assertion(&mut ret, assertion);
        Self::idp(&mut ret, idp);
        ret
    }
    #[doc = "Change the `assertion` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIdentityAssertionResult`*"]
    pub fn assertion(&mut self, val: &str) -> &mut Self {
        self.set_assertion_shim(val);
        self
    }
    #[cfg(feature = "RtcIdentityProviderDetails")]
    #[doc = "Change the `idp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIdentityAssertionResult`, `RtcIdentityProviderDetails`*"]
    pub fn idp(&mut self, val: &RtcIdentityProviderDetails) -> &mut Self {
        self.set_idp_shim(val);
        self
    }
}
