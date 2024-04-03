#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = FontFaceSetLoadEventInit)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `FontFaceSetLoadEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFaceSetLoadEventInit`*"]
    pub type FontFaceSetLoadEventInit;
    #[wasm_bindgen(method, setter = "bubbles")]
    fn bubbles_shim(this: &FontFaceSetLoadEventInit, val: bool);
    #[wasm_bindgen(method, setter = "cancelable")]
    fn cancelable_shim(this: &FontFaceSetLoadEventInit, val: bool);
    #[wasm_bindgen(method, setter = "composed")]
    fn composed_shim(this: &FontFaceSetLoadEventInit, val: bool);
    #[wasm_bindgen(method, setter = "fontfaces")]
    fn fontfaces_shim(this: &FontFaceSetLoadEventInit, val: &::wasm_bindgen::JsValue);
}
impl FontFaceSetLoadEventInit {
    #[doc = "Construct a new `FontFaceSetLoadEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFaceSetLoadEventInit`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFaceSetLoadEventInit`*"]
    pub fn bubbles(&mut self, val: bool) -> &mut Self {
        self.bubbles_shim(val);
        self
    }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFaceSetLoadEventInit`*"]
    pub fn cancelable(&mut self, val: bool) -> &mut Self {
        self.cancelable_shim(val);
        self
    }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFaceSetLoadEventInit`*"]
    pub fn composed(&mut self, val: bool) -> &mut Self {
        self.composed_shim(val);
        self
    }
    #[doc = "Change the `fontfaces` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFaceSetLoadEventInit`*"]
    pub fn fontfaces(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.fontfaces_shim(val);
        self
    }
}
impl Default for FontFaceSetLoadEventInit {
    fn default() -> Self {
        Self::new()
    }
}
