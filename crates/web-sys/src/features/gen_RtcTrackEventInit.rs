#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = RTCTrackEventInit)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcTrackEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcTrackEventInit`*"]
    pub type RtcTrackEventInit;
    #[wasm_bindgen(method, setter = "bubbles")]
    fn bubbles_shim(this: &RtcTrackEventInit, val: bool);
    #[wasm_bindgen(method, setter = "cancelable")]
    fn cancelable_shim(this: &RtcTrackEventInit, val: bool);
    #[wasm_bindgen(method, setter = "composed")]
    fn composed_shim(this: &RtcTrackEventInit, val: bool);
    #[cfg(feature = "RtcRtpReceiver")]
    #[wasm_bindgen(method, setter = "receiver")]
    fn receiver_shim(this: &RtcTrackEventInit, val: &RtcRtpReceiver);
    #[wasm_bindgen(method, setter = "streams")]
    fn streams_shim(this: &RtcTrackEventInit, val: &::wasm_bindgen::JsValue);
    #[cfg(feature = "MediaStreamTrack")]
    #[wasm_bindgen(method, setter = "track")]
    fn track_shim(this: &RtcTrackEventInit, val: &MediaStreamTrack);
    #[cfg(feature = "RtcRtpTransceiver")]
    #[wasm_bindgen(method, setter = "transceiver")]
    fn transceiver_shim(this: &RtcTrackEventInit, val: &RtcRtpTransceiver);
}
impl RtcTrackEventInit {
    #[cfg(all(
        feature = "MediaStreamTrack",
        feature = "RtcRtpReceiver",
        feature = "RtcRtpTransceiver",
    ))]
    #[doc = "Construct a new `RtcTrackEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamTrack`, `RtcRtpReceiver`, `RtcRtpTransceiver`, `RtcTrackEventInit`*"]
    pub fn new(
        receiver: &RtcRtpReceiver,
        track: &MediaStreamTrack,
        transceiver: &RtcRtpTransceiver,
    ) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.receiver(receiver);
        ret.track(track);
        ret.transceiver(transceiver);
        ret
    }
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcTrackEventInit`*"]
    pub fn bubbles(&mut self, val: bool) -> &mut Self {
        self.bubbles_shim(val);
        self
    }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcTrackEventInit`*"]
    pub fn cancelable(&mut self, val: bool) -> &mut Self {
        self.cancelable_shim(val);
        self
    }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcTrackEventInit`*"]
    pub fn composed(&mut self, val: bool) -> &mut Self {
        self.composed_shim(val);
        self
    }
    #[cfg(feature = "RtcRtpReceiver")]
    #[doc = "Change the `receiver` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpReceiver`, `RtcTrackEventInit`*"]
    pub fn receiver(&mut self, val: &RtcRtpReceiver) -> &mut Self {
        self.receiver_shim(val);
        self
    }
    #[doc = "Change the `streams` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcTrackEventInit`*"]
    pub fn streams(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.streams_shim(val);
        self
    }
    #[cfg(feature = "MediaStreamTrack")]
    #[doc = "Change the `track` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamTrack`, `RtcTrackEventInit`*"]
    pub fn track(&mut self, val: &MediaStreamTrack) -> &mut Self {
        self.track_shim(val);
        self
    }
    #[cfg(feature = "RtcRtpTransceiver")]
    #[doc = "Change the `transceiver` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpTransceiver`, `RtcTrackEventInit`*"]
    pub fn transceiver(&mut self, val: &RtcRtpTransceiver) -> &mut Self {
        self.transceiver_shim(val);
        self
    }
}
