#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = U2FClientData)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `U2fClientData` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `U2fClientData`*"]
    pub type U2fClientData;
    #[wasm_bindgen(method, getter = "challenge")]
    fn challenge_shim(this: &U2fClientData) -> &str;
    #[wasm_bindgen(method, setter = "challenge")]
    fn set_challenge_shim(this: &U2fClientData, val: &str);
    #[wasm_bindgen(method, getter = "origin")]
    fn origin_shim(this: &U2fClientData) -> &str;
    #[wasm_bindgen(method, setter = "origin")]
    fn set_origin_shim(this: &U2fClientData, val: &str);
    #[wasm_bindgen(method, getter = "typ")]
    fn typ_shim(this: &U2fClientData) -> &str;
    #[wasm_bindgen(method, setter = "typ")]
    fn set_typ_shim(this: &U2fClientData, val: &str);
}
#[doc = "The trait to access properties on the `U2fClientData` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `U2fClientData`*"]
pub trait U2fClientDataGetters {
    #[doc = "Get the `challenge` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `U2fClientData`*"]
    fn challenge(&self) -> &str;
    #[doc = "Get the `origin` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `U2fClientData`*"]
    fn origin(&self) -> &str;
    #[doc = "Get the `typ` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `U2fClientData`*"]
    fn typ(&self) -> &str;
}
impl U2fClientDataGetters for U2fClientData {
    fn challenge(&self) -> &str {
        self.challenge_shim()
    }
    fn origin(&self) -> &str {
        self.origin_shim()
    }
    fn typ(&self) -> &str {
        self.typ_shim()
    }
}
impl U2fClientData {
    #[doc = "Construct a new `U2fClientData`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `U2fClientData`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `challenge` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `U2fClientData`*"]
    pub fn challenge(&mut self, val: &str) -> &mut Self {
        self.set_challenge_shim(val);
        self
    }
    #[doc = "Change the `origin` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `U2fClientData`*"]
    pub fn origin(&mut self, val: &str) -> &mut Self {
        self.set_origin_shim(val);
        self
    }
    #[doc = "Change the `typ` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `U2fClientData`*"]
    pub fn typ(&mut self, val: &str) -> &mut Self {
        self.set_typ_shim(val);
        self
    }
}
impl Default for U2fClientData {
    fn default() -> Self {
        Self::new()
    }
}
