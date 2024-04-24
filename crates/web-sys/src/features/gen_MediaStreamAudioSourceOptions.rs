#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = MediaStreamAudioSourceOptions)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MediaStreamAudioSourceOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamAudioSourceOptions`*"]
    pub type MediaStreamAudioSourceOptions;
    #[cfg(feature = "MediaStream")]
    #[wasm_bindgen(method, getter = "mediaStream")]
    fn media_stream_shim(this: &MediaStreamAudioSourceOptions) -> &MediaStream;
    #[cfg(feature = "MediaStream")]
    #[wasm_bindgen(method, setter = "mediaStream")]
    fn set_media_stream_shim(this: &MediaStreamAudioSourceOptions, val: &MediaStream);
}
#[doc = "The trait to access properties on the `MediaStreamAudioSourceOptions` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `MediaStreamAudioSourceOptions`*"]
pub trait MediaStreamAudioSourceOptionsGetters {
    #[cfg(feature = "MediaStream")]
    #[doc = "Get the `mediaStream` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStream`, `MediaStreamAudioSourceOptions`*"]
    fn media_stream(&self) -> &MediaStream;
}
impl MediaStreamAudioSourceOptionsGetters for MediaStreamAudioSourceOptions {
    #[cfg(feature = "MediaStream")]
    fn media_stream(&self) -> &MediaStream {
        self.media_stream_shim()
    }
}
impl MediaStreamAudioSourceOptions {
    #[cfg(feature = "MediaStream")]
    #[doc = "Construct a new `MediaStreamAudioSourceOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStream`, `MediaStreamAudioSourceOptions`*"]
    pub fn new(media_stream: &MediaStream) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.media_stream(media_stream);
        ret
    }
    #[cfg(feature = "MediaStream")]
    #[doc = "Change the `mediaStream` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStream`, `MediaStreamAudioSourceOptions`*"]
    pub fn media_stream(&mut self, val: &MediaStream) -> &mut Self {
        self.set_media_stream_shim(val);
        self
    }
}
