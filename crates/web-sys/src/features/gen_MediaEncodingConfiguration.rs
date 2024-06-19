#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = MediaEncodingConfiguration)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MediaEncodingConfiguration` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaEncodingConfiguration`*"]
    pub type MediaEncodingConfiguration;
    #[cfg(feature = "AudioConfiguration")]
    #[doc = "Get the `audio` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioConfiguration`, `MediaEncodingConfiguration`*"]
    #[wasm_bindgen(method, getter = "audio")]
    pub fn get_audio(this: &MediaEncodingConfiguration) -> Option<AudioConfiguration>;
    #[cfg(feature = "AudioConfiguration")]
    #[wasm_bindgen(method, setter = "audio")]
    fn set_audio(this: &MediaEncodingConfiguration, val: &AudioConfiguration);
    #[cfg(feature = "VideoConfiguration")]
    #[doc = "Get the `video` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaEncodingConfiguration`, `VideoConfiguration`*"]
    #[wasm_bindgen(method, getter = "video")]
    pub fn get_video(this: &MediaEncodingConfiguration) -> Option<VideoConfiguration>;
    #[cfg(feature = "VideoConfiguration")]
    #[wasm_bindgen(method, setter = "video")]
    fn set_video(this: &MediaEncodingConfiguration, val: &VideoConfiguration);
    #[cfg(feature = "MediaEncodingType")]
    #[doc = "Get the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaEncodingConfiguration`, `MediaEncodingType`*"]
    #[wasm_bindgen(method, getter = "type")]
    pub fn get_type(this: &MediaEncodingConfiguration) -> MediaEncodingType;
    #[cfg(feature = "MediaEncodingType")]
    #[wasm_bindgen(method, setter = "type")]
    fn set_type(this: &MediaEncodingConfiguration, val: MediaEncodingType);
}
impl MediaEncodingConfiguration {
    #[cfg(feature = "MediaEncodingType")]
    #[doc = "Construct a new `MediaEncodingConfiguration`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaEncodingConfiguration`, `MediaEncodingType`*"]
    pub fn new(type_: MediaEncodingType) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.type_(type_);
        ret
    }
    #[cfg(feature = "AudioConfiguration")]
    #[doc = "Change the `audio` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioConfiguration`, `MediaEncodingConfiguration`*"]
    pub fn audio(&mut self, val: &AudioConfiguration) -> &mut Self {
        self.set_audio(val);
        self
    }
    #[cfg(feature = "VideoConfiguration")]
    #[doc = "Change the `video` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaEncodingConfiguration`, `VideoConfiguration`*"]
    pub fn video(&mut self, val: &VideoConfiguration) -> &mut Self {
        self.set_video(val);
        self
    }
    #[cfg(feature = "MediaEncodingType")]
    #[doc = "Change the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaEncodingConfiguration`, `MediaEncodingType`*"]
    pub fn type_(&mut self, val: MediaEncodingType) -> &mut Self {
        self.set_type(val);
        self
    }
}
