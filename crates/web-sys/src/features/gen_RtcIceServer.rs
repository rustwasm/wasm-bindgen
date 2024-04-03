#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = RTCIceServer)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcIceServer` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceServer`*"]
    pub type RtcIceServer;
    #[wasm_bindgen(method, setter = "credential")]
    fn credential_shim(this: &RtcIceServer, val: &str);
    #[cfg(feature = "RtcIceCredentialType")]
    #[wasm_bindgen(method, setter = "credentialType")]
    fn credential_type_shim(this: &RtcIceServer, val: RtcIceCredentialType);
    #[wasm_bindgen(method, setter = "url")]
    fn url_shim(this: &RtcIceServer, val: &str);
    #[wasm_bindgen(method, setter = "urls")]
    fn urls_shim(this: &RtcIceServer, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, setter = "username")]
    fn username_shim(this: &RtcIceServer, val: &str);
}
impl RtcIceServer {
    #[doc = "Construct a new `RtcIceServer`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceServer`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `credential` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceServer`*"]
    pub fn credential(&mut self, val: &str) -> &mut Self {
        self.credential_shim(val);
        self
    }
    #[cfg(feature = "RtcIceCredentialType")]
    #[doc = "Change the `credentialType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCredentialType`, `RtcIceServer`*"]
    pub fn credential_type(&mut self, val: RtcIceCredentialType) -> &mut Self {
        self.credential_type_shim(val);
        self
    }
    #[doc = "Change the `url` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceServer`*"]
    pub fn url(&mut self, val: &str) -> &mut Self {
        self.url_shim(val);
        self
    }
    #[doc = "Change the `urls` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceServer`*"]
    pub fn urls(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.urls_shim(val);
        self
    }
    #[doc = "Change the `username` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceServer`*"]
    pub fn username(&mut self, val: &str) -> &mut Self {
        self.username_shim(val);
        self
    }
}
impl Default for RtcIceServer {
    fn default() -> Self {
        Self::new()
    }
}
