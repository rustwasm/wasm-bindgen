#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = RTCIdentityAssertion)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcIdentityAssertion` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIdentityAssertion`*"]
    pub type RtcIdentityAssertion;
    #[wasm_bindgen(method, getter = "idp")]
    fn idp_shim(this: &RtcIdentityAssertion) -> String;
    #[wasm_bindgen(method, setter = "idp")]
    fn set_idp_shim(this: &RtcIdentityAssertion, val: &str);
    #[wasm_bindgen(method, getter = "name")]
    fn name_shim(this: &RtcIdentityAssertion) -> String;
    #[wasm_bindgen(method, setter = "name")]
    fn set_name_shim(this: &RtcIdentityAssertion, val: &str);
}
#[doc = "The trait to access properties on the `RtcIdentityAssertion` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `RtcIdentityAssertion`*"]
pub trait RtcIdentityAssertionGetters {
    #[doc = "Get the `idp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIdentityAssertion`*"]
    fn idp(&self) -> String;
    #[doc = "Get the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIdentityAssertion`*"]
    fn name(&self) -> String;
}
impl RtcIdentityAssertionGetters for RtcIdentityAssertion {
    fn idp(&self) -> String {
        self.idp_shim()
    }
    fn name(&self) -> String {
        self.name_shim()
    }
}
impl RtcIdentityAssertion {
    #[doc = "Construct a new `RtcIdentityAssertion`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIdentityAssertion`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `idp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIdentityAssertion`*"]
    pub fn idp(&mut self, val: &str) -> &mut Self {
        self.set_idp_shim(val);
        self
    }
    #[doc = "Change the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIdentityAssertion`*"]
    pub fn name(&mut self, val: &str) -> &mut Self {
        self.set_name_shim(val);
        self
    }
}
impl Default for RtcIdentityAssertion {
    fn default() -> Self {
        Self::new()
    }
}
