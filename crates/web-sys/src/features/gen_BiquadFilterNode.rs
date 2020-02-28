use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = AudioNode , extends = EventTarget , extends = :: js_sys :: Object , js_name = BiquadFilterNode , typescript_name = BiquadFilterNode ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `BiquadFilterNode` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BiquadFilterNode)\n\n*This API requires the following crate features to be activated: `BiquadFilterNode`*"]
    pub type BiquadFilterNode;
    # [ wasm_bindgen ( structural , method , getter , js_name = type ) ]
    #[cfg(feature = "BiquadFilterType")]
    #[doc = "Getter for the `type` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BiquadFilterNode/type)\n\n*This API requires the following crate features to be activated: `BiquadFilterNode`, `BiquadFilterType`*"]
    pub fn type_(this: &BiquadFilterNode) -> BiquadFilterType;
    # [ wasm_bindgen ( structural , method , setter , js_name = type ) ]
    #[cfg(feature = "BiquadFilterType")]
    #[doc = "Setter for the `type` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BiquadFilterNode/type)\n\n*This API requires the following crate features to be activated: `BiquadFilterNode`, `BiquadFilterType`*"]
    pub fn set_type(this: &BiquadFilterNode, value: BiquadFilterType);
    # [ wasm_bindgen ( structural , method , getter , js_name = frequency ) ]
    #[cfg(feature = "AudioParam")]
    #[doc = "Getter for the `frequency` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BiquadFilterNode/frequency)\n\n*This API requires the following crate features to be activated: `AudioParam`, `BiquadFilterNode`*"]
    pub fn frequency(this: &BiquadFilterNode) -> AudioParam;
    # [ wasm_bindgen ( structural , method , getter , js_name = detune ) ]
    #[cfg(feature = "AudioParam")]
    #[doc = "Getter for the `detune` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BiquadFilterNode/detune)\n\n*This API requires the following crate features to be activated: `AudioParam`, `BiquadFilterNode`*"]
    pub fn detune(this: &BiquadFilterNode) -> AudioParam;
    # [ wasm_bindgen ( structural , method , getter , js_name = Q ) ]
    #[cfg(feature = "AudioParam")]
    #[doc = "Getter for the `Q` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BiquadFilterNode/Q)\n\n*This API requires the following crate features to be activated: `AudioParam`, `BiquadFilterNode`*"]
    pub fn q(this: &BiquadFilterNode) -> AudioParam;
    # [ wasm_bindgen ( structural , method , getter , js_name = gain ) ]
    #[cfg(feature = "AudioParam")]
    #[doc = "Getter for the `gain` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BiquadFilterNode/gain)\n\n*This API requires the following crate features to be activated: `AudioParam`, `BiquadFilterNode`*"]
    pub fn gain(this: &BiquadFilterNode) -> AudioParam;
    #[cfg(feature = "BaseAudioContext")]
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new BiquadFilterNode(..)` constructor, creating a new instance of `BiquadFilterNode`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BiquadFilterNode/BiquadFilterNode)\n\n*This API requires the following crate features to be activated: `BaseAudioContext`, `BiquadFilterNode`*"]
    pub fn new(
        this: &BiquadFilterNode,
        context: &BaseAudioContext,
    ) -> Result<BiquadFilterNode, JsValue>;
    #[cfg(all(feature = "BaseAudioContext", feature = "BiquadFilterOptions",))]
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new BiquadFilterNode(..)` constructor, creating a new instance of `BiquadFilterNode`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BiquadFilterNode/BiquadFilterNode)\n\n*This API requires the following crate features to be activated: `BaseAudioContext`, `BiquadFilterNode`, `BiquadFilterOptions`*"]
    pub fn new_with_options(
        this: &BiquadFilterNode,
        context: &BaseAudioContext,
        options: &BiquadFilterOptions,
    ) -> Result<BiquadFilterNode, JsValue>;
    # [ wasm_bindgen ( method , structural , js_name = getFrequencyResponse ) ]
    #[doc = "The `getFrequencyResponse()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BiquadFilterNode/getFrequencyResponse)\n\n*This API requires the following crate features to be activated: `BiquadFilterNode`*"]
    pub fn get_frequency_response(
        this: &BiquadFilterNode,
        frequency_hz: &mut [f32],
        mag_response: &mut [f32],
        phase_response: &mut [f32],
    );
}
