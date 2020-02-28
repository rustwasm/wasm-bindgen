use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = AudioScheduledSourceNode , extends = AudioNode , extends = EventTarget , extends = :: js_sys :: Object , js_name = AudioBufferSourceNode , typescript_name = AudioBufferSourceNode ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AudioBufferSourceNode` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioBufferSourceNode)\n\n*This API requires the following crate features to be activated: `AudioBufferSourceNode`*"]
    pub type AudioBufferSourceNode;
    # [ wasm_bindgen ( structural , method , getter , js_name = buffer ) ]
    #[cfg(feature = "AudioBuffer")]
    #[doc = "Getter for the `buffer` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioBufferSourceNode/buffer)\n\n*This API requires the following crate features to be activated: `AudioBuffer`, `AudioBufferSourceNode`*"]
    pub fn buffer(this: &AudioBufferSourceNode) -> Option<AudioBuffer>;
    # [ wasm_bindgen ( structural , method , setter , js_name = buffer ) ]
    #[cfg(feature = "AudioBuffer")]
    #[doc = "Setter for the `buffer` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioBufferSourceNode/buffer)\n\n*This API requires the following crate features to be activated: `AudioBuffer`, `AudioBufferSourceNode`*"]
    pub fn set_buffer(this: &AudioBufferSourceNode, value: Option<AudioBuffer>);
    # [ wasm_bindgen ( structural , method , getter , js_name = playbackRate ) ]
    #[cfg(feature = "AudioParam")]
    #[doc = "Getter for the `playbackRate` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioBufferSourceNode/playbackRate)\n\n*This API requires the following crate features to be activated: `AudioBufferSourceNode`, `AudioParam`*"]
    pub fn playback_rate(this: &AudioBufferSourceNode) -> AudioParam;
    # [ wasm_bindgen ( structural , method , getter , js_name = detune ) ]
    #[cfg(feature = "AudioParam")]
    #[doc = "Getter for the `detune` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioBufferSourceNode/detune)\n\n*This API requires the following crate features to be activated: `AudioBufferSourceNode`, `AudioParam`*"]
    pub fn detune(this: &AudioBufferSourceNode) -> AudioParam;
    # [ wasm_bindgen ( structural , method , getter , js_name = loop ) ]
    #[doc = "Getter for the `loop` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioBufferSourceNode/loop)\n\n*This API requires the following crate features to be activated: `AudioBufferSourceNode`*"]
    pub fn loop_(this: &AudioBufferSourceNode) -> bool;
    # [ wasm_bindgen ( structural , method , setter , js_name = loop ) ]
    #[doc = "Setter for the `loop` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioBufferSourceNode/loop)\n\n*This API requires the following crate features to be activated: `AudioBufferSourceNode`*"]
    pub fn set_loop(this: &AudioBufferSourceNode, value: bool);
    # [ wasm_bindgen ( structural , method , getter , js_name = loopStart ) ]
    #[doc = "Getter for the `loopStart` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioBufferSourceNode/loopStart)\n\n*This API requires the following crate features to be activated: `AudioBufferSourceNode`*"]
    pub fn loop_start(this: &AudioBufferSourceNode) -> f64;
    # [ wasm_bindgen ( structural , method , setter , js_name = loopStart ) ]
    #[doc = "Setter for the `loopStart` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioBufferSourceNode/loopStart)\n\n*This API requires the following crate features to be activated: `AudioBufferSourceNode`*"]
    pub fn set_loop_start(this: &AudioBufferSourceNode, value: f64);
    # [ wasm_bindgen ( structural , method , getter , js_name = loopEnd ) ]
    #[doc = "Getter for the `loopEnd` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioBufferSourceNode/loopEnd)\n\n*This API requires the following crate features to be activated: `AudioBufferSourceNode`*"]
    pub fn loop_end(this: &AudioBufferSourceNode) -> f64;
    # [ wasm_bindgen ( structural , method , setter , js_name = loopEnd ) ]
    #[doc = "Setter for the `loopEnd` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioBufferSourceNode/loopEnd)\n\n*This API requires the following crate features to be activated: `AudioBufferSourceNode`*"]
    pub fn set_loop_end(this: &AudioBufferSourceNode, value: f64);
    # [ wasm_bindgen ( structural , method , getter , js_name = onended ) ]
    #[doc = "Getter for the `onended` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioBufferSourceNode/onended)\n\n*This API requires the following crate features to be activated: `AudioBufferSourceNode`*"]
    pub fn onended(this: &AudioBufferSourceNode) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onended ) ]
    #[doc = "Setter for the `onended` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioBufferSourceNode/onended)\n\n*This API requires the following crate features to be activated: `AudioBufferSourceNode`*"]
    pub fn set_onended(this: &AudioBufferSourceNode, value: Option<::js_sys::Function>);
    #[cfg(feature = "BaseAudioContext")]
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new AudioBufferSourceNode(..)` constructor, creating a new instance of `AudioBufferSourceNode`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioBufferSourceNode/AudioBufferSourceNode)\n\n*This API requires the following crate features to be activated: `AudioBufferSourceNode`, `BaseAudioContext`*"]
    pub fn new(
        this: &AudioBufferSourceNode,
        context: &BaseAudioContext,
    ) -> Result<AudioBufferSourceNode, JsValue>;
    #[cfg(all(feature = "AudioBufferSourceOptions", feature = "BaseAudioContext",))]
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new AudioBufferSourceNode(..)` constructor, creating a new instance of `AudioBufferSourceNode`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioBufferSourceNode/AudioBufferSourceNode)\n\n*This API requires the following crate features to be activated: `AudioBufferSourceNode`, `AudioBufferSourceOptions`, `BaseAudioContext`*"]
    pub fn new_with_options(
        this: &AudioBufferSourceNode,
        context: &BaseAudioContext,
        options: &AudioBufferSourceOptions,
    ) -> Result<AudioBufferSourceNode, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = start ) ]
    #[doc = "The `start()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioBufferSourceNode/start)\n\n*This API requires the following crate features to be activated: `AudioBufferSourceNode`*"]
    pub fn start(this: &AudioBufferSourceNode) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = start ) ]
    #[doc = "The `start()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioBufferSourceNode/start)\n\n*This API requires the following crate features to be activated: `AudioBufferSourceNode`*"]
    pub fn start_with_when(this: &AudioBufferSourceNode, when: f64) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = start ) ]
    #[doc = "The `start()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioBufferSourceNode/start)\n\n*This API requires the following crate features to be activated: `AudioBufferSourceNode`*"]
    pub fn start_with_when_and_grain_offset(
        this: &AudioBufferSourceNode,
        when: f64,
        grain_offset: f64,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = start ) ]
    #[doc = "The `start()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioBufferSourceNode/start)\n\n*This API requires the following crate features to be activated: `AudioBufferSourceNode`*"]
    pub fn start_with_when_and_grain_offset_and_grain_duration(
        this: &AudioBufferSourceNode,
        when: f64,
        grain_offset: f64,
        grain_duration: f64,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = stop ) ]
    #[doc = "The `stop()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioBufferSourceNode/stop)\n\n*This API requires the following crate features to be activated: `AudioBufferSourceNode`*"]
    pub fn stop(this: &AudioBufferSourceNode) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = stop ) ]
    #[doc = "The `stop()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioBufferSourceNode/stop)\n\n*This API requires the following crate features to be activated: `AudioBufferSourceNode`*"]
    pub fn stop_with_when(this: &AudioBufferSourceNode, when: f64) -> Result<(), JsValue>;
}
