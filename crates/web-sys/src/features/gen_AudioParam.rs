use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = AudioParam , typescript_name = AudioParam ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AudioParam` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioParam)\n\n*This API requires the following crate features to be activated: `AudioParam`*"]
    pub type AudioParam;
    # [ wasm_bindgen ( structural , method , getter , js_class = "AudioParam" , js_name = value ) ]
    #[doc = "Getter for the `value` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioParam/value)\n\n*This API requires the following crate features to be activated: `AudioParam`*"]
    pub fn value(this: &AudioParam) -> f32;
    # [ wasm_bindgen ( structural , method , setter , js_class = "AudioParam" , js_name = value ) ]
    #[doc = "Setter for the `value` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioParam/value)\n\n*This API requires the following crate features to be activated: `AudioParam`*"]
    pub fn set_value(this: &AudioParam, value: f32);
    # [ wasm_bindgen ( structural , method , getter , js_class = "AudioParam" , js_name = defaultValue ) ]
    #[doc = "Getter for the `defaultValue` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioParam/defaultValue)\n\n*This API requires the following crate features to be activated: `AudioParam`*"]
    pub fn default_value(this: &AudioParam) -> f32;
    # [ wasm_bindgen ( structural , method , getter , js_class = "AudioParam" , js_name = minValue ) ]
    #[doc = "Getter for the `minValue` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioParam/minValue)\n\n*This API requires the following crate features to be activated: `AudioParam`*"]
    pub fn min_value(this: &AudioParam) -> f32;
    # [ wasm_bindgen ( structural , method , getter , js_class = "AudioParam" , js_name = maxValue ) ]
    #[doc = "Getter for the `maxValue` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioParam/maxValue)\n\n*This API requires the following crate features to be activated: `AudioParam`*"]
    pub fn max_value(this: &AudioParam) -> f32;
    # [ wasm_bindgen ( catch , method , structural , js_class = "AudioParam" , js_name = cancelScheduledValues ) ]
    #[doc = "The `cancelScheduledValues()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioParam/cancelScheduledValues)\n\n*This API requires the following crate features to be activated: `AudioParam`*"]
    pub fn cancel_scheduled_values(
        this: &AudioParam,
        start_time: f64,
    ) -> Result<AudioParam, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_class = "AudioParam" , js_name = exponentialRampToValueAtTime ) ]
    #[doc = "The `exponentialRampToValueAtTime()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioParam/exponentialRampToValueAtTime)\n\n*This API requires the following crate features to be activated: `AudioParam`*"]
    pub fn exponential_ramp_to_value_at_time(
        this: &AudioParam,
        value: f32,
        end_time: f64,
    ) -> Result<AudioParam, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_class = "AudioParam" , js_name = linearRampToValueAtTime ) ]
    #[doc = "The `linearRampToValueAtTime()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioParam/linearRampToValueAtTime)\n\n*This API requires the following crate features to be activated: `AudioParam`*"]
    pub fn linear_ramp_to_value_at_time(
        this: &AudioParam,
        value: f32,
        end_time: f64,
    ) -> Result<AudioParam, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_class = "AudioParam" , js_name = setTargetAtTime ) ]
    #[doc = "The `setTargetAtTime()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioParam/setTargetAtTime)\n\n*This API requires the following crate features to be activated: `AudioParam`*"]
    pub fn set_target_at_time(
        this: &AudioParam,
        target: f32,
        start_time: f64,
        time_constant: f64,
    ) -> Result<AudioParam, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_class = "AudioParam" , js_name = setValueAtTime ) ]
    #[doc = "The `setValueAtTime()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioParam/setValueAtTime)\n\n*This API requires the following crate features to be activated: `AudioParam`*"]
    pub fn set_value_at_time(
        this: &AudioParam,
        value: f32,
        start_time: f64,
    ) -> Result<AudioParam, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_class = "AudioParam" , js_name = setValueCurveAtTime ) ]
    #[doc = "The `setValueCurveAtTime()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioParam/setValueCurveAtTime)\n\n*This API requires the following crate features to be activated: `AudioParam`*"]
    pub fn set_value_curve_at_time(
        this: &AudioParam,
        values: &mut [f32],
        start_time: f64,
        duration: f64,
    ) -> Result<AudioParam, JsValue>;
}
