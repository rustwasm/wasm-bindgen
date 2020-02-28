use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = AudioScheduledSourceNode , extends = AudioNode , extends = EventTarget , extends = :: js_sys :: Object , js_name = OscillatorNode , typescript_name = OscillatorNode ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `OscillatorNode` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OscillatorNode)\n\n*This API requires the following crate features to be activated: `OscillatorNode`*"]
    pub type OscillatorNode;
    # [ wasm_bindgen ( structural , method , getter , js_name = type ) ]
    #[cfg(feature = "OscillatorType")]
    #[doc = "Getter for the `type` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OscillatorNode/type)\n\n*This API requires the following crate features to be activated: `OscillatorNode`, `OscillatorType`*"]
    pub fn type_(this: &OscillatorNode) -> OscillatorType;
    # [ wasm_bindgen ( structural , method , setter , js_name = type ) ]
    #[cfg(feature = "OscillatorType")]
    #[doc = "Setter for the `type` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OscillatorNode/type)\n\n*This API requires the following crate features to be activated: `OscillatorNode`, `OscillatorType`*"]
    pub fn set_type(this: &OscillatorNode, value: OscillatorType);
    # [ wasm_bindgen ( structural , method , getter , js_name = frequency ) ]
    #[cfg(feature = "AudioParam")]
    #[doc = "Getter for the `frequency` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OscillatorNode/frequency)\n\n*This API requires the following crate features to be activated: `AudioParam`, `OscillatorNode`*"]
    pub fn frequency(this: &OscillatorNode) -> AudioParam;
    # [ wasm_bindgen ( structural , method , getter , js_name = detune ) ]
    #[cfg(feature = "AudioParam")]
    #[doc = "Getter for the `detune` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OscillatorNode/detune)\n\n*This API requires the following crate features to be activated: `AudioParam`, `OscillatorNode`*"]
    pub fn detune(this: &OscillatorNode) -> AudioParam;
    # [ wasm_bindgen ( structural , method , getter , js_name = onended ) ]
    #[doc = "Getter for the `onended` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OscillatorNode/onended)\n\n*This API requires the following crate features to be activated: `OscillatorNode`*"]
    pub fn onended(this: &OscillatorNode) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onended ) ]
    #[doc = "Setter for the `onended` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OscillatorNode/onended)\n\n*This API requires the following crate features to be activated: `OscillatorNode`*"]
    pub fn set_onended(this: &OscillatorNode, value: Option<::js_sys::Function>);
    #[cfg(feature = "BaseAudioContext")]
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new OscillatorNode(..)` constructor, creating a new instance of `OscillatorNode`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OscillatorNode/OscillatorNode)\n\n*This API requires the following crate features to be activated: `BaseAudioContext`, `OscillatorNode`*"]
    pub fn new(
        this: &OscillatorNode,
        context: &BaseAudioContext,
    ) -> Result<OscillatorNode, JsValue>;
    #[cfg(all(feature = "BaseAudioContext", feature = "OscillatorOptions",))]
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new OscillatorNode(..)` constructor, creating a new instance of `OscillatorNode`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OscillatorNode/OscillatorNode)\n\n*This API requires the following crate features to be activated: `BaseAudioContext`, `OscillatorNode`, `OscillatorOptions`*"]
    pub fn new_with_options(
        this: &OscillatorNode,
        context: &BaseAudioContext,
        options: &OscillatorOptions,
    ) -> Result<OscillatorNode, JsValue>;
    #[cfg(feature = "PeriodicWave")]
    # [ wasm_bindgen ( method , structural , js_name = setPeriodicWave ) ]
    #[doc = "The `setPeriodicWave()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OscillatorNode/setPeriodicWave)\n\n*This API requires the following crate features to be activated: `OscillatorNode`, `PeriodicWave`*"]
    pub fn set_periodic_wave(this: &OscillatorNode, periodic_wave: &PeriodicWave);
    # [ wasm_bindgen ( catch , method , structural , js_name = start ) ]
    #[doc = "The `start()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OscillatorNode/start)\n\n*This API requires the following crate features to be activated: `OscillatorNode`*"]
    pub fn start(this: &OscillatorNode) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = start ) ]
    #[doc = "The `start()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OscillatorNode/start)\n\n*This API requires the following crate features to be activated: `OscillatorNode`*"]
    pub fn start_with_when(this: &OscillatorNode, when: f64) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = stop ) ]
    #[doc = "The `stop()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OscillatorNode/stop)\n\n*This API requires the following crate features to be activated: `OscillatorNode`*"]
    pub fn stop(this: &OscillatorNode) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = stop ) ]
    #[doc = "The `stop()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OscillatorNode/stop)\n\n*This API requires the following crate features to be activated: `OscillatorNode`*"]
    pub fn stop_with_when(this: &OscillatorNode, when: f64) -> Result<(), JsValue>;
}
