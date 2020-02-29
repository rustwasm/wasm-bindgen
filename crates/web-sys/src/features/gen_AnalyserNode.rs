use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = AudioNode , extends = EventTarget , extends = :: js_sys :: Object , js_name = AnalyserNode , typescript_type = "AnalyserNode" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `AnalyserNode` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AnalyserNode)
    ///
    ///*This API requires the following crate features to be activated: `AnalyserNode`*
    pub type AnalyserNode;

    # [ wasm_bindgen ( structural , method , getter , js_class = "AnalyserNode" , js_name = fftSize ) ]
    ///Getter for the `fftSize` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AnalyserNode/fftSize)
    ///
    ///*This API requires the following crate features to be activated: `AnalyserNode`*
    pub fn fft_size(this: &AnalyserNode) -> u32;

    # [ wasm_bindgen ( structural , method , setter , js_class = "AnalyserNode" , js_name = fftSize ) ]
    ///Setter for the `fftSize` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AnalyserNode/fftSize)
    ///
    ///*This API requires the following crate features to be activated: `AnalyserNode`*
    pub fn set_fft_size(this: &AnalyserNode, value: u32);

    # [ wasm_bindgen ( structural , method , getter , js_class = "AnalyserNode" , js_name = frequencyBinCount ) ]
    ///Getter for the `frequencyBinCount` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AnalyserNode/frequencyBinCount)
    ///
    ///*This API requires the following crate features to be activated: `AnalyserNode`*
    pub fn frequency_bin_count(this: &AnalyserNode) -> u32;

    # [ wasm_bindgen ( structural , method , getter , js_class = "AnalyserNode" , js_name = minDecibels ) ]
    ///Getter for the `minDecibels` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AnalyserNode/minDecibels)
    ///
    ///*This API requires the following crate features to be activated: `AnalyserNode`*
    pub fn min_decibels(this: &AnalyserNode) -> f64;

    # [ wasm_bindgen ( structural , method , setter , js_class = "AnalyserNode" , js_name = minDecibels ) ]
    ///Setter for the `minDecibels` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AnalyserNode/minDecibels)
    ///
    ///*This API requires the following crate features to be activated: `AnalyserNode`*
    pub fn set_min_decibels(this: &AnalyserNode, value: f64);

    # [ wasm_bindgen ( structural , method , getter , js_class = "AnalyserNode" , js_name = maxDecibels ) ]
    ///Getter for the `maxDecibels` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AnalyserNode/maxDecibels)
    ///
    ///*This API requires the following crate features to be activated: `AnalyserNode`*
    pub fn max_decibels(this: &AnalyserNode) -> f64;

    # [ wasm_bindgen ( structural , method , setter , js_class = "AnalyserNode" , js_name = maxDecibels ) ]
    ///Setter for the `maxDecibels` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AnalyserNode/maxDecibels)
    ///
    ///*This API requires the following crate features to be activated: `AnalyserNode`*
    pub fn set_max_decibels(this: &AnalyserNode, value: f64);

    # [ wasm_bindgen ( structural , method , getter , js_class = "AnalyserNode" , js_name = smoothingTimeConstant ) ]
    ///Getter for the `smoothingTimeConstant` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AnalyserNode/smoothingTimeConstant)
    ///
    ///*This API requires the following crate features to be activated: `AnalyserNode`*
    pub fn smoothing_time_constant(this: &AnalyserNode) -> f64;

    # [ wasm_bindgen ( structural , method , setter , js_class = "AnalyserNode" , js_name = smoothingTimeConstant ) ]
    ///Setter for the `smoothingTimeConstant` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AnalyserNode/smoothingTimeConstant)
    ///
    ///*This API requires the following crate features to be activated: `AnalyserNode`*
    pub fn set_smoothing_time_constant(this: &AnalyserNode, value: f64);

    #[cfg(feature = "BaseAudioContext")]
    #[wasm_bindgen(catch, constructor, js_class = "AnalyserNode")]
    ///The `new AnalyserNode(..)` constructor, creating a new instance of `AnalyserNode`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AnalyserNode/AnalyserNode)
    ///
    ///*This API requires the following crate features to be activated: `AnalyserNode`, `BaseAudioContext`*
    pub fn new(context: &BaseAudioContext) -> Result<AnalyserNode, JsValue>;

    #[cfg(all(feature = "AnalyserOptions", feature = "BaseAudioContext",))]
    #[wasm_bindgen(catch, constructor, js_class = "AnalyserNode")]
    ///The `new AnalyserNode(..)` constructor, creating a new instance of `AnalyserNode`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AnalyserNode/AnalyserNode)
    ///
    ///*This API requires the following crate features to be activated: `AnalyserNode`, `AnalyserOptions`, `BaseAudioContext`*
    pub fn new_with_options(
        context: &BaseAudioContext,
        options: &AnalyserOptions,
    ) -> Result<AnalyserNode, JsValue>;

    # [ wasm_bindgen ( method , structural , js_class = "AnalyserNode" , js_name = getByteFrequencyData ) ]
    ///The `getByteFrequencyData()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AnalyserNode/getByteFrequencyData)
    ///
    ///*This API requires the following crate features to be activated: `AnalyserNode`*
    pub fn get_byte_frequency_data(this: &AnalyserNode, array: &mut [u8]);

    # [ wasm_bindgen ( method , structural , js_class = "AnalyserNode" , js_name = getByteTimeDomainData ) ]
    ///The `getByteTimeDomainData()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AnalyserNode/getByteTimeDomainData)
    ///
    ///*This API requires the following crate features to be activated: `AnalyserNode`*
    pub fn get_byte_time_domain_data(this: &AnalyserNode, array: &mut [u8]);

    # [ wasm_bindgen ( method , structural , js_class = "AnalyserNode" , js_name = getFloatFrequencyData ) ]
    ///The `getFloatFrequencyData()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AnalyserNode/getFloatFrequencyData)
    ///
    ///*This API requires the following crate features to be activated: `AnalyserNode`*
    pub fn get_float_frequency_data(this: &AnalyserNode, array: &mut [f32]);

    # [ wasm_bindgen ( method , structural , js_class = "AnalyserNode" , js_name = getFloatTimeDomainData ) ]
    ///The `getFloatTimeDomainData()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AnalyserNode/getFloatTimeDomainData)
    ///
    ///*This API requires the following crate features to be activated: `AnalyserNode`*
    pub fn get_float_time_domain_data(this: &AnalyserNode, array: &mut [f32]);

}
