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
    #[wasm_bindgen(method, getter = "bubbles")]
    fn bubbles_shim(this: &RtcPeerConnectionIceEventInit) -> bool;
    #[wasm_bindgen(method, setter = "bubbles")]
    fn set_bubbles_shim(this: &RtcPeerConnectionIceEventInit, val: bool);
    #[wasm_bindgen(method, getter = "cancelable")]
    fn cancelable_shim(this: &RtcPeerConnectionIceEventInit) -> bool;
    #[wasm_bindgen(method, setter = "cancelable")]
    fn set_cancelable_shim(this: &RtcPeerConnectionIceEventInit, val: bool);
    #[wasm_bindgen(method, getter = "composed")]
    fn composed_shim(this: &RtcPeerConnectionIceEventInit) -> bool;
    #[wasm_bindgen(method, setter = "composed")]
    fn set_composed_shim(this: &RtcPeerConnectionIceEventInit, val: bool);
    #[cfg(feature = "RtcIceCandidate")]
    #[wasm_bindgen(method, getter = "candidate")]
    fn candidate_shim(this: &RtcPeerConnectionIceEventInit) -> Option<&RtcIceCandidate>;
    #[cfg(feature = "RtcIceCandidate")]
    #[wasm_bindgen(method, setter = "candidate")]
    fn set_candidate_shim(this: &RtcPeerConnectionIceEventInit, val: Option<&RtcIceCandidate>);
}
#[doc = "The trait to access properties on the `RtcPeerConnectionIceEventInit` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `RtcPeerConnectionIceEventInit`*"]
pub trait RtcPeerConnectionIceEventInitGetters {
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcPeerConnectionIceEventInit`*"]
    fn bubbles(&self) -> bool;
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcPeerConnectionIceEventInit`*"]
    fn cancelable(&self) -> bool;
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcPeerConnectionIceEventInit`*"]
    fn composed(&self) -> bool;
    #[cfg(feature = "RtcIceCandidate")]
    #[doc = "Get the `candidate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidate`, `RtcPeerConnectionIceEventInit`*"]
    fn candidate(&self) -> Option<&RtcIceCandidate>;
}
impl RtcPeerConnectionIceEventInitGetters for RtcPeerConnectionIceEventInit {
    fn bubbles(&self) -> bool {
        self.bubbles_shim()
    }
    fn cancelable(&self) -> bool {
        self.cancelable_shim()
    }
    fn composed(&self) -> bool {
        self.composed_shim()
    }
    #[cfg(feature = "RtcIceCandidate")]
    fn candidate(&self) -> Option<&RtcIceCandidate> {
        self.candidate_shim()
    }
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
        self.set_bubbles_shim(val);
        self
    }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcPeerConnectionIceEventInit`*"]
    pub fn cancelable(&mut self, val: bool) -> &mut Self {
        self.set_cancelable_shim(val);
        self
    }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcPeerConnectionIceEventInit`*"]
    pub fn composed(&mut self, val: bool) -> &mut Self {
        self.set_composed_shim(val);
        self
    }
    #[cfg(feature = "RtcIceCandidate")]
    #[doc = "Change the `candidate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidate`, `RtcPeerConnectionIceEventInit`*"]
    pub fn candidate(&mut self, val: Option<&RtcIceCandidate>) -> &mut Self {
        self.set_candidate_shim(val);
        self
    }
}
impl Default for RtcPeerConnectionIceEventInit {
    fn default() -> Self {
        Self::new()
    }
}
