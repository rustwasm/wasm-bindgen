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
    #[wasm_bindgen(method, setter = "idp")]
    fn idp_shim(this: &RtcIdentityAssertion, val: &str);
    #[wasm_bindgen(method, setter = "name")]
    fn name_shim(this: &RtcIdentityAssertion, val: &str);
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
        self.idp_shim(val);
        self
    }
    #[doc = "Change the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIdentityAssertion`*"]
    pub fn name(&mut self, val: &str) -> &mut Self {
        self.name_shim(val);
        self
    }
}
impl Default for RtcIdentityAssertion {
    fn default() -> Self {
        Self::new()
    }
}
