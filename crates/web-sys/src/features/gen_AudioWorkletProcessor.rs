use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = AudioWorkletProcessor , typescript_name = AudioWorkletProcessor ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AudioWorkletProcessor` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioWorkletProcessor)\n\n*This API requires the following crate features to be activated: `AudioWorkletProcessor`*"]
    pub type AudioWorkletProcessor;
    # [ wasm_bindgen ( structural , catch , method , getter , js_name = port ) ]
    #[cfg(feature = "MessagePort")]
    #[doc = "Getter for the `port` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioWorkletProcessor/port)\n\n*This API requires the following crate features to be activated: `AudioWorkletProcessor`, `MessagePort`*"]
    pub fn port(this: &AudioWorkletProcessor) -> Result<MessagePort, JsValue>;
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new AudioWorkletProcessor(..)` constructor, creating a new instance of `AudioWorkletProcessor`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioWorkletProcessor/AudioWorkletProcessor)\n\n*This API requires the following crate features to be activated: `AudioWorkletProcessor`*"]
    pub fn new(this: &AudioWorkletProcessor) -> Result<AudioWorkletProcessor, JsValue>;
    #[cfg(feature = "AudioWorkletNodeOptions")]
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new AudioWorkletProcessor(..)` constructor, creating a new instance of `AudioWorkletProcessor`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioWorkletProcessor/AudioWorkletProcessor)\n\n*This API requires the following crate features to be activated: `AudioWorkletNodeOptions`, `AudioWorkletProcessor`*"]
    pub fn new_with_options(
        this: &AudioWorkletProcessor,
        options: &AudioWorkletNodeOptions,
    ) -> Result<AudioWorkletProcessor, JsValue>;
}
