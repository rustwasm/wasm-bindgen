use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = AudioNode , extends = EventTarget , extends = :: js_sys :: Object , js_name = MediaElementAudioSourceNode , typescript_name = MediaElementAudioSourceNode ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `MediaElementAudioSourceNode` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaElementAudioSourceNode)
    ///
    ///*This API requires the following crate features to be activated: `MediaElementAudioSourceNode`*
    pub type MediaElementAudioSourceNode;

    #[cfg(all(feature = "AudioContext", feature = "MediaElementAudioSourceOptions",))]
    #[wasm_bindgen(catch, constructor, js_class = "MediaElementAudioSourceNode")]
    ///The `new MediaElementAudioSourceNode(..)` constructor, creating a new instance of `MediaElementAudioSourceNode`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaElementAudioSourceNode/MediaElementAudioSourceNode)
    ///
    ///*This API requires the following crate features to be activated: `AudioContext`, `MediaElementAudioSourceNode`, `MediaElementAudioSourceOptions`*
    pub fn new(
        context: &AudioContext,
        options: &MediaElementAudioSourceOptions,
    ) -> Result<MediaElementAudioSourceNode, JsValue>;

}
