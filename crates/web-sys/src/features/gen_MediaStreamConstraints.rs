#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = MediaStreamConstraints)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MediaStreamConstraints` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamConstraints`*"]
    pub type MediaStreamConstraints;
    #[wasm_bindgen(method, setter = "audio")]
    fn audio_shim(this: &MediaStreamConstraints, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, setter = "fake")]
    fn fake_shim(this: &MediaStreamConstraints, val: bool);
    #[wasm_bindgen(method, setter = "peerIdentity")]
    fn peer_identity_shim(this: &MediaStreamConstraints, val: Option<&str>);
    #[wasm_bindgen(method, setter = "picture")]
    fn picture_shim(this: &MediaStreamConstraints, val: bool);
    #[wasm_bindgen(method, setter = "video")]
    fn video_shim(this: &MediaStreamConstraints, val: &::wasm_bindgen::JsValue);
}
impl MediaStreamConstraints {
    #[doc = "Construct a new `MediaStreamConstraints`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamConstraints`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `audio` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamConstraints`*"]
    pub fn audio(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.audio_shim(val);
        self
    }
    #[doc = "Change the `fake` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamConstraints`*"]
    pub fn fake(&mut self, val: bool) -> &mut Self {
        self.fake_shim(val);
        self
    }
    #[doc = "Change the `peerIdentity` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamConstraints`*"]
    pub fn peer_identity(&mut self, val: Option<&str>) -> &mut Self {
        self.peer_identity_shim(val);
        self
    }
    #[doc = "Change the `picture` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamConstraints`*"]
    pub fn picture(&mut self, val: bool) -> &mut Self {
        self.picture_shim(val);
        self
    }
    #[doc = "Change the `video` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamConstraints`*"]
    pub fn video(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.video_shim(val);
        self
    }
}
impl Default for MediaStreamConstraints {
    fn default() -> Self {
        Self::new()
    }
}
