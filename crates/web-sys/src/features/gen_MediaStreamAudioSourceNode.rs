use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = AudioNode , extends = EventTarget , extends = :: js_sys :: Object , js_name = MediaStreamAudioSourceNode , typescript_type = "MediaStreamAudioSourceNode" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `MediaStreamAudioSourceNode` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamAudioSourceNode)
    ///
    ///*This API requires the following crate features to be activated: `MediaStreamAudioSourceNode`*
    pub type MediaStreamAudioSourceNode;

    #[cfg(all(feature = "AudioContext", feature = "MediaStreamAudioSourceOptions",))]
    #[wasm_bindgen(catch, constructor, js_class = "MediaStreamAudioSourceNode")]
    ///The `new MediaStreamAudioSourceNode(..)` constructor, creating a new instance of `MediaStreamAudioSourceNode`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamAudioSourceNode/MediaStreamAudioSourceNode)
    ///
    ///*This API requires the following crate features to be activated: `AudioContext`, `MediaStreamAudioSourceNode`, `MediaStreamAudioSourceOptions`*
    pub fn new(
        context: &AudioContext,
        options: &MediaStreamAudioSourceOptions,
    ) -> Result<MediaStreamAudioSourceNode, JsValue>;

}
