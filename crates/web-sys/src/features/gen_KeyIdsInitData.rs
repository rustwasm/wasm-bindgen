#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = KeyIdsInitData)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `KeyIdsInitData` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyIdsInitData`*"]
    pub type KeyIdsInitData;
    #[wasm_bindgen(method, getter = "kids")]
    fn kids_shim(this: &KeyIdsInitData) -> ::js_sys::Array;
    #[wasm_bindgen(method, setter = "kids")]
    fn set_kids_shim(this: &KeyIdsInitData, val: &::wasm_bindgen::JsValue);
}
#[doc = "The trait to access properties on the `KeyIdsInitData` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `KeyIdsInitData`*"]
pub trait KeyIdsInitDataGetters {
    #[doc = "Get the `kids` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyIdsInitData`*"]
    fn kids(&self) -> ::js_sys::Array;
}
impl KeyIdsInitDataGetters for KeyIdsInitData {
    fn kids(&self) -> ::js_sys::Array {
        self.kids_shim()
    }
}
impl KeyIdsInitData {
    #[doc = "Construct a new `KeyIdsInitData`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyIdsInitData`*"]
    pub fn new(kids: &::wasm_bindgen::JsValue) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        Self::kids(&mut ret, kids);
        ret
    }
    #[doc = "Change the `kids` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyIdsInitData`*"]
    pub fn kids(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_kids_shim(val);
        self
    }
}
