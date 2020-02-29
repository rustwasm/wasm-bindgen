use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = AudioParam , typescript_name = AudioParam ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `AudioParam` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioParam)
    ///
    ///*This API requires the following crate features to be activated: `AudioParam`*
    pub type AudioParam;

    # [ wasm_bindgen ( structural , method , getter , js_class = "AudioParam" , js_name = value ) ]
    ///Getter for the `value` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioParam/value)
    ///
    ///*This API requires the following crate features to be activated: `AudioParam`*
    pub fn value(this: &AudioParam) -> f32;

    # [ wasm_bindgen ( structural , method , setter , js_class = "AudioParam" , js_name = value ) ]
    ///Setter for the `value` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioParam/value)
    ///
    ///*This API requires the following crate features to be activated: `AudioParam`*
    pub fn set_value(this: &AudioParam, value: f32);

    # [ wasm_bindgen ( structural , method , getter , js_class = "AudioParam" , js_name = defaultValue ) ]
    ///Getter for the `defaultValue` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioParam/defaultValue)
    ///
    ///*This API requires the following crate features to be activated: `AudioParam`*
    pub fn default_value(this: &AudioParam) -> f32;

    # [ wasm_bindgen ( structural , method , getter , js_class = "AudioParam" , js_name = minValue ) ]
    ///Getter for the `minValue` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioParam/minValue)
    ///
    ///*This API requires the following crate features to be activated: `AudioParam`*
    pub fn min_value(this: &AudioParam) -> f32;

    # [ wasm_bindgen ( structural , method , getter , js_class = "AudioParam" , js_name = maxValue ) ]
    ///Getter for the `maxValue` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioParam/maxValue)
    ///
    ///*This API requires the following crate features to be activated: `AudioParam`*
    pub fn max_value(this: &AudioParam) -> f32;

    # [ wasm_bindgen ( catch , method , structural , js_class = "AudioParam" , js_name = cancelScheduledValues ) ]
    ///The `cancelScheduledValues()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioParam/cancelScheduledValues)
    ///
    ///*This API requires the following crate features to be activated: `AudioParam`*
    pub fn cancel_scheduled_values(
        this: &AudioParam,
        start_time: f64,
    ) -> Result<AudioParam, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "AudioParam" , js_name = exponentialRampToValueAtTime ) ]
    ///The `exponentialRampToValueAtTime()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioParam/exponentialRampToValueAtTime)
    ///
    ///*This API requires the following crate features to be activated: `AudioParam`*
    pub fn exponential_ramp_to_value_at_time(
        this: &AudioParam,
        value: f32,
        end_time: f64,
    ) -> Result<AudioParam, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "AudioParam" , js_name = linearRampToValueAtTime ) ]
    ///The `linearRampToValueAtTime()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioParam/linearRampToValueAtTime)
    ///
    ///*This API requires the following crate features to be activated: `AudioParam`*
    pub fn linear_ramp_to_value_at_time(
        this: &AudioParam,
        value: f32,
        end_time: f64,
    ) -> Result<AudioParam, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "AudioParam" , js_name = setTargetAtTime ) ]
    ///The `setTargetAtTime()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioParam/setTargetAtTime)
    ///
    ///*This API requires the following crate features to be activated: `AudioParam`*
    pub fn set_target_at_time(
        this: &AudioParam,
        target: f32,
        start_time: f64,
        time_constant: f64,
    ) -> Result<AudioParam, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "AudioParam" , js_name = setValueAtTime ) ]
    ///The `setValueAtTime()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioParam/setValueAtTime)
    ///
    ///*This API requires the following crate features to be activated: `AudioParam`*
    pub fn set_value_at_time(
        this: &AudioParam,
        value: f32,
        start_time: f64,
    ) -> Result<AudioParam, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "AudioParam" , js_name = setValueCurveAtTime ) ]
    ///The `setValueCurveAtTime()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioParam/setValueCurveAtTime)
    ///
    ///*This API requires the following crate features to be activated: `AudioParam`*
    pub fn set_value_curve_at_time(
        this: &AudioParam,
        values: &mut [f32],
        start_time: f64,
        duration: f64,
    ) -> Result<AudioParam, JsValue>;

}
