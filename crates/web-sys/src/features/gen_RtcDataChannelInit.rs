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
    #[doc = "Get the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannelInit`*"]
    #[wasm_bindgen(method, getter = "id")]
    pub fn get_id(this: &RtcDataChannelInit) -> Option<u16>;
    #[wasm_bindgen(method, setter = "id")]
    fn set_id(this: &RtcDataChannelInit, val: u16);
    #[doc = "Get the `maxPacketLifeTime` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannelInit`*"]
    #[wasm_bindgen(method, getter = "maxPacketLifeTime")]
    pub fn get_max_packet_life_time(this: &RtcDataChannelInit) -> Option<u16>;
    #[wasm_bindgen(method, setter = "maxPacketLifeTime")]
    fn set_max_packet_life_time(this: &RtcDataChannelInit, val: u16);
    #[doc = "Get the `maxRetransmitTime` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannelInit`*"]
    #[wasm_bindgen(method, getter = "maxRetransmitTime")]
    pub fn get_max_retransmit_time(this: &RtcDataChannelInit) -> Option<u16>;
    #[wasm_bindgen(method, setter = "maxRetransmitTime")]
    fn set_max_retransmit_time(this: &RtcDataChannelInit, val: u16);
    #[doc = "Get the `maxRetransmits` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannelInit`*"]
    #[wasm_bindgen(method, getter = "maxRetransmits")]
    pub fn get_max_retransmits(this: &RtcDataChannelInit) -> Option<u16>;
    #[wasm_bindgen(method, setter = "maxRetransmits")]
    fn set_max_retransmits(this: &RtcDataChannelInit, val: u16);
    #[doc = "Get the `negotiated` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannelInit`*"]
    #[wasm_bindgen(method, getter = "negotiated")]
    pub fn get_negotiated(this: &RtcDataChannelInit) -> Option<bool>;
    #[wasm_bindgen(method, setter = "negotiated")]
    fn set_negotiated(this: &RtcDataChannelInit, val: bool);
    #[doc = "Get the `ordered` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannelInit`*"]
    #[wasm_bindgen(method, getter = "ordered")]
    pub fn get_ordered(this: &RtcDataChannelInit) -> Option<bool>;
    #[wasm_bindgen(method, setter = "ordered")]
    fn set_ordered(this: &RtcDataChannelInit, val: bool);
    #[doc = "Get the `protocol` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannelInit`*"]
    #[wasm_bindgen(method, getter = "protocol")]
    pub fn get_protocol(this: &RtcDataChannelInit) -> Option<String>;
    #[wasm_bindgen(method, setter = "protocol")]
    fn set_protocol(this: &RtcDataChannelInit, val: &str);
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
        self.set_id(val);
        self
    }
    #[doc = "Change the `maxPacketLifeTime` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannelInit`*"]
    pub fn max_packet_life_time(&mut self, val: u16) -> &mut Self {
        self.set_max_packet_life_time(val);
        self
    }
    #[doc = "Change the `maxRetransmitTime` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannelInit`*"]
    pub fn max_retransmit_time(&mut self, val: u16) -> &mut Self {
        self.set_max_retransmit_time(val);
        self
    }
    #[doc = "Change the `maxRetransmits` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannelInit`*"]
    pub fn max_retransmits(&mut self, val: u16) -> &mut Self {
        self.set_max_retransmits(val);
        self
    }
    #[doc = "Change the `negotiated` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannelInit`*"]
    pub fn negotiated(&mut self, val: bool) -> &mut Self {
        self.set_negotiated(val);
        self
    }
    #[doc = "Change the `ordered` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannelInit`*"]
    pub fn ordered(&mut self, val: bool) -> &mut Self {
        self.set_ordered(val);
        self
    }
    #[doc = "Change the `protocol` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannelInit`*"]
    pub fn protocol(&mut self, val: &str) -> &mut Self {
        self.set_protocol(val);
        self
    }
}
impl Default for RtcDataChannelInit {
    fn default() -> Self {
        Self::new()
    }
}
