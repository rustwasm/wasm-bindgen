#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = RTCDataChannelInit)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcDataChannelInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannelInit`*"]
    pub type RtcDataChannelInit;
    #[wasm_bindgen(method, setter = "id")]
    fn id_shim(this: &RtcDataChannelInit, val: u16);
    #[wasm_bindgen(method, setter = "maxPacketLifeTime")]
    fn max_packet_life_time_shim(this: &RtcDataChannelInit, val: u16);
    #[wasm_bindgen(method, setter = "maxRetransmitTime")]
    fn max_retransmit_time_shim(this: &RtcDataChannelInit, val: u16);
    #[wasm_bindgen(method, setter = "maxRetransmits")]
    fn max_retransmits_shim(this: &RtcDataChannelInit, val: u16);
    #[wasm_bindgen(method, setter = "negotiated")]
    fn negotiated_shim(this: &RtcDataChannelInit, val: bool);
    #[wasm_bindgen(method, setter = "ordered")]
    fn ordered_shim(this: &RtcDataChannelInit, val: bool);
    #[wasm_bindgen(method, setter = "protocol")]
    fn protocol_shim(this: &RtcDataChannelInit, val: &str);
}
impl RtcDataChannelInit {
    #[doc = "Construct a new `RtcDataChannelInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannelInit`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannelInit`*"]
    pub fn id(&mut self, val: u16) -> &mut Self {
        self.id_shim(val);
        self
    }
    #[doc = "Change the `maxPacketLifeTime` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannelInit`*"]
    pub fn max_packet_life_time(&mut self, val: u16) -> &mut Self {
        self.max_packet_life_time_shim(val);
        self
    }
    #[doc = "Change the `maxRetransmitTime` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannelInit`*"]
    pub fn max_retransmit_time(&mut self, val: u16) -> &mut Self {
        self.max_retransmit_time_shim(val);
        self
    }
    #[doc = "Change the `maxRetransmits` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannelInit`*"]
    pub fn max_retransmits(&mut self, val: u16) -> &mut Self {
        self.max_retransmits_shim(val);
        self
    }
    #[doc = "Change the `negotiated` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannelInit`*"]
    pub fn negotiated(&mut self, val: bool) -> &mut Self {
        self.negotiated_shim(val);
        self
    }
    #[doc = "Change the `ordered` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannelInit`*"]
    pub fn ordered(&mut self, val: bool) -> &mut Self {
        self.ordered_shim(val);
        self
    }
    #[doc = "Change the `protocol` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannelInit`*"]
    pub fn protocol(&mut self, val: &str) -> &mut Self {
        self.protocol_shim(val);
        self
    }
}
impl Default for RtcDataChannelInit {
    fn default() -> Self {
        Self::new()
    }
}
