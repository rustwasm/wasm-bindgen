use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = AudioNode , extends = EventTarget , extends = :: js_sys :: Object , js_name = MediaStreamAudioDestinationNode , typescript_type = "MediaStreamAudioDestinationNode" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `MediaStreamAudioDestinationNode` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamAudioDestinationNode)
    ///
    ///*This API requires the following crate features to be activated: `MediaStreamAudioDestinationNode`*
    pub type MediaStreamAudioDestinationNode;

    #[cfg(feature = "MediaStream")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "MediaStreamAudioDestinationNode" , js_name = stream ) ]
    ///Getter for the `stream` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamAudioDestinationNode/stream)
    ///
    ///*This API requires the following crate features to be activated: `MediaStream`, `MediaStreamAudioDestinationNode`*
    pub fn stream(this: &MediaStreamAudioDestinationNode) -> MediaStream;

    #[cfg(feature = "AudioContext")]
    #[wasm_bindgen(catch, constructor, js_class = "MediaStreamAudioDestinationNode")]
    ///The `new MediaStreamAudioDestinationNode(..)` constructor, creating a new instance of `MediaStreamAudioDestinationNode`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamAudioDestinationNode/MediaStreamAudioDestinationNode)
    ///
    ///*This API requires the following crate features to be activated: `AudioContext`, `MediaStreamAudioDestinationNode`*
    pub fn new(context: &AudioContext) -> Result<MediaStreamAudioDestinationNode, JsValue>;

    #[cfg(all(feature = "AudioContext", feature = "AudioNodeOptions",))]
    #[wasm_bindgen(catch, constructor, js_class = "MediaStreamAudioDestinationNode")]
    ///The `new MediaStreamAudioDestinationNode(..)` constructor, creating a new instance of `MediaStreamAudioDestinationNode`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamAudioDestinationNode/MediaStreamAudioDestinationNode)
    ///
    ///*This API requires the following crate features to be activated: `AudioContext`, `AudioNodeOptions`, `MediaStreamAudioDestinationNode`*
    pub fn new_with_options(
        context: &AudioContext,
        options: &AudioNodeOptions,
    ) -> Result<MediaStreamAudioDestinationNode, JsValue>;

}
