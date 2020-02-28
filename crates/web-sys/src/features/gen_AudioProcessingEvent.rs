use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = Event , extends = :: js_sys :: Object , js_name = AudioProcessingEvent , typescript_name = AudioProcessingEvent ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AudioProcessingEvent` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioProcessingEvent)\n\n*This API requires the following crate features to be activated: `AudioProcessingEvent`*"]
    pub type AudioProcessingEvent;
    # [ wasm_bindgen ( structural , method , getter , js_name = playbackTime ) ]
    #[doc = "Getter for the `playbackTime` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioProcessingEvent/playbackTime)\n\n*This API requires the following crate features to be activated: `AudioProcessingEvent`*"]
    pub fn playback_time(this: &AudioProcessingEvent) -> f64;
    # [ wasm_bindgen ( structural , catch , method , getter , js_name = inputBuffer ) ]
    #[cfg(feature = "AudioBuffer")]
    #[doc = "Getter for the `inputBuffer` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioProcessingEvent/inputBuffer)\n\n*This API requires the following crate features to be activated: `AudioBuffer`, `AudioProcessingEvent`*"]
    pub fn input_buffer(this: &AudioProcessingEvent) -> Result<AudioBuffer, JsValue>;
    # [ wasm_bindgen ( structural , catch , method , getter , js_name = outputBuffer ) ]
    #[cfg(feature = "AudioBuffer")]
    #[doc = "Getter for the `outputBuffer` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioProcessingEvent/outputBuffer)\n\n*This API requires the following crate features to be activated: `AudioBuffer`, `AudioProcessingEvent`*"]
    pub fn output_buffer(this: &AudioProcessingEvent) -> Result<AudioBuffer, JsValue>;
}
