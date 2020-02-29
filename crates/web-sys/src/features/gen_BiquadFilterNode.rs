use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = AudioNode , extends = EventTarget , extends = :: js_sys :: Object , js_name = BiquadFilterNode , typescript_type = "BiquadFilterNode" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `BiquadFilterNode` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BiquadFilterNode)
    ///
    ///*This API requires the following crate features to be activated: `BiquadFilterNode`*
    pub type BiquadFilterNode;

    #[cfg(feature = "BiquadFilterType")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "BiquadFilterNode" , js_name = type ) ]
    ///Getter for the `type` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BiquadFilterNode/type)
    ///
    ///*This API requires the following crate features to be activated: `BiquadFilterNode`, `BiquadFilterType`*
    pub fn type_(this: &BiquadFilterNode) -> BiquadFilterType;

    #[cfg(feature = "BiquadFilterType")]
    # [ wasm_bindgen ( structural , method , setter , js_class = "BiquadFilterNode" , js_name = type ) ]
    ///Setter for the `type` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BiquadFilterNode/type)
    ///
    ///*This API requires the following crate features to be activated: `BiquadFilterNode`, `BiquadFilterType`*
    pub fn set_type(this: &BiquadFilterNode, value: BiquadFilterType);

    #[cfg(feature = "AudioParam")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "BiquadFilterNode" , js_name = frequency ) ]
    ///Getter for the `frequency` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BiquadFilterNode/frequency)
    ///
    ///*This API requires the following crate features to be activated: `AudioParam`, `BiquadFilterNode`*
    pub fn frequency(this: &BiquadFilterNode) -> AudioParam;

    #[cfg(feature = "AudioParam")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "BiquadFilterNode" , js_name = detune ) ]
    ///Getter for the `detune` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BiquadFilterNode/detune)
    ///
    ///*This API requires the following crate features to be activated: `AudioParam`, `BiquadFilterNode`*
    pub fn detune(this: &BiquadFilterNode) -> AudioParam;

    #[cfg(feature = "AudioParam")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "BiquadFilterNode" , js_name = Q ) ]
    ///Getter for the `Q` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BiquadFilterNode/Q)
    ///
    ///*This API requires the following crate features to be activated: `AudioParam`, `BiquadFilterNode`*
    pub fn q(this: &BiquadFilterNode) -> AudioParam;

    #[cfg(feature = "AudioParam")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "BiquadFilterNode" , js_name = gain ) ]
    ///Getter for the `gain` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BiquadFilterNode/gain)
    ///
    ///*This API requires the following crate features to be activated: `AudioParam`, `BiquadFilterNode`*
    pub fn gain(this: &BiquadFilterNode) -> AudioParam;

    #[cfg(feature = "BaseAudioContext")]
    #[wasm_bindgen(catch, constructor, js_class = "BiquadFilterNode")]
    ///The `new BiquadFilterNode(..)` constructor, creating a new instance of `BiquadFilterNode`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BiquadFilterNode/BiquadFilterNode)
    ///
    ///*This API requires the following crate features to be activated: `BaseAudioContext`, `BiquadFilterNode`*
    pub fn new(context: &BaseAudioContext) -> Result<BiquadFilterNode, JsValue>;

    #[cfg(all(feature = "BaseAudioContext", feature = "BiquadFilterOptions",))]
    #[wasm_bindgen(catch, constructor, js_class = "BiquadFilterNode")]
    ///The `new BiquadFilterNode(..)` constructor, creating a new instance of `BiquadFilterNode`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BiquadFilterNode/BiquadFilterNode)
    ///
    ///*This API requires the following crate features to be activated: `BaseAudioContext`, `BiquadFilterNode`, `BiquadFilterOptions`*
    pub fn new_with_options(
        context: &BaseAudioContext,
        options: &BiquadFilterOptions,
    ) -> Result<BiquadFilterNode, JsValue>;

    # [ wasm_bindgen ( method , structural , js_class = "BiquadFilterNode" , js_name = getFrequencyResponse ) ]
    ///The `getFrequencyResponse()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BiquadFilterNode/getFrequencyResponse)
    ///
    ///*This API requires the following crate features to be activated: `BiquadFilterNode`*
    pub fn get_frequency_response(
        this: &BiquadFilterNode,
        frequency_hz: &mut [f32],
        mag_response: &mut [f32],
        phase_response: &mut [f32],
    );

}
