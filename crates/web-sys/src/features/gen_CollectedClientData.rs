#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = CollectedClientData)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `CollectedClientData` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CollectedClientData`*"]
    pub type CollectedClientData;
    #[wasm_bindgen(method, setter = "challenge")]
    fn challenge_shim(this: &CollectedClientData, val: &str);
    #[wasm_bindgen(method, setter = "crossOrigin")]
    fn cross_origin_shim(this: &CollectedClientData, val: bool);
    #[wasm_bindgen(method, setter = "origin")]
    fn origin_shim(this: &CollectedClientData, val: &str);
    #[wasm_bindgen(method, setter = "topOrigin")]
    fn top_origin_shim(this: &CollectedClientData, val: &str);
    #[wasm_bindgen(method, setter = "type")]
    fn type__shim(this: &CollectedClientData, val: &str);
}
impl CollectedClientData {
    #[doc = "Construct a new `CollectedClientData`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CollectedClientData`*"]
    pub fn new(challenge: &str, origin: &str, type_: &str) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.challenge(challenge);
        ret.origin(origin);
        ret.type_(type_);
        ret
    }
    #[doc = "Change the `challenge` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CollectedClientData`*"]
    pub fn challenge(&mut self, val: &str) -> &mut Self {
        self.challenge_shim(val);
        self
    }
    #[doc = "Change the `crossOrigin` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CollectedClientData`*"]
    pub fn cross_origin(&mut self, val: bool) -> &mut Self {
        self.cross_origin_shim(val);
        self
    }
    #[doc = "Change the `origin` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CollectedClientData`*"]
    pub fn origin(&mut self, val: &str) -> &mut Self {
        self.origin_shim(val);
        self
    }
    #[doc = "Change the `topOrigin` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CollectedClientData`*"]
    pub fn top_origin(&mut self, val: &str) -> &mut Self {
        self.top_origin_shim(val);
        self
    }
    #[doc = "Change the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CollectedClientData`*"]
    pub fn type_(&mut self, val: &str) -> &mut Self {
        self.type__shim(val);
        self
    }
}
