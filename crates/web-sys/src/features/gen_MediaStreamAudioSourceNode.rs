use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = AudioNode , extends = EventTarget , extends = :: js_sys :: Object , js_name = MediaStreamAudioSourceNode , typescript_name = MediaStreamAudioSourceNode ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MediaStreamAudioSourceNode` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamAudioSourceNode)\n\n*This API requires the following crate features to be activated: `MediaStreamAudioSourceNode`*"]
    pub type MediaStreamAudioSourceNode;
    #[cfg(all(feature = "AudioContext", feature = "MediaStreamAudioSourceOptions",))]
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new MediaStreamAudioSourceNode(..)` constructor, creating a new instance of `MediaStreamAudioSourceNode`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamAudioSourceNode/MediaStreamAudioSourceNode)\n\n*This API requires the following crate features to be activated: `AudioContext`, `MediaStreamAudioSourceNode`, `MediaStreamAudioSourceOptions`*"]
    pub fn new(
        this: &MediaStreamAudioSourceNode,
        context: &AudioContext,
        options: &MediaStreamAudioSourceOptions,
    ) -> Result<MediaStreamAudioSourceNode, JsValue>;
}
