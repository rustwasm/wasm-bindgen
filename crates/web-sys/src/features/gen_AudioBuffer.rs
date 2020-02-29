use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = AudioBuffer , typescript_name = AudioBuffer ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `AudioBuffer` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioBuffer)
    ///
    ///*This API requires the following crate features to be activated: `AudioBuffer`*
    pub type AudioBuffer;

    # [ wasm_bindgen ( structural , method , getter , js_class = "AudioBuffer" , js_name = sampleRate ) ]
    ///Getter for the `sampleRate` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioBuffer/sampleRate)
    ///
    ///*This API requires the following crate features to be activated: `AudioBuffer`*
    pub fn sample_rate(this: &AudioBuffer) -> f32;

    # [ wasm_bindgen ( structural , method , getter , js_class = "AudioBuffer" , js_name = length ) ]
    ///Getter for the `length` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioBuffer/length)
    ///
    ///*This API requires the following crate features to be activated: `AudioBuffer`*
    pub fn length(this: &AudioBuffer) -> u32;

    # [ wasm_bindgen ( structural , method , getter , js_class = "AudioBuffer" , js_name = duration ) ]
    ///Getter for the `duration` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioBuffer/duration)
    ///
    ///*This API requires the following crate features to be activated: `AudioBuffer`*
    pub fn duration(this: &AudioBuffer) -> f64;

    # [ wasm_bindgen ( structural , method , getter , js_class = "AudioBuffer" , js_name = numberOfChannels ) ]
    ///Getter for the `numberOfChannels` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioBuffer/numberOfChannels)
    ///
    ///*This API requires the following crate features to be activated: `AudioBuffer`*
    pub fn number_of_channels(this: &AudioBuffer) -> u32;

    #[cfg(feature = "AudioBufferOptions")]
    #[wasm_bindgen(catch, constructor, js_class = "AudioBuffer")]
    ///The `new AudioBuffer(..)` constructor, creating a new instance of `AudioBuffer`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioBuffer/AudioBuffer)
    ///
    ///*This API requires the following crate features to be activated: `AudioBuffer`, `AudioBufferOptions`*
    pub fn new(options: &AudioBufferOptions) -> Result<AudioBuffer, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "AudioBuffer" , js_name = copyFromChannel ) ]
    ///The `copyFromChannel()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioBuffer/copyFromChannel)
    ///
    ///*This API requires the following crate features to be activated: `AudioBuffer`*
    pub fn copy_from_channel(
        this: &AudioBuffer,
        destination: &mut [f32],
        channel_number: i32,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "AudioBuffer" , js_name = copyFromChannel ) ]
    ///The `copyFromChannel()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioBuffer/copyFromChannel)
    ///
    ///*This API requires the following crate features to be activated: `AudioBuffer`*
    pub fn copy_from_channel_with_start_in_channel(
        this: &AudioBuffer,
        destination: &mut [f32],
        channel_number: i32,
        start_in_channel: u32,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "AudioBuffer" , js_name = copyToChannel ) ]
    ///The `copyToChannel()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioBuffer/copyToChannel)
    ///
    ///*This API requires the following crate features to be activated: `AudioBuffer`*
    pub fn copy_to_channel(
        this: &AudioBuffer,
        source: &mut [f32],
        channel_number: i32,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "AudioBuffer" , js_name = copyToChannel ) ]
    ///The `copyToChannel()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioBuffer/copyToChannel)
    ///
    ///*This API requires the following crate features to be activated: `AudioBuffer`*
    pub fn copy_to_channel_with_start_in_channel(
        this: &AudioBuffer,
        source: &mut [f32],
        channel_number: i32,
        start_in_channel: u32,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "AudioBuffer" , js_name = getChannelData ) ]
    ///The `getChannelData()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioBuffer/getChannelData)
    ///
    ///*This API requires the following crate features to be activated: `AudioBuffer`*
    pub fn get_channel_data(this: &AudioBuffer, channel: u32) -> Result<Vec<f32>, JsValue>;

}
