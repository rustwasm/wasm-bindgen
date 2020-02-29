use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = Event , extends = :: js_sys :: Object , js_name = AudioProcessingEvent , typescript_name = AudioProcessingEvent ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `AudioProcessingEvent` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioProcessingEvent)
    ///
    ///*This API requires the following crate features to be activated: `AudioProcessingEvent`*
    pub type AudioProcessingEvent;

    # [ wasm_bindgen ( structural , method , getter , js_class = "AudioProcessingEvent" , js_name = playbackTime ) ]
    ///Getter for the `playbackTime` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioProcessingEvent/playbackTime)
    ///
    ///*This API requires the following crate features to be activated: `AudioProcessingEvent`*
    pub fn playback_time(this: &AudioProcessingEvent) -> f64;

    #[cfg(feature = "AudioBuffer")]
    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "AudioProcessingEvent" , js_name = inputBuffer ) ]
    ///Getter for the `inputBuffer` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioProcessingEvent/inputBuffer)
    ///
    ///*This API requires the following crate features to be activated: `AudioBuffer`, `AudioProcessingEvent`*
    pub fn input_buffer(this: &AudioProcessingEvent) -> Result<AudioBuffer, JsValue>;

    #[cfg(feature = "AudioBuffer")]
    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "AudioProcessingEvent" , js_name = outputBuffer ) ]
    ///Getter for the `outputBuffer` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioProcessingEvent/outputBuffer)
    ///
    ///*This API requires the following crate features to be activated: `AudioBuffer`, `AudioProcessingEvent`*
    pub fn output_buffer(this: &AudioProcessingEvent) -> Result<AudioBuffer, JsValue>;

}
