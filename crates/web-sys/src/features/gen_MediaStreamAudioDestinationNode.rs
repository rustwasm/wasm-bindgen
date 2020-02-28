use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = AudioNode , extends = EventTarget , extends = :: js_sys :: Object , js_name = MediaStreamAudioDestinationNode , typescript_name = MediaStreamAudioDestinationNode ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MediaStreamAudioDestinationNode` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamAudioDestinationNode)\n\n*This API requires the following crate features to be activated: `MediaStreamAudioDestinationNode`*"]
    pub type MediaStreamAudioDestinationNode;
    # [ wasm_bindgen ( structural , method , getter , js_class = "MediaStreamAudioDestinationNode" , js_name = stream ) ]
    #[cfg(feature = "MediaStream")]
    #[doc = "Getter for the `stream` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamAudioDestinationNode/stream)\n\n*This API requires the following crate features to be activated: `MediaStream`, `MediaStreamAudioDestinationNode`*"]
    pub fn stream(this: &MediaStreamAudioDestinationNode) -> MediaStream;
    #[cfg(feature = "AudioContext")]
    #[wasm_bindgen(catch, js_class = "MediaStreamAudioDestinationNode", constructor)]
    #[doc = "The `new MediaStreamAudioDestinationNode(..)` constructor, creating a new instance of `MediaStreamAudioDestinationNode`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamAudioDestinationNode/MediaStreamAudioDestinationNode)\n\n*This API requires the following crate features to be activated: `AudioContext`, `MediaStreamAudioDestinationNode`*"]
    pub fn new(
        this: &MediaStreamAudioDestinationNode,
        context: &AudioContext,
    ) -> Result<MediaStreamAudioDestinationNode, JsValue>;
    #[cfg(all(feature = "AudioContext", feature = "AudioNodeOptions",))]
    #[wasm_bindgen(catch, js_class = "MediaStreamAudioDestinationNode", constructor)]
    #[doc = "The `new MediaStreamAudioDestinationNode(..)` constructor, creating a new instance of `MediaStreamAudioDestinationNode`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamAudioDestinationNode/MediaStreamAudioDestinationNode)\n\n*This API requires the following crate features to be activated: `AudioContext`, `AudioNodeOptions`, `MediaStreamAudioDestinationNode`*"]
    pub fn new_with_options(
        this: &MediaStreamAudioDestinationNode,
        context: &AudioContext,
        options: &AudioNodeOptions,
    ) -> Result<MediaStreamAudioDestinationNode, JsValue>;
}
