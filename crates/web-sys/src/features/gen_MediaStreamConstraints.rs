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
    #[wasm_bindgen(method, getter = "audio")]
    fn audio_shim(this: &MediaStreamConstraints) -> &::wasm_bindgen::JsValue;
    #[wasm_bindgen(method, setter = "audio")]
    fn set_audio_shim(this: &MediaStreamConstraints, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, getter = "fake")]
    fn fake_shim(this: &MediaStreamConstraints) -> bool;
    #[wasm_bindgen(method, setter = "fake")]
    fn set_fake_shim(this: &MediaStreamConstraints, val: bool);
    #[wasm_bindgen(method, getter = "peerIdentity")]
    fn peer_identity_shim(this: &MediaStreamConstraints) -> Option<&str>;
    #[wasm_bindgen(method, setter = "peerIdentity")]
    fn set_peer_identity_shim(this: &MediaStreamConstraints, val: Option<&str>);
    #[wasm_bindgen(method, getter = "picture")]
    fn picture_shim(this: &MediaStreamConstraints) -> bool;
    #[wasm_bindgen(method, setter = "picture")]
    fn set_picture_shim(this: &MediaStreamConstraints, val: bool);
    #[wasm_bindgen(method, getter = "video")]
    fn video_shim(this: &MediaStreamConstraints) -> &::wasm_bindgen::JsValue;
    #[wasm_bindgen(method, setter = "video")]
    fn set_video_shim(this: &MediaStreamConstraints, val: &::wasm_bindgen::JsValue);
}
#[doc = "The trait to access properties on the `MediaStreamConstraints` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `MediaStreamConstraints`*"]
pub trait MediaStreamConstraintsGetters {
    #[doc = "Get the `audio` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamConstraints`*"]
    fn audio(&self) -> &::wasm_bindgen::JsValue;
    #[doc = "Get the `fake` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamConstraints`*"]
    fn fake(&self) -> bool;
    #[doc = "Get the `peerIdentity` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamConstraints`*"]
    fn peer_identity(&self) -> Option<&str>;
    #[doc = "Get the `picture` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamConstraints`*"]
    fn picture(&self) -> bool;
    #[doc = "Get the `video` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamConstraints`*"]
    fn video(&self) -> &::wasm_bindgen::JsValue;
}
impl MediaStreamConstraintsGetters for MediaStreamConstraints {
    fn audio(&self) -> &::wasm_bindgen::JsValue {
        self.audio_shim()
    }
    fn fake(&self) -> bool {
        self.fake_shim()
    }
    fn peer_identity(&self) -> Option<&str> {
        self.peer_identity_shim()
    }
    fn picture(&self) -> bool {
        self.picture_shim()
    }
    fn video(&self) -> &::wasm_bindgen::JsValue {
        self.video_shim()
    }
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
        self.set_audio_shim(val);
        self
    }
    #[doc = "Change the `fake` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamConstraints`*"]
    pub fn fake(&mut self, val: bool) -> &mut Self {
        self.set_fake_shim(val);
        self
    }
    #[doc = "Change the `peerIdentity` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamConstraints`*"]
    pub fn peer_identity(&mut self, val: Option<&str>) -> &mut Self {
        self.set_peer_identity_shim(val);
        self
    }
    #[doc = "Change the `picture` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamConstraints`*"]
    pub fn picture(&mut self, val: bool) -> &mut Self {
        self.set_picture_shim(val);
        self
    }
    #[doc = "Change the `video` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamConstraints`*"]
    pub fn video(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_video_shim(val);
        self
    }
}
impl Default for MediaStreamConstraints {
    fn default() -> Self {
        Self::new()
    }
}
