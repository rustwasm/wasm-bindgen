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
    #[wasm_bindgen(method, setter = "encrypted")]
    fn encrypted_shim(this: &RtcRtpHeaderExtensionParameters, val: bool);
    #[wasm_bindgen(method, setter = "id")]
    fn id_shim(this: &RtcRtpHeaderExtensionParameters, val: u16);
    #[wasm_bindgen(method, setter = "uri")]
    fn uri_shim(this: &RtcRtpHeaderExtensionParameters, val: &str);
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
        self.encrypted_shim(val);
        self
    }
    #[doc = "Change the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpHeaderExtensionParameters`*"]
    pub fn id(&mut self, val: u16) -> &mut Self {
        self.id_shim(val);
        self
    }
    #[doc = "Change the `uri` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpHeaderExtensionParameters`*"]
    pub fn uri(&mut self, val: &str) -> &mut Self {
        self.uri_shim(val);
        self
    }
}
impl Default for RtcRtpHeaderExtensionParameters {
    fn default() -> Self {
        Self::new()
    }
}
