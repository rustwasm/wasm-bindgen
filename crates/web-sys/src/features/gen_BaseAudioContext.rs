use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = EventTarget , extends = :: js_sys :: Object , js_name = BaseAudioContext , typescript_name = BaseAudioContext ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `BaseAudioContext` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BaseAudioContext)\n\n*This API requires the following crate features to be activated: `BaseAudioContext`*"]
    #[deprecated(note = "doesn't exist in Safari, use `AudioContext` instead now")]
    pub type BaseAudioContext;
    #[deprecated(note = "doesn't exist in Safari, use `AudioContext` instead now")]
    # [ wasm_bindgen ( structural , method , getter , js_name = destination ) ]
    #[cfg(feature = "AudioDestinationNode")]
    #[doc = "Getter for the `destination` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BaseAudioContext/destination)\n\n*This API requires the following crate features to be activated: `AudioDestinationNode`, `BaseAudioContext`*"]
    pub fn destination(this: &BaseAudioContext) -> AudioDestinationNode;
    #[deprecated(note = "doesn't exist in Safari, use `AudioContext` instead now")]
    # [ wasm_bindgen ( structural , method , getter , js_name = sampleRate ) ]
    #[doc = "Getter for the `sampleRate` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BaseAudioContext/sampleRate)\n\n*This API requires the following crate features to be activated: `BaseAudioContext`*"]
    pub fn sample_rate(this: &BaseAudioContext) -> f32;
    #[deprecated(note = "doesn't exist in Safari, use `AudioContext` instead now")]
    # [ wasm_bindgen ( structural , method , getter , js_name = currentTime ) ]
    #[doc = "Getter for the `currentTime` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BaseAudioContext/currentTime)\n\n*This API requires the following crate features to be activated: `BaseAudioContext`*"]
    pub fn current_time(this: &BaseAudioContext) -> f64;
    #[deprecated(note = "doesn't exist in Safari, use `AudioContext` instead now")]
    # [ wasm_bindgen ( structural , method , getter , js_name = listener ) ]
    #[cfg(feature = "AudioListener")]
    #[doc = "Getter for the `listener` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BaseAudioContext/listener)\n\n*This API requires the following crate features to be activated: `AudioListener`, `BaseAudioContext`*"]
    pub fn listener(this: &BaseAudioContext) -> AudioListener;
    #[deprecated(note = "doesn't exist in Safari, use `AudioContext` instead now")]
    # [ wasm_bindgen ( structural , method , getter , js_name = state ) ]
    #[cfg(feature = "AudioContextState")]
    #[doc = "Getter for the `state` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BaseAudioContext/state)\n\n*This API requires the following crate features to be activated: `AudioContextState`, `BaseAudioContext`*"]
    pub fn state(this: &BaseAudioContext) -> AudioContextState;
    #[deprecated(note = "doesn't exist in Safari, use `AudioContext` instead now")]
    # [ wasm_bindgen ( structural , catch , method , getter , js_name = audioWorklet ) ]
    #[cfg(feature = "AudioWorklet")]
    #[doc = "Getter for the `audioWorklet` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BaseAudioContext/audioWorklet)\n\n*This API requires the following crate features to be activated: `AudioWorklet`, `BaseAudioContext`*"]
    pub fn audio_worklet(this: &BaseAudioContext) -> Result<AudioWorklet, JsValue>;
    #[deprecated(note = "doesn't exist in Safari, use `AudioContext` instead now")]
    # [ wasm_bindgen ( structural , method , getter , js_name = onstatechange ) ]
    #[doc = "Getter for the `onstatechange` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BaseAudioContext/onstatechange)\n\n*This API requires the following crate features to be activated: `BaseAudioContext`*"]
    pub fn onstatechange(this: &BaseAudioContext) -> Option<::js_sys::Function>;
    #[deprecated(note = "doesn't exist in Safari, use `AudioContext` instead now")]
    # [ wasm_bindgen ( structural , method , setter , js_name = onstatechange ) ]
    #[doc = "Setter for the `onstatechange` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BaseAudioContext/onstatechange)\n\n*This API requires the following crate features to be activated: `BaseAudioContext`*"]
    pub fn set_onstatechange(this: &BaseAudioContext, value: Option<&::js_sys::Function>);
    #[deprecated(note = "doesn't exist in Safari, use `AudioContext` instead now")]
    #[cfg(feature = "AnalyserNode")]
    # [ wasm_bindgen ( catch , method , structural , js_name = createAnalyser ) ]
    #[doc = "The `createAnalyser()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BaseAudioContext/createAnalyser)\n\n*This API requires the following crate features to be activated: `AnalyserNode`, `BaseAudioContext`*"]
    pub fn create_analyser(this: &BaseAudioContext) -> Result<AnalyserNode, JsValue>;
    #[deprecated(note = "doesn't exist in Safari, use `AudioContext` instead now")]
    #[cfg(feature = "BiquadFilterNode")]
    # [ wasm_bindgen ( catch , method , structural , js_name = createBiquadFilter ) ]
    #[doc = "The `createBiquadFilter()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BaseAudioContext/createBiquadFilter)\n\n*This API requires the following crate features to be activated: `BaseAudioContext`, `BiquadFilterNode`*"]
    pub fn create_biquad_filter(this: &BaseAudioContext) -> Result<BiquadFilterNode, JsValue>;
    #[deprecated(note = "doesn't exist in Safari, use `AudioContext` instead now")]
    #[cfg(feature = "AudioBuffer")]
    # [ wasm_bindgen ( catch , method , structural , js_name = createBuffer ) ]
    #[doc = "The `createBuffer()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BaseAudioContext/createBuffer)\n\n*This API requires the following crate features to be activated: `AudioBuffer`, `BaseAudioContext`*"]
    pub fn create_buffer(
        this: &BaseAudioContext,
        number_of_channels: u32,
        length: u32,
        sample_rate: f32,
    ) -> Result<AudioBuffer, JsValue>;
    #[deprecated(note = "doesn't exist in Safari, use `AudioContext` instead now")]
    #[cfg(feature = "AudioBufferSourceNode")]
    # [ wasm_bindgen ( catch , method , structural , js_name = createBufferSource ) ]
    #[doc = "The `createBufferSource()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BaseAudioContext/createBufferSource)\n\n*This API requires the following crate features to be activated: `AudioBufferSourceNode`, `BaseAudioContext`*"]
    pub fn create_buffer_source(this: &BaseAudioContext) -> Result<AudioBufferSourceNode, JsValue>;
    #[deprecated(note = "doesn't exist in Safari, use `AudioContext` instead now")]
    #[cfg(feature = "ChannelMergerNode")]
    # [ wasm_bindgen ( catch , method , structural , js_name = createChannelMerger ) ]
    #[doc = "The `createChannelMerger()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BaseAudioContext/createChannelMerger)\n\n*This API requires the following crate features to be activated: `BaseAudioContext`, `ChannelMergerNode`*"]
    pub fn create_channel_merger(this: &BaseAudioContext) -> Result<ChannelMergerNode, JsValue>;
    #[deprecated(note = "doesn't exist in Safari, use `AudioContext` instead now")]
    #[cfg(feature = "ChannelMergerNode")]
    # [ wasm_bindgen ( catch , method , structural , js_name = createChannelMerger ) ]
    #[doc = "The `createChannelMerger()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BaseAudioContext/createChannelMerger)\n\n*This API requires the following crate features to be activated: `BaseAudioContext`, `ChannelMergerNode`*"]
    pub fn create_channel_merger_with_number_of_inputs(
        this: &BaseAudioContext,
        number_of_inputs: u32,
    ) -> Result<ChannelMergerNode, JsValue>;
    #[deprecated(note = "doesn't exist in Safari, use `AudioContext` instead now")]
    #[cfg(feature = "ChannelSplitterNode")]
    # [ wasm_bindgen ( catch , method , structural , js_name = createChannelSplitter ) ]
    #[doc = "The `createChannelSplitter()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BaseAudioContext/createChannelSplitter)\n\n*This API requires the following crate features to be activated: `BaseAudioContext`, `ChannelSplitterNode`*"]
    pub fn create_channel_splitter(this: &BaseAudioContext)
        -> Result<ChannelSplitterNode, JsValue>;
    #[deprecated(note = "doesn't exist in Safari, use `AudioContext` instead now")]
    #[cfg(feature = "ChannelSplitterNode")]
    # [ wasm_bindgen ( catch , method , structural , js_name = createChannelSplitter ) ]
    #[doc = "The `createChannelSplitter()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BaseAudioContext/createChannelSplitter)\n\n*This API requires the following crate features to be activated: `BaseAudioContext`, `ChannelSplitterNode`*"]
    pub fn create_channel_splitter_with_number_of_outputs(
        this: &BaseAudioContext,
        number_of_outputs: u32,
    ) -> Result<ChannelSplitterNode, JsValue>;
    #[deprecated(note = "doesn't exist in Safari, use `AudioContext` instead now")]
    #[cfg(feature = "ConstantSourceNode")]
    # [ wasm_bindgen ( catch , method , structural , js_name = createConstantSource ) ]
    #[doc = "The `createConstantSource()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BaseAudioContext/createConstantSource)\n\n*This API requires the following crate features to be activated: `BaseAudioContext`, `ConstantSourceNode`*"]
    pub fn create_constant_source(this: &BaseAudioContext) -> Result<ConstantSourceNode, JsValue>;
    #[deprecated(note = "doesn't exist in Safari, use `AudioContext` instead now")]
    #[cfg(feature = "ConvolverNode")]
    # [ wasm_bindgen ( catch , method , structural , js_name = createConvolver ) ]
    #[doc = "The `createConvolver()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BaseAudioContext/createConvolver)\n\n*This API requires the following crate features to be activated: `BaseAudioContext`, `ConvolverNode`*"]
    pub fn create_convolver(this: &BaseAudioContext) -> Result<ConvolverNode, JsValue>;
    #[deprecated(note = "doesn't exist in Safari, use `AudioContext` instead now")]
    #[cfg(feature = "DelayNode")]
    # [ wasm_bindgen ( catch , method , structural , js_name = createDelay ) ]
    #[doc = "The `createDelay()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BaseAudioContext/createDelay)\n\n*This API requires the following crate features to be activated: `BaseAudioContext`, `DelayNode`*"]
    pub fn create_delay(this: &BaseAudioContext) -> Result<DelayNode, JsValue>;
    #[deprecated(note = "doesn't exist in Safari, use `AudioContext` instead now")]
    #[cfg(feature = "DelayNode")]
    # [ wasm_bindgen ( catch , method , structural , js_name = createDelay ) ]
    #[doc = "The `createDelay()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BaseAudioContext/createDelay)\n\n*This API requires the following crate features to be activated: `BaseAudioContext`, `DelayNode`*"]
    pub fn create_delay_with_max_delay_time(
        this: &BaseAudioContext,
        max_delay_time: f64,
    ) -> Result<DelayNode, JsValue>;
    #[deprecated(note = "doesn't exist in Safari, use `AudioContext` instead now")]
    #[cfg(feature = "DynamicsCompressorNode")]
    # [ wasm_bindgen ( catch , method , structural , js_name = createDynamicsCompressor ) ]
    #[doc = "The `createDynamicsCompressor()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BaseAudioContext/createDynamicsCompressor)\n\n*This API requires the following crate features to be activated: `BaseAudioContext`, `DynamicsCompressorNode`*"]
    pub fn create_dynamics_compressor(
        this: &BaseAudioContext,
    ) -> Result<DynamicsCompressorNode, JsValue>;
    #[deprecated(note = "doesn't exist in Safari, use `AudioContext` instead now")]
    #[cfg(feature = "GainNode")]
    # [ wasm_bindgen ( catch , method , structural , js_name = createGain ) ]
    #[doc = "The `createGain()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BaseAudioContext/createGain)\n\n*This API requires the following crate features to be activated: `BaseAudioContext`, `GainNode`*"]
    pub fn create_gain(this: &BaseAudioContext) -> Result<GainNode, JsValue>;
    #[deprecated(note = "doesn't exist in Safari, use `AudioContext` instead now")]
    #[cfg(feature = "IirFilterNode")]
    # [ wasm_bindgen ( catch , method , structural , js_name = createIIRFilter ) ]
    #[doc = "The `createIIRFilter()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BaseAudioContext/createIIRFilter)\n\n*This API requires the following crate features to be activated: `BaseAudioContext`, `IirFilterNode`*"]
    pub fn create_iir_filter(
        this: &BaseAudioContext,
        feedforward: &::wasm_bindgen::JsValue,
        feedback: &::wasm_bindgen::JsValue,
    ) -> Result<IirFilterNode, JsValue>;
    #[deprecated(note = "doesn't exist in Safari, use `AudioContext` instead now")]
    #[cfg(feature = "OscillatorNode")]
    # [ wasm_bindgen ( catch , method , structural , js_name = createOscillator ) ]
    #[doc = "The `createOscillator()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BaseAudioContext/createOscillator)\n\n*This API requires the following crate features to be activated: `BaseAudioContext`, `OscillatorNode`*"]
    pub fn create_oscillator(this: &BaseAudioContext) -> Result<OscillatorNode, JsValue>;
    #[deprecated(note = "doesn't exist in Safari, use `AudioContext` instead now")]
    #[cfg(feature = "PannerNode")]
    # [ wasm_bindgen ( catch , method , structural , js_name = createPanner ) ]
    #[doc = "The `createPanner()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BaseAudioContext/createPanner)\n\n*This API requires the following crate features to be activated: `BaseAudioContext`, `PannerNode`*"]
    pub fn create_panner(this: &BaseAudioContext) -> Result<PannerNode, JsValue>;
    #[deprecated(note = "doesn't exist in Safari, use `AudioContext` instead now")]
    #[cfg(feature = "PeriodicWave")]
    # [ wasm_bindgen ( catch , method , structural , js_name = createPeriodicWave ) ]
    #[doc = "The `createPeriodicWave()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BaseAudioContext/createPeriodicWave)\n\n*This API requires the following crate features to be activated: `BaseAudioContext`, `PeriodicWave`*"]
    pub fn create_periodic_wave(
        this: &BaseAudioContext,
        real: &mut [f32],
        imag: &mut [f32],
    ) -> Result<PeriodicWave, JsValue>;
    #[deprecated(note = "doesn't exist in Safari, use `AudioContext` instead now")]
    #[cfg(all(feature = "PeriodicWave", feature = "PeriodicWaveConstraints",))]
    # [ wasm_bindgen ( catch , method , structural , js_name = createPeriodicWave ) ]
    #[doc = "The `createPeriodicWave()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BaseAudioContext/createPeriodicWave)\n\n*This API requires the following crate features to be activated: `BaseAudioContext`, `PeriodicWave`, `PeriodicWaveConstraints`*"]
    pub fn create_periodic_wave_with_constraints(
        this: &BaseAudioContext,
        real: &mut [f32],
        imag: &mut [f32],
        constraints: &PeriodicWaveConstraints,
    ) -> Result<PeriodicWave, JsValue>;
    #[deprecated(note = "doesn't exist in Safari, use `AudioContext` instead now")]
    #[cfg(feature = "ScriptProcessorNode")]
    # [ wasm_bindgen ( catch , method , structural , js_name = createScriptProcessor ) ]
    #[doc = "The `createScriptProcessor()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BaseAudioContext/createScriptProcessor)\n\n*This API requires the following crate features to be activated: `BaseAudioContext`, `ScriptProcessorNode`*"]
    pub fn create_script_processor(this: &BaseAudioContext)
        -> Result<ScriptProcessorNode, JsValue>;
    #[deprecated(note = "doesn't exist in Safari, use `AudioContext` instead now")]
    #[cfg(feature = "ScriptProcessorNode")]
    # [ wasm_bindgen ( catch , method , structural , js_name = createScriptProcessor ) ]
    #[doc = "The `createScriptProcessor()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BaseAudioContext/createScriptProcessor)\n\n*This API requires the following crate features to be activated: `BaseAudioContext`, `ScriptProcessorNode`*"]
    pub fn create_script_processor_with_buffer_size(
        this: &BaseAudioContext,
        buffer_size: u32,
    ) -> Result<ScriptProcessorNode, JsValue>;
    #[deprecated(note = "doesn't exist in Safari, use `AudioContext` instead now")]
    #[cfg(feature = "ScriptProcessorNode")]
    # [ wasm_bindgen ( catch , method , structural , js_name = createScriptProcessor ) ]
    #[doc = "The `createScriptProcessor()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BaseAudioContext/createScriptProcessor)\n\n*This API requires the following crate features to be activated: `BaseAudioContext`, `ScriptProcessorNode`*"]
    pub fn create_script_processor_with_buffer_size_and_number_of_input_channels(
        this: &BaseAudioContext,
        buffer_size: u32,
        number_of_input_channels: u32,
    ) -> Result<ScriptProcessorNode, JsValue>;
    #[deprecated(note = "doesn't exist in Safari, use `AudioContext` instead now")]
    #[cfg(feature = "ScriptProcessorNode")]
    # [ wasm_bindgen ( catch , method , structural , js_name = createScriptProcessor ) ]
    #[doc = "The `createScriptProcessor()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BaseAudioContext/createScriptProcessor)\n\n*This API requires the following crate features to be activated: `BaseAudioContext`, `ScriptProcessorNode`*"]
    pub fn create_script_processor_with_buffer_size_and_number_of_input_channels_and_number_of_output_channels(
        this: &BaseAudioContext,
        buffer_size: u32,
        number_of_input_channels: u32,
        number_of_output_channels: u32,
    ) -> Result<ScriptProcessorNode, JsValue>;
    #[deprecated(note = "doesn't exist in Safari, use `AudioContext` instead now")]
    #[cfg(feature = "StereoPannerNode")]
    # [ wasm_bindgen ( catch , method , structural , js_name = createStereoPanner ) ]
    #[doc = "The `createStereoPanner()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BaseAudioContext/createStereoPanner)\n\n*This API requires the following crate features to be activated: `BaseAudioContext`, `StereoPannerNode`*"]
    pub fn create_stereo_panner(this: &BaseAudioContext) -> Result<StereoPannerNode, JsValue>;
    #[deprecated(note = "doesn't exist in Safari, use `AudioContext` instead now")]
    #[cfg(feature = "WaveShaperNode")]
    # [ wasm_bindgen ( catch , method , structural , js_name = createWaveShaper ) ]
    #[doc = "The `createWaveShaper()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BaseAudioContext/createWaveShaper)\n\n*This API requires the following crate features to be activated: `BaseAudioContext`, `WaveShaperNode`*"]
    pub fn create_wave_shaper(this: &BaseAudioContext) -> Result<WaveShaperNode, JsValue>;
    #[deprecated(note = "doesn't exist in Safari, use `AudioContext` instead now")]
    # [ wasm_bindgen ( catch , method , structural , js_name = decodeAudioData ) ]
    #[doc = "The `decodeAudioData()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BaseAudioContext/decodeAudioData)\n\n*This API requires the following crate features to be activated: `BaseAudioContext`*"]
    pub fn decode_audio_data(
        this: &BaseAudioContext,
        audio_data: &::js_sys::ArrayBuffer,
    ) -> Result<::js_sys::Promise, JsValue>;
    #[deprecated(note = "doesn't exist in Safari, use `AudioContext` instead now")]
    # [ wasm_bindgen ( catch , method , structural , js_name = decodeAudioData ) ]
    #[doc = "The `decodeAudioData()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BaseAudioContext/decodeAudioData)\n\n*This API requires the following crate features to be activated: `BaseAudioContext`*"]
    pub fn decode_audio_data_with_success_callback(
        this: &BaseAudioContext,
        audio_data: &::js_sys::ArrayBuffer,
        success_callback: &::js_sys::Function,
    ) -> Result<::js_sys::Promise, JsValue>;
    #[deprecated(note = "doesn't exist in Safari, use `AudioContext` instead now")]
    # [ wasm_bindgen ( catch , method , structural , js_name = decodeAudioData ) ]
    #[doc = "The `decodeAudioData()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BaseAudioContext/decodeAudioData)\n\n*This API requires the following crate features to be activated: `BaseAudioContext`*"]
    pub fn decode_audio_data_with_success_callback_and_error_callback(
        this: &BaseAudioContext,
        audio_data: &::js_sys::ArrayBuffer,
        success_callback: &::js_sys::Function,
        error_callback: &::js_sys::Function,
    ) -> Result<::js_sys::Promise, JsValue>;
    #[deprecated(note = "doesn't exist in Safari, use `AudioContext` instead now")]
    # [ wasm_bindgen ( catch , method , structural , js_name = resume ) ]
    #[doc = "The `resume()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BaseAudioContext/resume)\n\n*This API requires the following crate features to be activated: `BaseAudioContext`*"]
    pub fn resume(this: &BaseAudioContext) -> Result<::js_sys::Promise, JsValue>;
}
