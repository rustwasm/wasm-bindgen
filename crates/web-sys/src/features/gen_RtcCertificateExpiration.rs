#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = RTCCertificateExpiration)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcCertificateExpiration` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcCertificateExpiration`*"]
    pub type RtcCertificateExpiration;
    #[wasm_bindgen(method, getter = "expires")]
    fn expires_shim(this: &RtcCertificateExpiration) -> f64;
    #[wasm_bindgen(method, setter = "expires")]
    fn set_expires_shim(this: &RtcCertificateExpiration, val: f64);
}
#[doc = "The trait to access properties on the `RtcCertificateExpiration` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `RtcCertificateExpiration`*"]
pub trait RtcCertificateExpirationGetters {
    #[doc = "Get the `expires` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcCertificateExpiration`*"]
    fn expires(&self) -> f64;
}
impl RtcCertificateExpirationGetters for RtcCertificateExpiration {
    fn expires(&self) -> f64 {
        self.expires_shim()
    }
}
impl RtcCertificateExpiration {
    #[doc = "Construct a new `RtcCertificateExpiration`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcCertificateExpiration`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `expires` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcCertificateExpiration`*"]
    pub fn expires(&mut self, val: f64) -> &mut Self {
        self.set_expires_shim(val);
        self
    }
}
impl Default for RtcCertificateExpiration {
    fn default() -> Self {
        Self::new()
    }
}
