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
    #[wasm_bindgen(method, getter = "id")]
    fn id_shim(this: &RtcDataChannelInit) -> u16;
    #[wasm_bindgen(method, setter = "id")]
    fn set_id_shim(this: &RtcDataChannelInit, val: u16);
    #[wasm_bindgen(method, getter = "maxPacketLifeTime")]
    fn max_packet_life_time_shim(this: &RtcDataChannelInit) -> u16;
    #[wasm_bindgen(method, setter = "maxPacketLifeTime")]
    fn set_max_packet_life_time_shim(this: &RtcDataChannelInit, val: u16);
    #[wasm_bindgen(method, getter = "maxRetransmitTime")]
    fn max_retransmit_time_shim(this: &RtcDataChannelInit) -> u16;
    #[wasm_bindgen(method, setter = "maxRetransmitTime")]
    fn set_max_retransmit_time_shim(this: &RtcDataChannelInit, val: u16);
    #[wasm_bindgen(method, getter = "maxRetransmits")]
    fn max_retransmits_shim(this: &RtcDataChannelInit) -> u16;
    #[wasm_bindgen(method, setter = "maxRetransmits")]
    fn set_max_retransmits_shim(this: &RtcDataChannelInit, val: u16);
    #[wasm_bindgen(method, getter = "negotiated")]
    fn negotiated_shim(this: &RtcDataChannelInit) -> bool;
    #[wasm_bindgen(method, setter = "negotiated")]
    fn set_negotiated_shim(this: &RtcDataChannelInit, val: bool);
    #[wasm_bindgen(method, getter = "ordered")]
    fn ordered_shim(this: &RtcDataChannelInit) -> bool;
    #[wasm_bindgen(method, setter = "ordered")]
    fn set_ordered_shim(this: &RtcDataChannelInit, val: bool);
    #[wasm_bindgen(method, getter = "protocol")]
    fn protocol_shim(this: &RtcDataChannelInit) -> &str;
    #[wasm_bindgen(method, setter = "protocol")]
    fn set_protocol_shim(this: &RtcDataChannelInit, val: &str);
}
#[doc = "The trait to access properties on the `RtcDataChannelInit` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `RtcDataChannelInit`*"]
pub trait RtcDataChannelInitGetters {
    #[doc = "Get the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannelInit`*"]
    fn id(&self) -> u16;
    #[doc = "Get the `maxPacketLifeTime` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannelInit`*"]
    fn max_packet_life_time(&self) -> u16;
    #[doc = "Get the `maxRetransmitTime` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannelInit`*"]
    fn max_retransmit_time(&self) -> u16;
    #[doc = "Get the `maxRetransmits` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannelInit`*"]
    fn max_retransmits(&self) -> u16;
    #[doc = "Get the `negotiated` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannelInit`*"]
    fn negotiated(&self) -> bool;
    #[doc = "Get the `ordered` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannelInit`*"]
    fn ordered(&self) -> bool;
    #[doc = "Get the `protocol` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannelInit`*"]
    fn protocol(&self) -> &str;
}
impl RtcDataChannelInitGetters for RtcDataChannelInit {
    fn id(&self) -> u16 {
        self.id_shim()
    }
    fn max_packet_life_time(&self) -> u16 {
        self.max_packet_life_time_shim()
    }
    fn max_retransmit_time(&self) -> u16 {
        self.max_retransmit_time_shim()
    }
    fn max_retransmits(&self) -> u16 {
        self.max_retransmits_shim()
    }
    fn negotiated(&self) -> bool {
        self.negotiated_shim()
    }
    fn ordered(&self) -> bool {
        self.ordered_shim()
    }
    fn protocol(&self) -> &str {
        self.protocol_shim()
    }
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
        self.set_id_shim(val);
        self
    }
    #[doc = "Change the `maxPacketLifeTime` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannelInit`*"]
    pub fn max_packet_life_time(&mut self, val: u16) -> &mut Self {
        self.set_max_packet_life_time_shim(val);
        self
    }
    #[doc = "Change the `maxRetransmitTime` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannelInit`*"]
    pub fn max_retransmit_time(&mut self, val: u16) -> &mut Self {
        self.set_max_retransmit_time_shim(val);
        self
    }
    #[doc = "Change the `maxRetransmits` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannelInit`*"]
    pub fn max_retransmits(&mut self, val: u16) -> &mut Self {
        self.set_max_retransmits_shim(val);
        self
    }
    #[doc = "Change the `negotiated` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannelInit`*"]
    pub fn negotiated(&mut self, val: bool) -> &mut Self {
        self.set_negotiated_shim(val);
        self
    }
    #[doc = "Change the `ordered` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannelInit`*"]
    pub fn ordered(&mut self, val: bool) -> &mut Self {
        self.set_ordered_shim(val);
        self
    }
    #[doc = "Change the `protocol` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannelInit`*"]
    pub fn protocol(&mut self, val: &str) -> &mut Self {
        self.set_protocol_shim(val);
        self
    }
}
impl Default for RtcDataChannelInit {
    fn default() -> Self {
        Self::new()
    }
}
