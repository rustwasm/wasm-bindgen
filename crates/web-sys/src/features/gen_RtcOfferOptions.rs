#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = RTCOfferOptions)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcOfferOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOfferOptions`*"]
    pub type RtcOfferOptions;
    #[wasm_bindgen(method, getter = "iceRestart")]
    fn ice_restart_shim(this: &RtcOfferOptions) -> bool;
    #[wasm_bindgen(method, setter = "iceRestart")]
    fn set_ice_restart_shim(this: &RtcOfferOptions, val: bool);
    #[wasm_bindgen(method, getter = "offerToReceiveAudio")]
    fn offer_to_receive_audio_shim(this: &RtcOfferOptions) -> bool;
    #[wasm_bindgen(method, setter = "offerToReceiveAudio")]
    fn set_offer_to_receive_audio_shim(this: &RtcOfferOptions, val: bool);
    #[wasm_bindgen(method, getter = "offerToReceiveVideo")]
    fn offer_to_receive_video_shim(this: &RtcOfferOptions) -> bool;
    #[wasm_bindgen(method, setter = "offerToReceiveVideo")]
    fn set_offer_to_receive_video_shim(this: &RtcOfferOptions, val: bool);
}
#[doc = "The trait to access properties on the `RtcOfferOptions` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `RtcOfferOptions`*"]
pub trait RtcOfferOptionsGetters {
    #[doc = "Get the `iceRestart` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOfferOptions`*"]
    fn ice_restart(&self) -> bool;
    #[doc = "Get the `offerToReceiveAudio` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOfferOptions`*"]
    fn offer_to_receive_audio(&self) -> bool;
    #[doc = "Get the `offerToReceiveVideo` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOfferOptions`*"]
    fn offer_to_receive_video(&self) -> bool;
}
impl RtcOfferOptionsGetters for RtcOfferOptions {
    fn ice_restart(&self) -> bool {
        self.ice_restart_shim()
    }
    fn offer_to_receive_audio(&self) -> bool {
        self.offer_to_receive_audio_shim()
    }
    fn offer_to_receive_video(&self) -> bool {
        self.offer_to_receive_video_shim()
    }
}
impl RtcOfferOptions {
    #[doc = "Construct a new `RtcOfferOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOfferOptions`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `iceRestart` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOfferOptions`*"]
    pub fn ice_restart(&mut self, val: bool) -> &mut Self {
        self.set_ice_restart_shim(val);
        self
    }
    #[doc = "Change the `offerToReceiveAudio` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOfferOptions`*"]
    pub fn offer_to_receive_audio(&mut self, val: bool) -> &mut Self {
        self.set_offer_to_receive_audio_shim(val);
        self
    }
    #[doc = "Change the `offerToReceiveVideo` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOfferOptions`*"]
    pub fn offer_to_receive_video(&mut self, val: bool) -> &mut Self {
        self.set_offer_to_receive_video_shim(val);
        self
    }
}
impl Default for RtcOfferOptions {
    fn default() -> Self {
        Self::new()
    }
}
