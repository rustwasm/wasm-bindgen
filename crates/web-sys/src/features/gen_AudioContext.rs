use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( vendor_prefix = webkit , extends = BaseAudioContext , extends = EventTarget , extends = :: js_sys :: Object , js_name = AudioContext , typescript_type = "AudioContext" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `AudioContext` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext)
    ///
    ///*This API requires the following crate features to be activated: `AudioContext`*
    pub type AudioContext;

    #[cfg(feature = "AudioDestinationNode")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "AudioContext" , js_name = destination ) ]
    ///Getter for the `destination` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/destination)
    ///
    ///*This API requires the following crate features to be activated: `AudioContext`, `AudioDestinationNode`*
    pub fn destination(this: &AudioContext) -> AudioDestinationNode;

    # [ wasm_bindgen ( structural , method , getter , js_class = "AudioContext" , js_name = sampleRate ) ]
    ///Getter for the `sampleRate` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/sampleRate)
    ///
    ///*This API requires the following crate features to be activated: `AudioContext`*
    pub fn sample_rate(this: &AudioContext) -> f32;

    # [ wasm_bindgen ( structural , method , getter , js_class = "AudioContext" , js_name = currentTime ) ]
    ///Getter for the `currentTime` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/currentTime)
    ///
    ///*This API requires the following crate features to be activated: `AudioContext`*
    pub fn current_time(this: &AudioContext) -> f64;

    #[cfg(feature = "AudioListener")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "AudioContext" , js_name = listener ) ]
    ///Getter for the `listener` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/listener)
    ///
    ///*This API requires the following crate features to be activated: `AudioContext`, `AudioListener`*
    pub fn listener(this: &AudioContext) -> AudioListener;

    #[cfg(feature = "AudioContextState")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "AudioContext" , js_name = state ) ]
    ///Getter for the `state` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/state)
    ///
    ///*This API requires the following crate features to be activated: `AudioContext`, `AudioContextState`*
    pub fn state(this: &AudioContext) -> AudioContextState;

    #[cfg(feature = "AudioWorklet")]
    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "AudioContext" , js_name = audioWorklet ) ]
    ///Getter for the `audioWorklet` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/audioWorklet)
    ///
    ///*This API requires the following crate features to be activated: `AudioContext`, `AudioWorklet`*
    pub fn audio_worklet(this: &AudioContext) -> Result<AudioWorklet, JsValue>;

    # [ wasm_bindgen ( structural , method , getter , js_class = "AudioContext" , js_name = onstatechange ) ]
    ///Getter for the `onstatechange` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/onstatechange)
    ///
    ///*This API requires the following crate features to be activated: `AudioContext`*
    pub fn onstatechange(this: &AudioContext) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "AudioContext" , js_name = onstatechange ) ]
    ///Setter for the `onstatechange` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/onstatechange)
    ///
    ///*This API requires the following crate features to be activated: `AudioContext`*
    pub fn set_onstatechange(this: &AudioContext, value: Option<&::js_sys::Function>);

    #[wasm_bindgen(catch, constructor, js_class = "AudioContext")]
    ///The `new AudioContext(..)` constructor, creating a new instance of `AudioContext`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/AudioContext)
    ///
    ///*This API requires the following crate features to be activated: `AudioContext`*
    pub fn new() -> Result<AudioContext, JsValue>;

    #[cfg(feature = "AudioContextOptions")]
    #[wasm_bindgen(catch, constructor, js_class = "AudioContext")]
    ///The `new AudioContext(..)` constructor, creating a new instance of `AudioContext`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/AudioContext)
    ///
    ///*This API requires the following crate features to be activated: `AudioContext`, `AudioContextOptions`*
    pub fn new_with_context_options(
        context_options: &AudioContextOptions,
    ) -> Result<AudioContext, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "AudioContext" , js_name = close ) ]
    ///The `close()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/close)
    ///
    ///*This API requires the following crate features to be activated: `AudioContext`*
    pub fn close(this: &AudioContext) -> Result<::js_sys::Promise, JsValue>;

    #[cfg(all(feature = "HtmlMediaElement", feature = "MediaElementAudioSourceNode",))]
    # [ wasm_bindgen ( catch , method , structural , js_class = "AudioContext" , js_name = createMediaElementSource ) ]
    ///The `createMediaElementSource()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/createMediaElementSource)
    ///
    ///*This API requires the following crate features to be activated: `AudioContext`, `HtmlMediaElement`, `MediaElementAudioSourceNode`*
    pub fn create_media_element_source(
        this: &AudioContext,
        media_element: &HtmlMediaElement,
    ) -> Result<MediaElementAudioSourceNode, JsValue>;

    #[cfg(feature = "MediaStreamAudioDestinationNode")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "AudioContext" , js_name = createMediaStreamDestination ) ]
    ///The `createMediaStreamDestination()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/createMediaStreamDestination)
    ///
    ///*This API requires the following crate features to be activated: `AudioContext`, `MediaStreamAudioDestinationNode`*
    pub fn create_media_stream_destination(
        this: &AudioContext,
    ) -> Result<MediaStreamAudioDestinationNode, JsValue>;

    #[cfg(all(feature = "MediaStream", feature = "MediaStreamAudioSourceNode",))]
    # [ wasm_bindgen ( catch , method , structural , js_class = "AudioContext" , js_name = createMediaStreamSource ) ]
    ///The `createMediaStreamSource()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/createMediaStreamSource)
    ///
    ///*This API requires the following crate features to be activated: `AudioContext`, `MediaStream`, `MediaStreamAudioSourceNode`*
    pub fn create_media_stream_source(
        this: &AudioContext,
        media_stream: &MediaStream,
    ) -> Result<MediaStreamAudioSourceNode, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "AudioContext" , js_name = suspend ) ]
    ///The `suspend()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/suspend)
    ///
    ///*This API requires the following crate features to be activated: `AudioContext`*
    pub fn suspend(this: &AudioContext) -> Result<::js_sys::Promise, JsValue>;

    #[cfg(feature = "AnalyserNode")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "AudioContext" , js_name = createAnalyser ) ]
    ///The `createAnalyser()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/createAnalyser)
    ///
    ///*This API requires the following crate features to be activated: `AnalyserNode`, `AudioContext`*
    pub fn create_analyser(this: &AudioContext) -> Result<AnalyserNode, JsValue>;

    #[cfg(feature = "BiquadFilterNode")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "AudioContext" , js_name = createBiquadFilter ) ]
    ///The `createBiquadFilter()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/createBiquadFilter)
    ///
    ///*This API requires the following crate features to be activated: `AudioContext`, `BiquadFilterNode`*
    pub fn create_biquad_filter(this: &AudioContext) -> Result<BiquadFilterNode, JsValue>;

    #[cfg(feature = "AudioBuffer")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "AudioContext" , js_name = createBuffer ) ]
    ///The `createBuffer()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/createBuffer)
    ///
    ///*This API requires the following crate features to be activated: `AudioBuffer`, `AudioContext`*
    pub fn create_buffer(
        this: &AudioContext,
        number_of_channels: u32,
        length: u32,
        sample_rate: f32,
    ) -> Result<AudioBuffer, JsValue>;

    #[cfg(feature = "AudioBufferSourceNode")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "AudioContext" , js_name = createBufferSource ) ]
    ///The `createBufferSource()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/createBufferSource)
    ///
    ///*This API requires the following crate features to be activated: `AudioBufferSourceNode`, `AudioContext`*
    pub fn create_buffer_source(this: &AudioContext) -> Result<AudioBufferSourceNode, JsValue>;

    #[cfg(feature = "ChannelMergerNode")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "AudioContext" , js_name = createChannelMerger ) ]
    ///The `createChannelMerger()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/createChannelMerger)
    ///
    ///*This API requires the following crate features to be activated: `AudioContext`, `ChannelMergerNode`*
    pub fn create_channel_merger(this: &AudioContext) -> Result<ChannelMergerNode, JsValue>;

    #[cfg(feature = "ChannelMergerNode")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "AudioContext" , js_name = createChannelMerger ) ]
    ///The `createChannelMerger()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/createChannelMerger)
    ///
    ///*This API requires the following crate features to be activated: `AudioContext`, `ChannelMergerNode`*
    pub fn create_channel_merger_with_number_of_inputs(
        this: &AudioContext,
        number_of_inputs: u32,
    ) -> Result<ChannelMergerNode, JsValue>;

    #[cfg(feature = "ChannelSplitterNode")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "AudioContext" , js_name = createChannelSplitter ) ]
    ///The `createChannelSplitter()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/createChannelSplitter)
    ///
    ///*This API requires the following crate features to be activated: `AudioContext`, `ChannelSplitterNode`*
    pub fn create_channel_splitter(this: &AudioContext) -> Result<ChannelSplitterNode, JsValue>;

    #[cfg(feature = "ChannelSplitterNode")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "AudioContext" , js_name = createChannelSplitter ) ]
    ///The `createChannelSplitter()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/createChannelSplitter)
    ///
    ///*This API requires the following crate features to be activated: `AudioContext`, `ChannelSplitterNode`*
    pub fn create_channel_splitter_with_number_of_outputs(
        this: &AudioContext,
        number_of_outputs: u32,
    ) -> Result<ChannelSplitterNode, JsValue>;

    #[cfg(feature = "ConstantSourceNode")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "AudioContext" , js_name = createConstantSource ) ]
    ///The `createConstantSource()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/createConstantSource)
    ///
    ///*This API requires the following crate features to be activated: `AudioContext`, `ConstantSourceNode`*
    pub fn create_constant_source(this: &AudioContext) -> Result<ConstantSourceNode, JsValue>;

    #[cfg(feature = "ConvolverNode")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "AudioContext" , js_name = createConvolver ) ]
    ///The `createConvolver()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/createConvolver)
    ///
    ///*This API requires the following crate features to be activated: `AudioContext`, `ConvolverNode`*
    pub fn create_convolver(this: &AudioContext) -> Result<ConvolverNode, JsValue>;

    #[cfg(feature = "DelayNode")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "AudioContext" , js_name = createDelay ) ]
    ///The `createDelay()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/createDelay)
    ///
    ///*This API requires the following crate features to be activated: `AudioContext`, `DelayNode`*
    pub fn create_delay(this: &AudioContext) -> Result<DelayNode, JsValue>;

    #[cfg(feature = "DelayNode")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "AudioContext" , js_name = createDelay ) ]
    ///The `createDelay()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/createDelay)
    ///
    ///*This API requires the following crate features to be activated: `AudioContext`, `DelayNode`*
    pub fn create_delay_with_max_delay_time(
        this: &AudioContext,
        max_delay_time: f64,
    ) -> Result<DelayNode, JsValue>;

    #[cfg(feature = "DynamicsCompressorNode")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "AudioContext" , js_name = createDynamicsCompressor ) ]
    ///The `createDynamicsCompressor()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/createDynamicsCompressor)
    ///
    ///*This API requires the following crate features to be activated: `AudioContext`, `DynamicsCompressorNode`*
    pub fn create_dynamics_compressor(
        this: &AudioContext,
    ) -> Result<DynamicsCompressorNode, JsValue>;

    #[cfg(feature = "GainNode")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "AudioContext" , js_name = createGain ) ]
    ///The `createGain()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/createGain)
    ///
    ///*This API requires the following crate features to be activated: `AudioContext`, `GainNode`*
    pub fn create_gain(this: &AudioContext) -> Result<GainNode, JsValue>;

    #[cfg(feature = "IirFilterNode")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "AudioContext" , js_name = createIIRFilter ) ]
    ///The `createIIRFilter()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/createIIRFilter)
    ///
    ///*This API requires the following crate features to be activated: `AudioContext`, `IirFilterNode`*
    pub fn create_iir_filter(
        this: &AudioContext,
        feedforward: &::wasm_bindgen::JsValue,
        feedback: &::wasm_bindgen::JsValue,
    ) -> Result<IirFilterNode, JsValue>;

    #[cfg(feature = "OscillatorNode")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "AudioContext" , js_name = createOscillator ) ]
    ///The `createOscillator()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/createOscillator)
    ///
    ///*This API requires the following crate features to be activated: `AudioContext`, `OscillatorNode`*
    pub fn create_oscillator(this: &AudioContext) -> Result<OscillatorNode, JsValue>;

    #[cfg(feature = "PannerNode")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "AudioContext" , js_name = createPanner ) ]
    ///The `createPanner()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/createPanner)
    ///
    ///*This API requires the following crate features to be activated: `AudioContext`, `PannerNode`*
    pub fn create_panner(this: &AudioContext) -> Result<PannerNode, JsValue>;

    #[cfg(feature = "PeriodicWave")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "AudioContext" , js_name = createPeriodicWave ) ]
    ///The `createPeriodicWave()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/createPeriodicWave)
    ///
    ///*This API requires the following crate features to be activated: `AudioContext`, `PeriodicWave`*
    pub fn create_periodic_wave(
        this: &AudioContext,
        real: &mut [f32],
        imag: &mut [f32],
    ) -> Result<PeriodicWave, JsValue>;

    #[cfg(all(feature = "PeriodicWave", feature = "PeriodicWaveConstraints",))]
    # [ wasm_bindgen ( catch , method , structural , js_class = "AudioContext" , js_name = createPeriodicWave ) ]
    ///The `createPeriodicWave()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/createPeriodicWave)
    ///
    ///*This API requires the following crate features to be activated: `AudioContext`, `PeriodicWave`, `PeriodicWaveConstraints`*
    pub fn create_periodic_wave_with_constraints(
        this: &AudioContext,
        real: &mut [f32],
        imag: &mut [f32],
        constraints: &PeriodicWaveConstraints,
    ) -> Result<PeriodicWave, JsValue>;

    #[cfg(feature = "ScriptProcessorNode")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "AudioContext" , js_name = createScriptProcessor ) ]
    ///The `createScriptProcessor()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/createScriptProcessor)
    ///
    ///*This API requires the following crate features to be activated: `AudioContext`, `ScriptProcessorNode`*
    pub fn create_script_processor(this: &AudioContext) -> Result<ScriptProcessorNode, JsValue>;

    #[cfg(feature = "ScriptProcessorNode")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "AudioContext" , js_name = createScriptProcessor ) ]
    ///The `createScriptProcessor()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/createScriptProcessor)
    ///
    ///*This API requires the following crate features to be activated: `AudioContext`, `ScriptProcessorNode`*
    pub fn create_script_processor_with_buffer_size(
        this: &AudioContext,
        buffer_size: u32,
    ) -> Result<ScriptProcessorNode, JsValue>;

    #[cfg(feature = "ScriptProcessorNode")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "AudioContext" , js_name = createScriptProcessor ) ]
    ///The `createScriptProcessor()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/createScriptProcessor)
    ///
    ///*This API requires the following crate features to be activated: `AudioContext`, `ScriptProcessorNode`*
    pub fn create_script_processor_with_buffer_size_and_number_of_input_channels(
        this: &AudioContext,
        buffer_size: u32,
        number_of_input_channels: u32,
    ) -> Result<ScriptProcessorNode, JsValue>;

    #[cfg(feature = "ScriptProcessorNode")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "AudioContext" , js_name = createScriptProcessor ) ]
    ///The `createScriptProcessor()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/createScriptProcessor)
    ///
    ///*This API requires the following crate features to be activated: `AudioContext`, `ScriptProcessorNode`*
    pub fn create_script_processor_with_buffer_size_and_number_of_input_channels_and_number_of_output_channels(
        this: &AudioContext,
        buffer_size: u32,
        number_of_input_channels: u32,
        number_of_output_channels: u32,
    ) -> Result<ScriptProcessorNode, JsValue>;

    #[cfg(feature = "StereoPannerNode")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "AudioContext" , js_name = createStereoPanner ) ]
    ///The `createStereoPanner()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/createStereoPanner)
    ///
    ///*This API requires the following crate features to be activated: `AudioContext`, `StereoPannerNode`*
    pub fn create_stereo_panner(this: &AudioContext) -> Result<StereoPannerNode, JsValue>;

    #[cfg(feature = "WaveShaperNode")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "AudioContext" , js_name = createWaveShaper ) ]
    ///The `createWaveShaper()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/createWaveShaper)
    ///
    ///*This API requires the following crate features to be activated: `AudioContext`, `WaveShaperNode`*
    pub fn create_wave_shaper(this: &AudioContext) -> Result<WaveShaperNode, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "AudioContext" , js_name = decodeAudioData ) ]
    ///The `decodeAudioData()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/decodeAudioData)
    ///
    ///*This API requires the following crate features to be activated: `AudioContext`*
    pub fn decode_audio_data(
        this: &AudioContext,
        audio_data: &::js_sys::ArrayBuffer,
    ) -> Result<::js_sys::Promise, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "AudioContext" , js_name = decodeAudioData ) ]
    ///The `decodeAudioData()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/decodeAudioData)
    ///
    ///*This API requires the following crate features to be activated: `AudioContext`*
    pub fn decode_audio_data_with_success_callback(
        this: &AudioContext,
        audio_data: &::js_sys::ArrayBuffer,
        success_callback: &::js_sys::Function,
    ) -> Result<::js_sys::Promise, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "AudioContext" , js_name = decodeAudioData ) ]
    ///The `decodeAudioData()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/decodeAudioData)
    ///
    ///*This API requires the following crate features to be activated: `AudioContext`*
    pub fn decode_audio_data_with_success_callback_and_error_callback(
        this: &AudioContext,
        audio_data: &::js_sys::ArrayBuffer,
        success_callback: &::js_sys::Function,
        error_callback: &::js_sys::Function,
    ) -> Result<::js_sys::Promise, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "AudioContext" , js_name = resume ) ]
    ///The `resume()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/resume)
    ///
    ///*This API requires the following crate features to be activated: `AudioContext`*
    pub fn resume(this: &AudioContext) -> Result<::js_sys::Promise, JsValue>;

}
