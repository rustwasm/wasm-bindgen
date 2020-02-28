use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = WorkletGlobalScope , extends = :: js_sys :: Object , js_name = AudioWorkletGlobalScope , typescript_name = AudioWorkletGlobalScope ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AudioWorkletGlobalScope` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioWorkletGlobalScope)\n\n*This API requires the following crate features to be activated: `AudioWorkletGlobalScope`*"]
    pub type AudioWorkletGlobalScope;
    # [ wasm_bindgen ( structural , method , getter , js_name = currentFrame ) ]
    #[doc = "Getter for the `currentFrame` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioWorkletGlobalScope/currentFrame)\n\n*This API requires the following crate features to be activated: `AudioWorkletGlobalScope`*"]
    pub fn current_frame(this: &AudioWorkletGlobalScope) -> f64;
    # [ wasm_bindgen ( structural , method , getter , js_name = currentTime ) ]
    #[doc = "Getter for the `currentTime` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioWorkletGlobalScope/currentTime)\n\n*This API requires the following crate features to be activated: `AudioWorkletGlobalScope`*"]
    pub fn current_time(this: &AudioWorkletGlobalScope) -> f64;
    # [ wasm_bindgen ( structural , method , getter , js_name = sampleRate ) ]
    #[doc = "Getter for the `sampleRate` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioWorkletGlobalScope/sampleRate)\n\n*This API requires the following crate features to be activated: `AudioWorkletGlobalScope`*"]
    pub fn sample_rate(this: &AudioWorkletGlobalScope) -> f32;
    # [ wasm_bindgen ( method , structural , js_name = registerProcessor ) ]
    #[doc = "The `registerProcessor()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioWorkletGlobalScope/registerProcessor)\n\n*This API requires the following crate features to be activated: `AudioWorkletGlobalScope`*"]
    pub fn register_processor(
        this: &AudioWorkletGlobalScope,
        name: &str,
        processor_ctor: &::js_sys::Function,
    );
}
