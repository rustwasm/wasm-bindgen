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
    #[wasm_bindgen(method, getter = "bubbles")]
    fn bubbles_shim(this: &RtcDataChannelEventInit) -> bool;
    #[wasm_bindgen(method, setter = "bubbles")]
    fn set_bubbles_shim(this: &RtcDataChannelEventInit, val: bool);
    #[wasm_bindgen(method, getter = "cancelable")]
    fn cancelable_shim(this: &RtcDataChannelEventInit) -> bool;
    #[wasm_bindgen(method, setter = "cancelable")]
    fn set_cancelable_shim(this: &RtcDataChannelEventInit, val: bool);
    #[wasm_bindgen(method, getter = "composed")]
    fn composed_shim(this: &RtcDataChannelEventInit) -> bool;
    #[wasm_bindgen(method, setter = "composed")]
    fn set_composed_shim(this: &RtcDataChannelEventInit, val: bool);
    #[cfg(feature = "RtcDataChannel")]
    #[wasm_bindgen(method, getter = "channel")]
    fn channel_shim(this: &RtcDataChannelEventInit) -> RtcDataChannel;
    #[cfg(feature = "RtcDataChannel")]
    #[wasm_bindgen(method, setter = "channel")]
    fn set_channel_shim(this: &RtcDataChannelEventInit, val: &RtcDataChannel);
}
#[doc = "The trait to access properties on the `RtcDataChannelEventInit` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `RtcDataChannelEventInit`*"]
pub trait RtcDataChannelEventInitGetters {
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannelEventInit`*"]
    fn bubbles(&self) -> bool;
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannelEventInit`*"]
    fn cancelable(&self) -> bool;
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannelEventInit`*"]
    fn composed(&self) -> bool;
    #[cfg(feature = "RtcDataChannel")]
    #[doc = "Get the `channel` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannel`, `RtcDataChannelEventInit`*"]
    fn channel(&self) -> RtcDataChannel;
}
impl RtcDataChannelEventInitGetters for RtcDataChannelEventInit {
    fn bubbles(&self) -> bool {
        self.bubbles_shim()
    }
    fn cancelable(&self) -> bool {
        self.cancelable_shim()
    }
    fn composed(&self) -> bool {
        self.composed_shim()
    }
    #[cfg(feature = "RtcDataChannel")]
    fn channel(&self) -> RtcDataChannel {
        self.channel_shim()
    }
}
impl RtcDataChannelEventInit {
    #[cfg(feature = "RtcDataChannel")]
    #[doc = "Construct a new `RtcDataChannelEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannel`, `RtcDataChannelEventInit`*"]
    pub fn new(channel: &RtcDataChannel) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        Self::channel(&mut ret, channel);
        ret
    }
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannelEventInit`*"]
    pub fn bubbles(&mut self, val: bool) -> &mut Self {
        self.set_bubbles_shim(val);
        self
    }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannelEventInit`*"]
    pub fn cancelable(&mut self, val: bool) -> &mut Self {
        self.set_cancelable_shim(val);
        self
    }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannelEventInit`*"]
    pub fn composed(&mut self, val: bool) -> &mut Self {
        self.set_composed_shim(val);
        self
    }
    #[cfg(feature = "RtcDataChannel")]
    #[doc = "Change the `channel` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannel`, `RtcDataChannelEventInit`*"]
    pub fn channel(&mut self, val: &RtcDataChannel) -> &mut Self {
        self.set_channel_shim(val);
        self
    }
}
