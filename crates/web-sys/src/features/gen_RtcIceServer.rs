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
    #[wasm_bindgen(method, getter = "credential")]
    fn credential_shim(this: &RtcIceServer) -> &str;
    #[wasm_bindgen(method, setter = "credential")]
    fn set_credential_shim(this: &RtcIceServer, val: &str);
    #[cfg(feature = "RtcIceCredentialType")]
    #[wasm_bindgen(method, getter = "credentialType")]
    fn credential_type_shim(this: &RtcIceServer) -> RtcIceCredentialType;
    #[cfg(feature = "RtcIceCredentialType")]
    #[wasm_bindgen(method, setter = "credentialType")]
    fn set_credential_type_shim(this: &RtcIceServer, val: RtcIceCredentialType);
    #[wasm_bindgen(method, getter = "url")]
    fn url_shim(this: &RtcIceServer) -> &str;
    #[wasm_bindgen(method, setter = "url")]
    fn set_url_shim(this: &RtcIceServer, val: &str);
    #[wasm_bindgen(method, getter = "urls")]
    fn urls_shim(this: &RtcIceServer) -> &::wasm_bindgen::JsValue;
    #[wasm_bindgen(method, setter = "urls")]
    fn set_urls_shim(this: &RtcIceServer, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, getter = "username")]
    fn username_shim(this: &RtcIceServer) -> &str;
    #[wasm_bindgen(method, setter = "username")]
    fn set_username_shim(this: &RtcIceServer, val: &str);
}
#[doc = "The trait to access properties on the `RtcIceServer` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `RtcIceServer`*"]
pub trait RtcIceServerGetters {
    #[doc = "Get the `credential` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceServer`*"]
    fn credential(&self) -> &str;
    #[cfg(feature = "RtcIceCredentialType")]
    #[doc = "Get the `credentialType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCredentialType`, `RtcIceServer`*"]
    fn credential_type(&self) -> RtcIceCredentialType;
    #[doc = "Get the `url` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceServer`*"]
    fn url(&self) -> &str;
    #[doc = "Get the `urls` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceServer`*"]
    fn urls(&self) -> &::wasm_bindgen::JsValue;
    #[doc = "Get the `username` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceServer`*"]
    fn username(&self) -> &str;
}
impl RtcIceServerGetters for RtcIceServer {
    fn credential(&self) -> &str {
        self.credential_shim()
    }
    #[cfg(feature = "RtcIceCredentialType")]
    fn credential_type(&self) -> RtcIceCredentialType {
        self.credential_type_shim()
    }
    fn url(&self) -> &str {
        self.url_shim()
    }
    fn urls(&self) -> &::wasm_bindgen::JsValue {
        self.urls_shim()
    }
    fn username(&self) -> &str {
        self.username_shim()
    }
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
        self.set_credential_shim(val);
        self
    }
    #[cfg(feature = "RtcIceCredentialType")]
    #[doc = "Change the `credentialType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCredentialType`, `RtcIceServer`*"]
    pub fn credential_type(&mut self, val: RtcIceCredentialType) -> &mut Self {
        self.set_credential_type_shim(val);
        self
    }
    #[doc = "Change the `url` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceServer`*"]
    pub fn url(&mut self, val: &str) -> &mut Self {
        self.set_url_shim(val);
        self
    }
    #[doc = "Change the `urls` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceServer`*"]
    pub fn urls(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_urls_shim(val);
        self
    }
    #[doc = "Change the `username` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceServer`*"]
    pub fn username(&mut self, val: &str) -> &mut Self {
        self.set_username_shim(val);
        self
    }
}
impl Default for RtcIceServer {
    fn default() -> Self {
        Self::new()
    }
}
