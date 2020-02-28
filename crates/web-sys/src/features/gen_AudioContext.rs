use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( vendor_prefix = webkit , extends = BaseAudioContext , extends = EventTarget , extends = :: js_sys :: Object , js_name = AudioContext , typescript_name = AudioContext ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AudioContext` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext)\n\n*This API requires the following crate features to be activated: `AudioContext`*"]
    pub type AudioContext;
    # [ wasm_bindgen ( structural , method , getter , js_name = destination ) ]
    #[cfg(feature = "AudioDestinationNode")]
    #[doc = "Getter for the `destination` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/destination)\n\n*This API requires the following crate features to be activated: `AudioContext`, `AudioDestinationNode`*"]
    pub fn destination(this: &AudioContext) -> AudioDestinationNode;
    # [ wasm_bindgen ( structural , method , getter , js_name = sampleRate ) ]
    #[doc = "Getter for the `sampleRate` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/sampleRate)\n\n*This API requires the following crate features to be activated: `AudioContext`*"]
    pub fn sample_rate(this: &AudioContext) -> f32;
    # [ wasm_bindgen ( structural , method , getter , js_name = currentTime ) ]
    #[doc = "Getter for the `currentTime` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/currentTime)\n\n*This API requires the following crate features to be activated: `AudioContext`*"]
    pub fn current_time(this: &AudioContext) -> f64;
    # [ wasm_bindgen ( structural , method , getter , js_name = listener ) ]
    #[cfg(feature = "AudioListener")]
    #[doc = "Getter for the `listener` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/listener)\n\n*This API requires the following crate features to be activated: `AudioContext`, `AudioListener`*"]
    pub fn listener(this: &AudioContext) -> AudioListener;
    # [ wasm_bindgen ( structural , method , getter , js_name = state ) ]
    #[cfg(feature = "AudioContextState")]
    #[doc = "Getter for the `state` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/state)\n\n*This API requires the following crate features to be activated: `AudioContext`, `AudioContextState`*"]
    pub fn state(this: &AudioContext) -> AudioContextState;
    # [ wasm_bindgen ( structural , catch , method , getter , js_name = audioWorklet ) ]
    #[cfg(feature = "AudioWorklet")]
    #[doc = "Getter for the `audioWorklet` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/audioWorklet)\n\n*This API requires the following crate features to be activated: `AudioContext`, `AudioWorklet`*"]
    pub fn audio_worklet(this: &AudioContext) -> Result<AudioWorklet, JsValue>;
    # [ wasm_bindgen ( structural , method , getter , js_name = onstatechange ) ]
    #[doc = "Getter for the `onstatechange` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/onstatechange)\n\n*This API requires the following crate features to be activated: `AudioContext`*"]
    pub fn onstatechange(this: &AudioContext) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onstatechange ) ]
    #[doc = "Setter for the `onstatechange` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/onstatechange)\n\n*This API requires the following crate features to be activated: `AudioContext`*"]
    pub fn set_onstatechange(this: &AudioContext, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new AudioContext(..)` constructor, creating a new instance of `AudioContext`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/AudioContext)\n\n*This API requires the following crate features to be activated: `AudioContext`*"]
    pub fn new(this: &AudioContext) -> Result<AudioContext, JsValue>;
    #[cfg(feature = "AudioContextOptions")]
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new AudioContext(..)` constructor, creating a new instance of `AudioContext`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/AudioContext)\n\n*This API requires the following crate features to be activated: `AudioContext`, `AudioContextOptions`*"]
    pub fn new_with_context_options(
        this: &AudioContext,
        context_options: &AudioContextOptions,
    ) -> Result<AudioContext, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = close ) ]
    #[doc = "The `close()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/close)\n\n*This API requires the following crate features to be activated: `AudioContext`*"]
    pub fn close(this: &AudioContext) -> Result<::js_sys::Promise, JsValue>;
    #[cfg(all(feature = "HtmlMediaElement", feature = "MediaElementAudioSourceNode",))]
    # [ wasm_bindgen ( catch , method , structural , js_name = createMediaElementSource ) ]
    #[doc = "The `createMediaElementSource()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/createMediaElementSource)\n\n*This API requires the following crate features to be activated: `AudioContext`, `HtmlMediaElement`, `MediaElementAudioSourceNode`*"]
    pub fn create_media_element_source(
        this: &AudioContext,
        media_element: &HtmlMediaElement,
    ) -> Result<MediaElementAudioSourceNode, JsValue>;
    #[cfg(feature = "MediaStreamAudioDestinationNode")]
    # [ wasm_bindgen ( catch , method , structural , js_name = createMediaStreamDestination ) ]
    #[doc = "The `createMediaStreamDestination()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/createMediaStreamDestination)\n\n*This API requires the following crate features to be activated: `AudioContext`, `MediaStreamAudioDestinationNode`*"]
    pub fn create_media_stream_destination(
        this: &AudioContext,
    ) -> Result<MediaStreamAudioDestinationNode, JsValue>;
    #[cfg(all(feature = "MediaStream", feature = "MediaStreamAudioSourceNode",))]
    # [ wasm_bindgen ( catch , method , structural , js_name = createMediaStreamSource ) ]
    #[doc = "The `createMediaStreamSource()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/createMediaStreamSource)\n\n*This API requires the following crate features to be activated: `AudioContext`, `MediaStream`, `MediaStreamAudioSourceNode`*"]
    pub fn create_media_stream_source(
        this: &AudioContext,
        media_stream: &MediaStream,
    ) -> Result<MediaStreamAudioSourceNode, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = suspend ) ]
    #[doc = "The `suspend()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/suspend)\n\n*This API requires the following crate features to be activated: `AudioContext`*"]
    pub fn suspend(this: &AudioContext) -> Result<::js_sys::Promise, JsValue>;
    #[cfg(feature = "AnalyserNode")]
    # [ wasm_bindgen ( catch , method , structural , js_name = createAnalyser ) ]
    #[doc = "The `createAnalyser()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/createAnalyser)\n\n*This API requires the following crate features to be activated: `AnalyserNode`, `AudioContext`*"]
    pub fn create_analyser(this: &AudioContext) -> Result<AnalyserNode, JsValue>;
    #[cfg(feature = "BiquadFilterNode")]
    # [ wasm_bindgen ( catch , method , structural , js_name = createBiquadFilter ) ]
    #[doc = "The `createBiquadFilter()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/createBiquadFilter)\n\n*This API requires the following crate features to be activated: `AudioContext`, `BiquadFilterNode`*"]
    pub fn create_biquad_filter(this: &AudioContext) -> Result<BiquadFilterNode, JsValue>;
    #[cfg(feature = "AudioBuffer")]
    # [ wasm_bindgen ( catch , method , structural , js_name = createBuffer ) ]
    #[doc = "The `createBuffer()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/createBuffer)\n\n*This API requires the following crate features to be activated: `AudioBuffer`, `AudioContext`*"]
    pub fn create_buffer(
        this: &AudioContext,
        number_of_channels: u32,
        length: u32,
        sample_rate: f32,
    ) -> Result<AudioBuffer, JsValue>;
    #[cfg(feature = "AudioBufferSourceNode")]
    # [ wasm_bindgen ( catch , method , structural , js_name = createBufferSource ) ]
    #[doc = "The `createBufferSource()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/createBufferSource)\n\n*This API requires the following crate features to be activated: `AudioBufferSourceNode`, `AudioContext`*"]
    pub fn create_buffer_source(this: &AudioContext) -> Result<AudioBufferSourceNode, JsValue>;
    #[cfg(feature = "ChannelMergerNode")]
    # [ wasm_bindgen ( catch , method , structural , js_name = createChannelMerger ) ]
    #[doc = "The `createChannelMerger()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/createChannelMerger)\n\n*This API requires the following crate features to be activated: `AudioContext`, `ChannelMergerNode`*"]
    pub fn create_channel_merger(this: &AudioContext) -> Result<ChannelMergerNode, JsValue>;
    #[cfg(feature = "ChannelMergerNode")]
    # [ wasm_bindgen ( catch , method , structural , js_name = createChannelMerger ) ]
    #[doc = "The `createChannelMerger()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/createChannelMerger)\n\n*This API requires the following crate features to be activated: `AudioContext`, `ChannelMergerNode`*"]
    pub fn create_channel_merger_with_number_of_inputs(
        this: &AudioContext,
        number_of_inputs: u32,
    ) -> Result<ChannelMergerNode, JsValue>;
    #[cfg(feature = "ChannelSplitterNode")]
    # [ wasm_bindgen ( catch , method , structural , js_name = createChannelSplitter ) ]
    #[doc = "The `createChannelSplitter()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/createChannelSplitter)\n\n*This API requires the following crate features to be activated: `AudioContext`, `ChannelSplitterNode`*"]
    pub fn create_channel_splitter(this: &AudioContext) -> Result<ChannelSplitterNode, JsValue>;
    #[cfg(feature = "ChannelSplitterNode")]
    # [ wasm_bindgen ( catch , method , structural , js_name = createChannelSplitter ) ]
    #[doc = "The `createChannelSplitter()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/createChannelSplitter)\n\n*This API requires the following crate features to be activated: `AudioContext`, `ChannelSplitterNode`*"]
    pub fn create_channel_splitter_with_number_of_outputs(
        this: &AudioContext,
        number_of_outputs: u32,
    ) -> Result<ChannelSplitterNode, JsValue>;
    #[cfg(feature = "ConstantSourceNode")]
    # [ wasm_bindgen ( catch , method , structural , js_name = createConstantSource ) ]
    #[doc = "The `createConstantSource()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/createConstantSource)\n\n*This API requires the following crate features to be activated: `AudioContext`, `ConstantSourceNode`*"]
    pub fn create_constant_source(this: &AudioContext) -> Result<ConstantSourceNode, JsValue>;
    #[cfg(feature = "ConvolverNode")]
    # [ wasm_bindgen ( catch , method , structural , js_name = createConvolver ) ]
    #[doc = "The `createConvolver()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/createConvolver)\n\n*This API requires the following crate features to be activated: `AudioContext`, `ConvolverNode`*"]
    pub fn create_convolver(this: &AudioContext) -> Result<ConvolverNode, JsValue>;
    #[cfg(feature = "DelayNode")]
    # [ wasm_bindgen ( catch , method , structural , js_name = createDelay ) ]
    #[doc = "The `createDelay()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/createDelay)\n\n*This API requires the following crate features to be activated: `AudioContext`, `DelayNode`*"]
    pub fn create_delay(this: &AudioContext) -> Result<DelayNode, JsValue>;
    #[cfg(feature = "DelayNode")]
    # [ wasm_bindgen ( catch , method , structural , js_name = createDelay ) ]
    #[doc = "The `createDelay()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/createDelay)\n\n*This API requires the following crate features to be activated: `AudioContext`, `DelayNode`*"]
    pub fn create_delay_with_max_delay_time(
        this: &AudioContext,
        max_delay_time: f64,
    ) -> Result<DelayNode, JsValue>;
    #[cfg(feature = "DynamicsCompressorNode")]
    # [ wasm_bindgen ( catch , method , structural , js_name = createDynamicsCompressor ) ]
    #[doc = "The `createDynamicsCompressor()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/createDynamicsCompressor)\n\n*This API requires the following crate features to be activated: `AudioContext`, `DynamicsCompressorNode`*"]
    pub fn create_dynamics_compressor(
        this: &AudioContext,
    ) -> Result<DynamicsCompressorNode, JsValue>;
    #[cfg(feature = "GainNode")]
    # [ wasm_bindgen ( catch , method , structural , js_name = createGain ) ]
    #[doc = "The `createGain()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/createGain)\n\n*This API requires the following crate features to be activated: `AudioContext`, `GainNode`*"]
    pub fn create_gain(this: &AudioContext) -> Result<GainNode, JsValue>;
    #[cfg(feature = "IirFilterNode")]
    # [ wasm_bindgen ( catch , method , structural , js_name = createIIRFilter ) ]
    #[doc = "The `createIIRFilter()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/createIIRFilter)\n\n*This API requires the following crate features to be activated: `AudioContext`, `IirFilterNode`*"]
    pub fn create_iir_filter(
        this: &AudioContext,
        feedforward: &::wasm_bindgen::JsValue,
        feedback: &::wasm_bindgen::JsValue,
    ) -> Result<IirFilterNode, JsValue>;
    #[cfg(feature = "OscillatorNode")]
    # [ wasm_bindgen ( catch , method , structural , js_name = createOscillator ) ]
    #[doc = "The `createOscillator()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/createOscillator)\n\n*This API requires the following crate features to be activated: `AudioContext`, `OscillatorNode`*"]
    pub fn create_oscillator(this: &AudioContext) -> Result<OscillatorNode, JsValue>;
    #[cfg(feature = "PannerNode")]
    # [ wasm_bindgen ( catch , method , structural , js_name = createPanner ) ]
    #[doc = "The `createPanner()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/createPanner)\n\n*This API requires the following crate features to be activated: `AudioContext`, `PannerNode`*"]
    pub fn create_panner(this: &AudioContext) -> Result<PannerNode, JsValue>;
    #[cfg(feature = "PeriodicWave")]
    # [ wasm_bindgen ( catch , method , structural , js_name = createPeriodicWave ) ]
    #[doc = "The `createPeriodicWave()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/createPeriodicWave)\n\n*This API requires the following crate features to be activated: `AudioContext`, `PeriodicWave`*"]
    pub fn create_periodic_wave(
        this: &AudioContext,
        real: &mut [f32],
        imag: &mut [f32],
    ) -> Result<PeriodicWave, JsValue>;
    #[cfg(all(feature = "PeriodicWave", feature = "PeriodicWaveConstraints",))]
    # [ wasm_bindgen ( catch , method , structural , js_name = createPeriodicWave ) ]
    #[doc = "The `createPeriodicWave()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/createPeriodicWave)\n\n*This API requires the following crate features to be activated: `AudioContext`, `PeriodicWave`, `PeriodicWaveConstraints`*"]
    pub fn create_periodic_wave_with_constraints(
        this: &AudioContext,
        real: &mut [f32],
        imag: &mut [f32],
        constraints: &PeriodicWaveConstraints,
    ) -> Result<PeriodicWave, JsValue>;
    #[cfg(feature = "ScriptProcessorNode")]
    # [ wasm_bindgen ( catch , method , structural , js_name = createScriptProcessor ) ]
    #[doc = "The `createScriptProcessor()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/createScriptProcessor)\n\n*This API requires the following crate features to be activated: `AudioContext`, `ScriptProcessorNode`*"]
    pub fn create_script_processor(this: &AudioContext) -> Result<ScriptProcessorNode, JsValue>;
    #[cfg(feature = "ScriptProcessorNode")]
    # [ wasm_bindgen ( catch , method , structural , js_name = createScriptProcessor ) ]
    #[doc = "The `createScriptProcessor()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/createScriptProcessor)\n\n*This API requires the following crate features to be activated: `AudioContext`, `ScriptProcessorNode`*"]
    pub fn create_script_processor_with_buffer_size(
        this: &AudioContext,
        buffer_size: u32,
    ) -> Result<ScriptProcessorNode, JsValue>;
    #[cfg(feature = "ScriptProcessorNode")]
    # [ wasm_bindgen ( catch , method , structural , js_name = createScriptProcessor ) ]
    #[doc = "The `createScriptProcessor()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/createScriptProcessor)\n\n*This API requires the following crate features to be activated: `AudioContext`, `ScriptProcessorNode`*"]
    pub fn create_script_processor_with_buffer_size_and_number_of_input_channels(
        this: &AudioContext,
        buffer_size: u32,
        number_of_input_channels: u32,
    ) -> Result<ScriptProcessorNode, JsValue>;
    #[cfg(feature = "ScriptProcessorNode")]
    # [ wasm_bindgen ( catch , method , structural , js_name = createScriptProcessor ) ]
    #[doc = "The `createScriptProcessor()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/createScriptProcessor)\n\n*This API requires the following crate features to be activated: `AudioContext`, `ScriptProcessorNode`*"]
    pub fn create_script_processor_with_buffer_size_and_number_of_input_channels_and_number_of_output_channels(
        this: &AudioContext,
        buffer_size: u32,
        number_of_input_channels: u32,
        number_of_output_channels: u32,
    ) -> Result<ScriptProcessorNode, JsValue>;
    #[cfg(feature = "StereoPannerNode")]
    # [ wasm_bindgen ( catch , method , structural , js_name = createStereoPanner ) ]
    #[doc = "The `createStereoPanner()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/createStereoPanner)\n\n*This API requires the following crate features to be activated: `AudioContext`, `StereoPannerNode`*"]
    pub fn create_stereo_panner(this: &AudioContext) -> Result<StereoPannerNode, JsValue>;
    #[cfg(feature = "WaveShaperNode")]
    # [ wasm_bindgen ( catch , method , structural , js_name = createWaveShaper ) ]
    #[doc = "The `createWaveShaper()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/createWaveShaper)\n\n*This API requires the following crate features to be activated: `AudioContext`, `WaveShaperNode`*"]
    pub fn create_wave_shaper(this: &AudioContext) -> Result<WaveShaperNode, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = decodeAudioData ) ]
    #[doc = "The `decodeAudioData()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/decodeAudioData)\n\n*This API requires the following crate features to be activated: `AudioContext`*"]
    pub fn decode_audio_data(
        this: &AudioContext,
        audio_data: &::js_sys::ArrayBuffer,
    ) -> Result<::js_sys::Promise, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = decodeAudioData ) ]
    #[doc = "The `decodeAudioData()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/decodeAudioData)\n\n*This API requires the following crate features to be activated: `AudioContext`*"]
    pub fn decode_audio_data_with_success_callback(
        this: &AudioContext,
        audio_data: &::js_sys::ArrayBuffer,
        success_callback: &::js_sys::Function,
    ) -> Result<::js_sys::Promise, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = decodeAudioData ) ]
    #[doc = "The `decodeAudioData()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/decodeAudioData)\n\n*This API requires the following crate features to be activated: `AudioContext`*"]
    pub fn decode_audio_data_with_success_callback_and_error_callback(
        this: &AudioContext,
        audio_data: &::js_sys::ArrayBuffer,
        success_callback: &::js_sys::Function,
        error_callback: &::js_sys::Function,
    ) -> Result<::js_sys::Promise, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = resume ) ]
    #[doc = "The `resume()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/resume)\n\n*This API requires the following crate features to be activated: `AudioContext`*"]
    pub fn resume(this: &AudioContext) -> Result<::js_sys::Promise, JsValue>;
}
