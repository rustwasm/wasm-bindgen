use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = WorkletGlobalScope , extends = :: js_sys :: Object , js_name = AudioWorkletGlobalScope , typescript_name = AudioWorkletGlobalScope ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `AudioWorkletGlobalScope` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioWorkletGlobalScope)
    ///
    ///*This API requires the following crate features to be activated: `AudioWorkletGlobalScope`*
    pub type AudioWorkletGlobalScope;

    # [ wasm_bindgen ( structural , method , getter , js_class = "AudioWorkletGlobalScope" , js_name = currentFrame ) ]
    ///Getter for the `currentFrame` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioWorkletGlobalScope/currentFrame)
    ///
    ///*This API requires the following crate features to be activated: `AudioWorkletGlobalScope`*
    pub fn current_frame(this: &AudioWorkletGlobalScope) -> f64;

    # [ wasm_bindgen ( structural , method , getter , js_class = "AudioWorkletGlobalScope" , js_name = currentTime ) ]
    ///Getter for the `currentTime` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioWorkletGlobalScope/currentTime)
    ///
    ///*This API requires the following crate features to be activated: `AudioWorkletGlobalScope`*
    pub fn current_time(this: &AudioWorkletGlobalScope) -> f64;

    # [ wasm_bindgen ( structural , method , getter , js_class = "AudioWorkletGlobalScope" , js_name = sampleRate ) ]
    ///Getter for the `sampleRate` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioWorkletGlobalScope/sampleRate)
    ///
    ///*This API requires the following crate features to be activated: `AudioWorkletGlobalScope`*
    pub fn sample_rate(this: &AudioWorkletGlobalScope) -> f32;

    # [ wasm_bindgen ( method , structural , js_class = "AudioWorkletGlobalScope" , js_name = registerProcessor ) ]
    ///The `registerProcessor()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioWorkletGlobalScope/registerProcessor)
    ///
    ///*This API requires the following crate features to be activated: `AudioWorkletGlobalScope`*
    pub fn register_processor(
        this: &AudioWorkletGlobalScope,
        name: &str,
        processor_ctor: &::js_sys::Function,
    );

}
