#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = RTCRtpHeaderExtensionParameters)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcRtpHeaderExtensionParameters` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpHeaderExtensionParameters`*"]
    pub type RtcRtpHeaderExtensionParameters;
    #[wasm_bindgen(method, getter = "encrypted")]
    fn encrypted_shim(this: &RtcRtpHeaderExtensionParameters) -> bool;
    #[wasm_bindgen(method, setter = "encrypted")]
    fn set_encrypted_shim(this: &RtcRtpHeaderExtensionParameters, val: bool);
    #[wasm_bindgen(method, getter = "id")]
    fn id_shim(this: &RtcRtpHeaderExtensionParameters) -> u16;
    #[wasm_bindgen(method, setter = "id")]
    fn set_id_shim(this: &RtcRtpHeaderExtensionParameters, val: u16);
    #[wasm_bindgen(method, getter = "uri")]
    fn uri_shim(this: &RtcRtpHeaderExtensionParameters) -> String;
    #[wasm_bindgen(method, setter = "uri")]
    fn set_uri_shim(this: &RtcRtpHeaderExtensionParameters, val: &str);
}
#[doc = "The trait to access properties on the `RtcRtpHeaderExtensionParameters` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `RtcRtpHeaderExtensionParameters`*"]
pub trait RtcRtpHeaderExtensionParametersGetters {
    #[doc = "Get the `encrypted` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpHeaderExtensionParameters`*"]
    fn encrypted(&self) -> bool;
    #[doc = "Get the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpHeaderExtensionParameters`*"]
    fn id(&self) -> u16;
    #[doc = "Get the `uri` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpHeaderExtensionParameters`*"]
    fn uri(&self) -> String;
}
impl RtcRtpHeaderExtensionParametersGetters for RtcRtpHeaderExtensionParameters {
    fn encrypted(&self) -> bool {
        self.encrypted_shim()
    }
    fn id(&self) -> u16 {
        self.id_shim()
    }
    fn uri(&self) -> String {
        self.uri_shim()
    }
}
impl RtcRtpHeaderExtensionParameters {
    #[doc = "Construct a new `RtcRtpHeaderExtensionParameters`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpHeaderExtensionParameters`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `encrypted` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpHeaderExtensionParameters`*"]
    pub fn encrypted(&mut self, val: bool) -> &mut Self {
        self.set_encrypted_shim(val);
        self
    }
    #[doc = "Change the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpHeaderExtensionParameters`*"]
    pub fn id(&mut self, val: u16) -> &mut Self {
        self.set_id_shim(val);
        self
    }
    #[doc = "Change the `uri` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpHeaderExtensionParameters`*"]
    pub fn uri(&mut self, val: &str) -> &mut Self {
        self.set_uri_shim(val);
        self
    }
}
impl Default for RtcRtpHeaderExtensionParameters {
    fn default() -> Self {
        Self::new()
    }
}
