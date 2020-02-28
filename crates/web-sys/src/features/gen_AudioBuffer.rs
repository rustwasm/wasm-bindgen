use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = AudioBuffer , typescript_name = AudioBuffer ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AudioBuffer` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioBuffer)\n\n*This API requires the following crate features to be activated: `AudioBuffer`*"]
    pub type AudioBuffer;
    # [ wasm_bindgen ( structural , method , getter , js_name = sampleRate ) ]
    #[doc = "Getter for the `sampleRate` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioBuffer/sampleRate)\n\n*This API requires the following crate features to be activated: `AudioBuffer`*"]
    pub fn sample_rate(this: &AudioBuffer) -> f32;
    # [ wasm_bindgen ( structural , method , getter , js_name = length ) ]
    #[doc = "Getter for the `length` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioBuffer/length)\n\n*This API requires the following crate features to be activated: `AudioBuffer`*"]
    pub fn length(this: &AudioBuffer) -> u32;
    # [ wasm_bindgen ( structural , method , getter , js_name = duration ) ]
    #[doc = "Getter for the `duration` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioBuffer/duration)\n\n*This API requires the following crate features to be activated: `AudioBuffer`*"]
    pub fn duration(this: &AudioBuffer) -> f64;
    # [ wasm_bindgen ( structural , method , getter , js_name = numberOfChannels ) ]
    #[doc = "Getter for the `numberOfChannels` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioBuffer/numberOfChannels)\n\n*This API requires the following crate features to be activated: `AudioBuffer`*"]
    pub fn number_of_channels(this: &AudioBuffer) -> u32;
    #[cfg(feature = "AudioBufferOptions")]
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new AudioBuffer(..)` constructor, creating a new instance of `AudioBuffer`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioBuffer/AudioBuffer)\n\n*This API requires the following crate features to be activated: `AudioBuffer`, `AudioBufferOptions`*"]
    pub fn new(this: &AudioBuffer, options: &AudioBufferOptions) -> Result<AudioBuffer, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = copyFromChannel ) ]
    #[doc = "The `copyFromChannel()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioBuffer/copyFromChannel)\n\n*This API requires the following crate features to be activated: `AudioBuffer`*"]
    pub fn copy_from_channel(
        this: &AudioBuffer,
        destination: &mut [f32],
        channel_number: i32,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = copyFromChannel ) ]
    #[doc = "The `copyFromChannel()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioBuffer/copyFromChannel)\n\n*This API requires the following crate features to be activated: `AudioBuffer`*"]
    pub fn copy_from_channel_with_start_in_channel(
        this: &AudioBuffer,
        destination: &mut [f32],
        channel_number: i32,
        start_in_channel: u32,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = copyToChannel ) ]
    #[doc = "The `copyToChannel()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioBuffer/copyToChannel)\n\n*This API requires the following crate features to be activated: `AudioBuffer`*"]
    pub fn copy_to_channel(
        this: &AudioBuffer,
        source: &mut [f32],
        channel_number: i32,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = copyToChannel ) ]
    #[doc = "The `copyToChannel()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioBuffer/copyToChannel)\n\n*This API requires the following crate features to be activated: `AudioBuffer`*"]
    pub fn copy_to_channel_with_start_in_channel(
        this: &AudioBuffer,
        source: &mut [f32],
        channel_number: i32,
        start_in_channel: u32,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = getChannelData ) ]
    #[doc = "The `getChannelData()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioBuffer/getChannelData)\n\n*This API requires the following crate features to be activated: `AudioBuffer`*"]
    pub fn get_channel_data(this: &AudioBuffer, channel: u32) -> Result<Vec<f32>, JsValue>;
}
