use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = AudioNode , extends = EventTarget , extends = :: js_sys :: Object , js_name = MediaElementAudioSourceNode , typescript_name = MediaElementAudioSourceNode ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MediaElementAudioSourceNode` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaElementAudioSourceNode)\n\n*This API requires the following crate features to be activated: `MediaElementAudioSourceNode`*"]
    pub type MediaElementAudioSourceNode;
    #[cfg(all(feature = "AudioContext", feature = "MediaElementAudioSourceOptions",))]
    #[wasm_bindgen(catch, js_class = "MediaElementAudioSourceNode", constructor)]
    #[doc = "The `new MediaElementAudioSourceNode(..)` constructor, creating a new instance of `MediaElementAudioSourceNode`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaElementAudioSourceNode/MediaElementAudioSourceNode)\n\n*This API requires the following crate features to be activated: `AudioContext`, `MediaElementAudioSourceNode`, `MediaElementAudioSourceOptions`*"]
    pub fn new(
        this: &MediaElementAudioSourceNode,
        context: &AudioContext,
        options: &MediaElementAudioSourceOptions,
    ) -> Result<MediaElementAudioSourceNode, JsValue>;
}
