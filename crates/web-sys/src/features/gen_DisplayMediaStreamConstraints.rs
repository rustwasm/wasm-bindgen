#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = DisplayMediaStreamConstraints)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DisplayMediaStreamConstraints` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DisplayMediaStreamConstraints`*"]
    pub type DisplayMediaStreamConstraints;
    #[wasm_bindgen(method, getter = "audio")]
    fn audio_shim(this: &DisplayMediaStreamConstraints) -> ::wasm_bindgen::JsValue;
    #[wasm_bindgen(method, setter = "audio")]
    fn set_audio_shim(this: &DisplayMediaStreamConstraints, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, getter = "video")]
    fn video_shim(this: &DisplayMediaStreamConstraints) -> ::wasm_bindgen::JsValue;
    #[wasm_bindgen(method, setter = "video")]
    fn set_video_shim(this: &DisplayMediaStreamConstraints, val: &::wasm_bindgen::JsValue);
}
#[doc = "The trait to access properties on the `DisplayMediaStreamConstraints` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `DisplayMediaStreamConstraints`*"]
pub trait DisplayMediaStreamConstraintsGetters {
    #[doc = "Get the `audio` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DisplayMediaStreamConstraints`*"]
    fn audio(&self) -> ::wasm_bindgen::JsValue;
    #[doc = "Get the `video` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DisplayMediaStreamConstraints`*"]
    fn video(&self) -> ::wasm_bindgen::JsValue;
}
impl DisplayMediaStreamConstraintsGetters for DisplayMediaStreamConstraints {
    fn audio(&self) -> ::wasm_bindgen::JsValue {
        self.audio_shim()
    }
    fn video(&self) -> ::wasm_bindgen::JsValue {
        self.video_shim()
    }
}
impl DisplayMediaStreamConstraints {
    #[doc = "Construct a new `DisplayMediaStreamConstraints`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DisplayMediaStreamConstraints`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `audio` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DisplayMediaStreamConstraints`*"]
    pub fn audio(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_audio_shim(val);
        self
    }
    #[doc = "Change the `video` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DisplayMediaStreamConstraints`*"]
    pub fn video(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_video_shim(val);
        self
    }
}
impl Default for DisplayMediaStreamConstraints {
    fn default() -> Self {
        Self::new()
    }
}
