#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = RTCDataChannelEventInit)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcDataChannelEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannelEventInit`*"]
    pub type RtcDataChannelEventInit;
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannelEventInit`*"]
    #[wasm_bindgen(method, getter = "bubbles")]
    pub fn get_bubbles(this: &RtcDataChannelEventInit) -> Option<bool>;
    #[wasm_bindgen(method, setter = "bubbles")]
    fn set_bubbles(this: &RtcDataChannelEventInit, val: bool);
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannelEventInit`*"]
    #[wasm_bindgen(method, getter = "cancelable")]
    pub fn get_cancelable(this: &RtcDataChannelEventInit) -> Option<bool>;
    #[wasm_bindgen(method, setter = "cancelable")]
    fn set_cancelable(this: &RtcDataChannelEventInit, val: bool);
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannelEventInit`*"]
    #[wasm_bindgen(method, getter = "composed")]
    pub fn get_composed(this: &RtcDataChannelEventInit) -> Option<bool>;
    #[wasm_bindgen(method, setter = "composed")]
    fn set_composed(this: &RtcDataChannelEventInit, val: bool);
    #[cfg(feature = "RtcDataChannel")]
    #[doc = "Get the `channel` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannel`, `RtcDataChannelEventInit`*"]
    #[wasm_bindgen(method, getter = "channel")]
    pub fn get_channel(this: &RtcDataChannelEventInit) -> RtcDataChannel;
    #[cfg(feature = "RtcDataChannel")]
    #[wasm_bindgen(method, setter = "channel")]
    fn set_channel(this: &RtcDataChannelEventInit, val: &RtcDataChannel);
}
impl RtcDataChannelEventInit {
    #[cfg(feature = "RtcDataChannel")]
    #[doc = "Construct a new `RtcDataChannelEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannel`, `RtcDataChannelEventInit`*"]
    pub fn new(channel: &RtcDataChannel) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.channel(channel);
        ret
    }
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannelEventInit`*"]
    pub fn bubbles(&mut self, val: bool) -> &mut Self {
        self.set_bubbles(val);
        self
    }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannelEventInit`*"]
    pub fn cancelable(&mut self, val: bool) -> &mut Self {
        self.set_cancelable(val);
        self
    }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannelEventInit`*"]
    pub fn composed(&mut self, val: bool) -> &mut Self {
        self.set_composed(val);
        self
    }
    #[cfg(feature = "RtcDataChannel")]
    #[doc = "Change the `channel` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannel`, `RtcDataChannelEventInit`*"]
    pub fn channel(&mut self, val: &RtcDataChannel) -> &mut Self {
        self.set_channel(val);
        self
    }
}
