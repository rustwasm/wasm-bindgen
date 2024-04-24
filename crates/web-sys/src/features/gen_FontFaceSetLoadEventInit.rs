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
    #[wasm_bindgen(method, getter = "bubbles")]
    fn bubbles_shim(this: &FontFaceSetLoadEventInit) -> bool;
    #[wasm_bindgen(method, setter = "bubbles")]
    fn set_bubbles_shim(this: &FontFaceSetLoadEventInit, val: bool);
    #[wasm_bindgen(method, getter = "cancelable")]
    fn cancelable_shim(this: &FontFaceSetLoadEventInit) -> bool;
    #[wasm_bindgen(method, setter = "cancelable")]
    fn set_cancelable_shim(this: &FontFaceSetLoadEventInit, val: bool);
    #[wasm_bindgen(method, getter = "composed")]
    fn composed_shim(this: &FontFaceSetLoadEventInit) -> bool;
    #[wasm_bindgen(method, setter = "composed")]
    fn set_composed_shim(this: &FontFaceSetLoadEventInit, val: bool);
    #[wasm_bindgen(method, getter = "fontfaces")]
    fn fontfaces_shim(this: &FontFaceSetLoadEventInit) -> &::wasm_bindgen::JsValue;
    #[wasm_bindgen(method, setter = "fontfaces")]
    fn set_fontfaces_shim(this: &FontFaceSetLoadEventInit, val: &::wasm_bindgen::JsValue);
}
#[doc = "The trait to access properties on the `FontFaceSetLoadEventInit` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `FontFaceSetLoadEventInit`*"]
pub trait FontFaceSetLoadEventInitGetters {
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFaceSetLoadEventInit`*"]
    fn bubbles(&self) -> bool;
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFaceSetLoadEventInit`*"]
    fn cancelable(&self) -> bool;
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFaceSetLoadEventInit`*"]
    fn composed(&self) -> bool;
    #[doc = "Get the `fontfaces` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFaceSetLoadEventInit`*"]
    fn fontfaces(&self) -> &::wasm_bindgen::JsValue;
}
impl FontFaceSetLoadEventInitGetters for FontFaceSetLoadEventInit {
    fn bubbles(&self) -> bool {
        self.bubbles_shim()
    }
    fn cancelable(&self) -> bool {
        self.cancelable_shim()
    }
    fn composed(&self) -> bool {
        self.composed_shim()
    }
    fn fontfaces(&self) -> &::wasm_bindgen::JsValue {
        self.fontfaces_shim()
    }
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
        self.set_bubbles_shim(val);
        self
    }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFaceSetLoadEventInit`*"]
    pub fn cancelable(&mut self, val: bool) -> &mut Self {
        self.set_cancelable_shim(val);
        self
    }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFaceSetLoadEventInit`*"]
    pub fn composed(&mut self, val: bool) -> &mut Self {
        self.set_composed_shim(val);
        self
    }
    #[doc = "Change the `fontfaces` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFaceSetLoadEventInit`*"]
    pub fn fontfaces(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_fontfaces_shim(val);
        self
    }
}
impl Default for FontFaceSetLoadEventInit {
    fn default() -> Self {
        Self::new()
    }
}
