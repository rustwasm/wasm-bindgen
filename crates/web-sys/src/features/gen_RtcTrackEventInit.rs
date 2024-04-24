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
    #[wasm_bindgen(method, getter = "bubbles")]
    fn bubbles_shim(this: &RtcTrackEventInit) -> bool;
    #[wasm_bindgen(method, setter = "bubbles")]
    fn set_bubbles_shim(this: &RtcTrackEventInit, val: bool);
    #[wasm_bindgen(method, getter = "cancelable")]
    fn cancelable_shim(this: &RtcTrackEventInit) -> bool;
    #[wasm_bindgen(method, setter = "cancelable")]
    fn set_cancelable_shim(this: &RtcTrackEventInit, val: bool);
    #[wasm_bindgen(method, getter = "composed")]
    fn composed_shim(this: &RtcTrackEventInit) -> bool;
    #[wasm_bindgen(method, setter = "composed")]
    fn set_composed_shim(this: &RtcTrackEventInit, val: bool);
    #[cfg(feature = "RtcRtpReceiver")]
    #[wasm_bindgen(method, getter = "receiver")]
    fn receiver_shim(this: &RtcTrackEventInit) -> RtcRtpReceiver;
    #[cfg(feature = "RtcRtpReceiver")]
    #[wasm_bindgen(method, setter = "receiver")]
    fn set_receiver_shim(this: &RtcTrackEventInit, val: &RtcRtpReceiver);
    #[wasm_bindgen(method, getter = "streams")]
    fn streams_shim(this: &RtcTrackEventInit) -> ::js_sys::Array;
    #[wasm_bindgen(method, setter = "streams")]
    fn set_streams_shim(this: &RtcTrackEventInit, val: &::wasm_bindgen::JsValue);
    #[cfg(feature = "MediaStreamTrack")]
    #[wasm_bindgen(method, getter = "track")]
    fn track_shim(this: &RtcTrackEventInit) -> MediaStreamTrack;
    #[cfg(feature = "MediaStreamTrack")]
    #[wasm_bindgen(method, setter = "track")]
    fn set_track_shim(this: &RtcTrackEventInit, val: &MediaStreamTrack);
    #[cfg(feature = "RtcRtpTransceiver")]
    #[wasm_bindgen(method, getter = "transceiver")]
    fn transceiver_shim(this: &RtcTrackEventInit) -> RtcRtpTransceiver;
    #[cfg(feature = "RtcRtpTransceiver")]
    #[wasm_bindgen(method, setter = "transceiver")]
    fn set_transceiver_shim(this: &RtcTrackEventInit, val: &RtcRtpTransceiver);
}
#[doc = "The trait to access properties on the `RtcTrackEventInit` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `RtcTrackEventInit`*"]
pub trait RtcTrackEventInitGetters {
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcTrackEventInit`*"]
    fn bubbles(&self) -> bool;
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcTrackEventInit`*"]
    fn cancelable(&self) -> bool;
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcTrackEventInit`*"]
    fn composed(&self) -> bool;
    #[cfg(feature = "RtcRtpReceiver")]
    #[doc = "Get the `receiver` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpReceiver`, `RtcTrackEventInit`*"]
    fn receiver(&self) -> RtcRtpReceiver;
    #[doc = "Get the `streams` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcTrackEventInit`*"]
    fn streams(&self) -> ::js_sys::Array;
    #[cfg(feature = "MediaStreamTrack")]
    #[doc = "Get the `track` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamTrack`, `RtcTrackEventInit`*"]
    fn track(&self) -> MediaStreamTrack;
    #[cfg(feature = "RtcRtpTransceiver")]
    #[doc = "Get the `transceiver` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpTransceiver`, `RtcTrackEventInit`*"]
    fn transceiver(&self) -> RtcRtpTransceiver;
}
impl RtcTrackEventInitGetters for RtcTrackEventInit {
    fn bubbles(&self) -> bool {
        self.bubbles_shim()
    }
    fn cancelable(&self) -> bool {
        self.cancelable_shim()
    }
    fn composed(&self) -> bool {
        self.composed_shim()
    }
    #[cfg(feature = "RtcRtpReceiver")]
    fn receiver(&self) -> RtcRtpReceiver {
        self.receiver_shim()
    }
    fn streams(&self) -> ::js_sys::Array {
        self.streams_shim()
    }
    #[cfg(feature = "MediaStreamTrack")]
    fn track(&self) -> MediaStreamTrack {
        self.track_shim()
    }
    #[cfg(feature = "RtcRtpTransceiver")]
    fn transceiver(&self) -> RtcRtpTransceiver {
        self.transceiver_shim()
    }
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
        self.set_bubbles_shim(val);
        self
    }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcTrackEventInit`*"]
    pub fn cancelable(&mut self, val: bool) -> &mut Self {
        self.set_cancelable_shim(val);
        self
    }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcTrackEventInit`*"]
    pub fn composed(&mut self, val: bool) -> &mut Self {
        self.set_composed_shim(val);
        self
    }
    #[cfg(feature = "RtcRtpReceiver")]
    #[doc = "Change the `receiver` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpReceiver`, `RtcTrackEventInit`*"]
    pub fn receiver(&mut self, val: &RtcRtpReceiver) -> &mut Self {
        self.set_receiver_shim(val);
        self
    }
    #[doc = "Change the `streams` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcTrackEventInit`*"]
    pub fn streams(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_streams_shim(val);
        self
    }
    #[cfg(feature = "MediaStreamTrack")]
    #[doc = "Change the `track` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamTrack`, `RtcTrackEventInit`*"]
    pub fn track(&mut self, val: &MediaStreamTrack) -> &mut Self {
        self.set_track_shim(val);
        self
    }
    #[cfg(feature = "RtcRtpTransceiver")]
    #[doc = "Change the `transceiver` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpTransceiver`, `RtcTrackEventInit`*"]
    pub fn transceiver(&mut self, val: &RtcRtpTransceiver) -> &mut Self {
        self.set_transceiver_shim(val);
        self
    }
}
