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
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcdtmfToneChangeEventInit`*"]
    #[wasm_bindgen(method, getter = "bubbles")]
    pub fn get_bubbles(this: &RtcdtmfToneChangeEventInit) -> Option<bool>;
    #[wasm_bindgen(method, setter = "bubbles")]
    fn set_bubbles(this: &RtcdtmfToneChangeEventInit, val: bool);
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcdtmfToneChangeEventInit`*"]
    #[wasm_bindgen(method, getter = "cancelable")]
    pub fn get_cancelable(this: &RtcdtmfToneChangeEventInit) -> Option<bool>;
    #[wasm_bindgen(method, setter = "cancelable")]
    fn set_cancelable(this: &RtcdtmfToneChangeEventInit, val: bool);
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcdtmfToneChangeEventInit`*"]
    #[wasm_bindgen(method, getter = "composed")]
    pub fn get_composed(this: &RtcdtmfToneChangeEventInit) -> Option<bool>;
    #[wasm_bindgen(method, setter = "composed")]
    fn set_composed(this: &RtcdtmfToneChangeEventInit, val: bool);
    #[doc = "Get the `tone` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcdtmfToneChangeEventInit`*"]
    #[wasm_bindgen(method, getter = "tone")]
    pub fn get_tone(this: &RtcdtmfToneChangeEventInit) -> Option<String>;
    #[wasm_bindgen(method, setter = "tone")]
    fn set_tone(this: &RtcdtmfToneChangeEventInit, val: &str);
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
        self.set_bubbles(val);
        self
    }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcdtmfToneChangeEventInit`*"]
    pub fn cancelable(&mut self, val: bool) -> &mut Self {
        self.set_cancelable(val);
        self
    }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcdtmfToneChangeEventInit`*"]
    pub fn composed(&mut self, val: bool) -> &mut Self {
        self.set_composed(val);
        self
    }
    #[doc = "Change the `tone` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcdtmfToneChangeEventInit`*"]
    pub fn tone(&mut self, val: &str) -> &mut Self {
        self.set_tone(val);
        self
    }
}
impl Default for RtcdtmfToneChangeEventInit {
    fn default() -> Self {
        Self::new()
    }
}
