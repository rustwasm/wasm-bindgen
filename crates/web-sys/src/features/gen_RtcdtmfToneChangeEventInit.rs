#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = RTCDTMFToneChangeEventInit)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcdtmfToneChangeEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcdtmfToneChangeEventInit`*"]
    pub type RtcdtmfToneChangeEventInit;
    #[wasm_bindgen(method, getter = "bubbles")]
    fn bubbles_shim(this: &RtcdtmfToneChangeEventInit) -> bool;
    #[wasm_bindgen(method, setter = "bubbles")]
    fn set_bubbles_shim(this: &RtcdtmfToneChangeEventInit, val: bool);
    #[wasm_bindgen(method, getter = "cancelable")]
    fn cancelable_shim(this: &RtcdtmfToneChangeEventInit) -> bool;
    #[wasm_bindgen(method, setter = "cancelable")]
    fn set_cancelable_shim(this: &RtcdtmfToneChangeEventInit, val: bool);
    #[wasm_bindgen(method, getter = "composed")]
    fn composed_shim(this: &RtcdtmfToneChangeEventInit) -> bool;
    #[wasm_bindgen(method, setter = "composed")]
    fn set_composed_shim(this: &RtcdtmfToneChangeEventInit, val: bool);
    #[wasm_bindgen(method, getter = "tone")]
    fn tone_shim(this: &RtcdtmfToneChangeEventInit) -> String;
    #[wasm_bindgen(method, setter = "tone")]
    fn set_tone_shim(this: &RtcdtmfToneChangeEventInit, val: &str);
}
#[doc = "The trait to access properties on the `RtcdtmfToneChangeEventInit` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `RtcdtmfToneChangeEventInit`*"]
pub trait RtcdtmfToneChangeEventInitGetters {
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcdtmfToneChangeEventInit`*"]
    fn bubbles(&self) -> bool;
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcdtmfToneChangeEventInit`*"]
    fn cancelable(&self) -> bool;
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcdtmfToneChangeEventInit`*"]
    fn composed(&self) -> bool;
    #[doc = "Get the `tone` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcdtmfToneChangeEventInit`*"]
    fn tone(&self) -> String;
}
impl RtcdtmfToneChangeEventInitGetters for RtcdtmfToneChangeEventInit {
    fn bubbles(&self) -> bool {
        self.bubbles_shim()
    }
    fn cancelable(&self) -> bool {
        self.cancelable_shim()
    }
    fn composed(&self) -> bool {
        self.composed_shim()
    }
    fn tone(&self) -> String {
        self.tone_shim()
    }
}
impl RtcdtmfToneChangeEventInit {
    #[doc = "Construct a new `RtcdtmfToneChangeEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcdtmfToneChangeEventInit`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcdtmfToneChangeEventInit`*"]
    pub fn bubbles(&mut self, val: bool) -> &mut Self {
        self.set_bubbles_shim(val);
        self
    }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcdtmfToneChangeEventInit`*"]
    pub fn cancelable(&mut self, val: bool) -> &mut Self {
        self.set_cancelable_shim(val);
        self
    }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcdtmfToneChangeEventInit`*"]
    pub fn composed(&mut self, val: bool) -> &mut Self {
        self.set_composed_shim(val);
        self
    }
    #[doc = "Change the `tone` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcdtmfToneChangeEventInit`*"]
    pub fn tone(&mut self, val: &str) -> &mut Self {
        self.set_tone_shim(val);
        self
    }
}
impl Default for RtcdtmfToneChangeEventInit {
    fn default() -> Self {
        Self::new()
    }
}
