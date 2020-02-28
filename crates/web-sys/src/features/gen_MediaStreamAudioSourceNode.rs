use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = AudioNode , extends = EventTarget , extends = :: js_sys :: Object , js_name = MediaStreamAudioSourceNode , typescript_name = MediaStreamAudioSourceNode ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MediaStreamAudioSourceNode` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamAudioSourceNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamAudioSourceNode`*"]
    pub type MediaStreamAudioSourceNode;
    #[cfg(all(feature = "AudioContext", feature = "MediaStreamAudioSourceOptions",))]
    #[wasm_bindgen(catch, js_class = "MediaStreamAudioSourceNode", constructor)]
    #[doc = "The `new MediaStreamAudioSourceNode(..)` constructor, creating a new instance of `MediaStreamAudioSourceNode`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamAudioSourceNode/MediaStreamAudioSourceNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioContext`, `MediaStreamAudioSourceNode`, `MediaStreamAudioSourceOptions`*"]
    pub fn new(
        this: &MediaStreamAudioSourceNode,
        context: &AudioContext,
        options: &MediaStreamAudioSourceOptions,
    ) -> Result<MediaStreamAudioSourceNode, JsValue>;
}
