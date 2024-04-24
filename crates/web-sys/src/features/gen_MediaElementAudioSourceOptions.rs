#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = MediaElementAudioSourceOptions)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MediaElementAudioSourceOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaElementAudioSourceOptions`*"]
    pub type MediaElementAudioSourceOptions;
    #[cfg(feature = "HtmlMediaElement")]
    #[wasm_bindgen(method, getter = "mediaElement")]
    fn media_element_shim(this: &MediaElementAudioSourceOptions) -> &HtmlMediaElement;
    #[cfg(feature = "HtmlMediaElement")]
    #[wasm_bindgen(method, setter = "mediaElement")]
    fn set_media_element_shim(this: &MediaElementAudioSourceOptions, val: &HtmlMediaElement);
}
#[doc = "The trait to access properties on the `MediaElementAudioSourceOptions` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `MediaElementAudioSourceOptions`*"]
pub trait MediaElementAudioSourceOptionsGetters {
    #[cfg(feature = "HtmlMediaElement")]
    #[doc = "Get the `mediaElement` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMediaElement`, `MediaElementAudioSourceOptions`*"]
    fn media_element(&self) -> &HtmlMediaElement;
}
impl MediaElementAudioSourceOptionsGetters for MediaElementAudioSourceOptions {
    #[cfg(feature = "HtmlMediaElement")]
    fn media_element(&self) -> &HtmlMediaElement {
        self.media_element_shim()
    }
}
impl MediaElementAudioSourceOptions {
    #[cfg(feature = "HtmlMediaElement")]
    #[doc = "Construct a new `MediaElementAudioSourceOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMediaElement`, `MediaElementAudioSourceOptions`*"]
    pub fn new(media_element: &HtmlMediaElement) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.media_element(media_element);
        ret
    }
    #[cfg(feature = "HtmlMediaElement")]
    #[doc = "Change the `mediaElement` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMediaElement`, `MediaElementAudioSourceOptions`*"]
    pub fn media_element(&mut self, val: &HtmlMediaElement) -> &mut Self {
        self.set_media_element_shim(val);
        self
    }
}
