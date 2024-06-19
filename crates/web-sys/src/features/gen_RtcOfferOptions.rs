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
    #[doc = "Get the `iceRestart` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOfferOptions`*"]
    #[wasm_bindgen(method, getter = "iceRestart")]
    pub fn get_ice_restart(this: &RtcOfferOptions) -> Option<bool>;
    #[wasm_bindgen(method, setter = "iceRestart")]
    fn set_ice_restart(this: &RtcOfferOptions, val: bool);
    #[doc = "Get the `offerToReceiveAudio` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOfferOptions`*"]
    #[wasm_bindgen(method, getter = "offerToReceiveAudio")]
    pub fn get_offer_to_receive_audio(this: &RtcOfferOptions) -> Option<bool>;
    #[wasm_bindgen(method, setter = "offerToReceiveAudio")]
    fn set_offer_to_receive_audio(this: &RtcOfferOptions, val: bool);
    #[doc = "Get the `offerToReceiveVideo` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOfferOptions`*"]
    #[wasm_bindgen(method, getter = "offerToReceiveVideo")]
    pub fn get_offer_to_receive_video(this: &RtcOfferOptions) -> Option<bool>;
    #[wasm_bindgen(method, setter = "offerToReceiveVideo")]
    fn set_offer_to_receive_video(this: &RtcOfferOptions, val: bool);
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
        self.set_ice_restart(val);
        self
    }
    #[doc = "Change the `offerToReceiveAudio` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOfferOptions`*"]
    pub fn offer_to_receive_audio(&mut self, val: bool) -> &mut Self {
        self.set_offer_to_receive_audio(val);
        self
    }
    #[doc = "Change the `offerToReceiveVideo` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOfferOptions`*"]
    pub fn offer_to_receive_video(&mut self, val: bool) -> &mut Self {
        self.set_offer_to_receive_video(val);
        self
    }
}
impl Default for RtcOfferOptions {
    fn default() -> Self {
        Self::new()
    }
}
