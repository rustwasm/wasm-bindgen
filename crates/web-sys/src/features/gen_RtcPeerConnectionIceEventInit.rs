#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = RTCPeerConnectionIceEventInit)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcPeerConnectionIceEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcPeerConnectionIceEventInit`*"]
    pub type RtcPeerConnectionIceEventInit;
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcPeerConnectionIceEventInit`*"]
    #[wasm_bindgen(method, getter = "bubbles")]
    pub fn get_bubbles(this: &RtcPeerConnectionIceEventInit) -> Option<bool>;
    #[wasm_bindgen(method, setter = "bubbles")]
    fn set_bubbles(this: &RtcPeerConnectionIceEventInit, val: bool);
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcPeerConnectionIceEventInit`*"]
    #[wasm_bindgen(method, getter = "cancelable")]
    pub fn get_cancelable(this: &RtcPeerConnectionIceEventInit) -> Option<bool>;
    #[wasm_bindgen(method, setter = "cancelable")]
    fn set_cancelable(this: &RtcPeerConnectionIceEventInit, val: bool);
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcPeerConnectionIceEventInit`*"]
    #[wasm_bindgen(method, getter = "composed")]
    pub fn get_composed(this: &RtcPeerConnectionIceEventInit) -> Option<bool>;
    #[wasm_bindgen(method, setter = "composed")]
    fn set_composed(this: &RtcPeerConnectionIceEventInit, val: bool);
    #[cfg(feature = "RtcIceCandidate")]
    #[doc = "Get the `candidate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidate`, `RtcPeerConnectionIceEventInit`*"]
    #[wasm_bindgen(method, getter = "candidate")]
    pub fn get_candidate(this: &RtcPeerConnectionIceEventInit) -> Option<RtcIceCandidate>;
    #[cfg(feature = "RtcIceCandidate")]
    #[wasm_bindgen(method, setter = "candidate")]
    fn set_candidate(this: &RtcPeerConnectionIceEventInit, val: Option<&RtcIceCandidate>);
}
impl RtcPeerConnectionIceEventInit {
    #[doc = "Construct a new `RtcPeerConnectionIceEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcPeerConnectionIceEventInit`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcPeerConnectionIceEventInit`*"]
    pub fn bubbles(&mut self, val: bool) -> &mut Self {
        self.set_bubbles(val);
        self
    }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcPeerConnectionIceEventInit`*"]
    pub fn cancelable(&mut self, val: bool) -> &mut Self {
        self.set_cancelable(val);
        self
    }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcPeerConnectionIceEventInit`*"]
    pub fn composed(&mut self, val: bool) -> &mut Self {
        self.set_composed(val);
        self
    }
    #[cfg(feature = "RtcIceCandidate")]
    #[doc = "Change the `candidate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidate`, `RtcPeerConnectionIceEventInit`*"]
    pub fn candidate(&mut self, val: Option<&RtcIceCandidate>) -> &mut Self {
        self.set_candidate(val);
        self
    }
}
impl Default for RtcPeerConnectionIceEventInit {
    fn default() -> Self {
        Self::new()
    }
}
