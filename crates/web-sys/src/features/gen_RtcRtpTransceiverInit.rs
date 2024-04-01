#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = RTCRtpTransceiverInit)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcRtpTransceiverInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpTransceiverInit`*"]
    pub type RtcRtpTransceiverInit;
    #[cfg(feature = "RtcRtpTransceiverDirection")]
    #[wasm_bindgen(method, setter = "direction")]
    fn direction_shim(this: &RtcRtpTransceiverInit, val: RtcRtpTransceiverDirection);
    #[wasm_bindgen(method, setter = "sendEncodings")]
    fn send_encodings_shim(this: &RtcRtpTransceiverInit, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, setter = "streams")]
    fn streams_shim(this: &RtcRtpTransceiverInit, val: &::wasm_bindgen::JsValue);
}
impl RtcRtpTransceiverInit {
    #[doc = "Construct a new `RtcRtpTransceiverInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpTransceiverInit`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(feature = "RtcRtpTransceiverDirection")]
    #[doc = "Change the `direction` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpTransceiverDirection`, `RtcRtpTransceiverInit`*"]
    pub fn direction(&mut self, val: RtcRtpTransceiverDirection) -> &mut Self {
        self.direction_shim(val);
        self
    }
    #[doc = "Change the `sendEncodings` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpTransceiverInit`*"]
    pub fn send_encodings(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.send_encodings_shim(val);
        self
    }
    #[doc = "Change the `streams` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpTransceiverInit`*"]
    pub fn streams(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.streams_shim(val);
        self
    }
}
impl Default for RtcRtpTransceiverInit {
    fn default() -> Self {
        Self::new()
    }
}
