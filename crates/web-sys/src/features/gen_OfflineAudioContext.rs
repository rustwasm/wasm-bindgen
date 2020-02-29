use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( vendor_prefix = webkit , extends = BaseAudioContext , extends = EventTarget , extends = :: js_sys :: Object , js_name = OfflineAudioContext , typescript_type = "OfflineAudioContext" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `OfflineAudioContext` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext)
    ///
    ///*This API requires the following crate features to be activated: `OfflineAudioContext`*
    pub type OfflineAudioContext;

    # [ wasm_bindgen ( structural , method , getter , js_class = "OfflineAudioContext" , js_name = length ) ]
    ///Getter for the `length` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/length)
    ///
    ///*This API requires the following crate features to be activated: `OfflineAudioContext`*
    pub fn length(this: &OfflineAudioContext) -> u32;

    # [ wasm_bindgen ( structural , method , getter , js_class = "OfflineAudioContext" , js_name = oncomplete ) ]
    ///Getter for the `oncomplete` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/oncomplete)
    ///
    ///*This API requires the following crate features to be activated: `OfflineAudioContext`*
    pub fn oncomplete(this: &OfflineAudioContext) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "OfflineAudioContext" , js_name = oncomplete ) ]
    ///Setter for the `oncomplete` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/oncomplete)
    ///
    ///*This API requires the following crate features to be activated: `OfflineAudioContext`*
    pub fn set_oncomplete(this: &OfflineAudioContext, value: Option<&::js_sys::Function>);

    #[cfg(feature = "AudioDestinationNode")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "OfflineAudioContext" , js_name = destination ) ]
    ///Getter for the `destination` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/destination)
    ///
    ///*This API requires the following crate features to be activated: `AudioDestinationNode`, `OfflineAudioContext`*
    pub fn destination(this: &OfflineAudioContext) -> AudioDestinationNode;

    # [ wasm_bindgen ( structural , method , getter , js_class = "OfflineAudioContext" , js_name = sampleRate ) ]
    ///Getter for the `sampleRate` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/sampleRate)
    ///
    ///*This API requires the following crate features to be activated: `OfflineAudioContext`*
    pub fn sample_rate(this: &OfflineAudioContext) -> f32;

    # [ wasm_bindgen ( structural , method , getter , js_class = "OfflineAudioContext" , js_name = currentTime ) ]
    ///Getter for the `currentTime` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/currentTime)
    ///
    ///*This API requires the following crate features to be activated: `OfflineAudioContext`*
    pub fn current_time(this: &OfflineAudioContext) -> f64;

    #[cfg(feature = "AudioListener")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "OfflineAudioContext" , js_name = listener ) ]
    ///Getter for the `listener` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/listener)
    ///
    ///*This API requires the following crate features to be activated: `AudioListener`, `OfflineAudioContext`*
    pub fn listener(this: &OfflineAudioContext) -> AudioListener;

    #[cfg(feature = "AudioContextState")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "OfflineAudioContext" , js_name = state ) ]
    ///Getter for the `state` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/state)
    ///
    ///*This API requires the following crate features to be activated: `AudioContextState`, `OfflineAudioContext`*
    pub fn state(this: &OfflineAudioContext) -> AudioContextState;

    #[cfg(feature = "AudioWorklet")]
    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "OfflineAudioContext" , js_name = audioWorklet ) ]
    ///Getter for the `audioWorklet` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/audioWorklet)
    ///
    ///*This API requires the following crate features to be activated: `AudioWorklet`, `OfflineAudioContext`*
    pub fn audio_worklet(this: &OfflineAudioContext) -> Result<AudioWorklet, JsValue>;

    # [ wasm_bindgen ( structural , method , getter , js_class = "OfflineAudioContext" , js_name = onstatechange ) ]
    ///Getter for the `onstatechange` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/onstatechange)
    ///
    ///*This API requires the following crate features to be activated: `OfflineAudioContext`*
    pub fn onstatechange(this: &OfflineAudioContext) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "OfflineAudioContext" , js_name = onstatechange ) ]
    ///Setter for the `onstatechange` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/onstatechange)
    ///
    ///*This API requires the following crate features to be activated: `OfflineAudioContext`*
    pub fn set_onstatechange(this: &OfflineAudioContext, value: Option<&::js_sys::Function>);

    #[cfg(feature = "OfflineAudioContextOptions")]
    #[wasm_bindgen(catch, constructor, js_class = "OfflineAudioContext")]
    ///The `new OfflineAudioContext(..)` constructor, creating a new instance of `OfflineAudioContext`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/OfflineAudioContext)
    ///
    ///*This API requires the following crate features to be activated: `OfflineAudioContext`, `OfflineAudioContextOptions`*
    pub fn new_with_context_options(
        context_options: &OfflineAudioContextOptions,
    ) -> Result<OfflineAudioContext, JsValue>;

    #[wasm_bindgen(catch, constructor, js_class = "OfflineAudioContext")]
    ///The `new OfflineAudioContext(..)` constructor, creating a new instance of `OfflineAudioContext`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/OfflineAudioContext)
    ///
    ///*This API requires the following crate features to be activated: `OfflineAudioContext`*
    pub fn new_with_number_of_channels_and_length_and_sample_rate(
        number_of_channels: u32,
        length: u32,
        sample_rate: f32,
    ) -> Result<OfflineAudioContext, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "OfflineAudioContext" , js_name = startRendering ) ]
    ///The `startRendering()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/startRendering)
    ///
    ///*This API requires the following crate features to be activated: `OfflineAudioContext`*
    pub fn start_rendering(this: &OfflineAudioContext) -> Result<::js_sys::Promise, JsValue>;

    #[cfg(feature = "AnalyserNode")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "OfflineAudioContext" , js_name = createAnalyser ) ]
    ///The `createAnalyser()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/createAnalyser)
    ///
    ///*This API requires the following crate features to be activated: `AnalyserNode`, `OfflineAudioContext`*
    pub fn create_analyser(this: &OfflineAudioContext) -> Result<AnalyserNode, JsValue>;

    #[cfg(feature = "BiquadFilterNode")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "OfflineAudioContext" , js_name = createBiquadFilter ) ]
    ///The `createBiquadFilter()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/createBiquadFilter)
    ///
    ///*This API requires the following crate features to be activated: `BiquadFilterNode`, `OfflineAudioContext`*
    pub fn create_biquad_filter(this: &OfflineAudioContext) -> Result<BiquadFilterNode, JsValue>;

    #[cfg(feature = "AudioBuffer")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "OfflineAudioContext" , js_name = createBuffer ) ]
    ///The `createBuffer()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/createBuffer)
    ///
    ///*This API requires the following crate features to be activated: `AudioBuffer`, `OfflineAudioContext`*
    pub fn create_buffer(
        this: &OfflineAudioContext,
        number_of_channels: u32,
        length: u32,
        sample_rate: f32,
    ) -> Result<AudioBuffer, JsValue>;

    #[cfg(feature = "AudioBufferSourceNode")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "OfflineAudioContext" , js_name = createBufferSource ) ]
    ///The `createBufferSource()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/createBufferSource)
    ///
    ///*This API requires the following crate features to be activated: `AudioBufferSourceNode`, `OfflineAudioContext`*
    pub fn create_buffer_source(
        this: &OfflineAudioContext,
    ) -> Result<AudioBufferSourceNode, JsValue>;

    #[cfg(feature = "ChannelMergerNode")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "OfflineAudioContext" , js_name = createChannelMerger ) ]
    ///The `createChannelMerger()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/createChannelMerger)
    ///
    ///*This API requires the following crate features to be activated: `ChannelMergerNode`, `OfflineAudioContext`*
    pub fn create_channel_merger(this: &OfflineAudioContext) -> Result<ChannelMergerNode, JsValue>;

    #[cfg(feature = "ChannelMergerNode")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "OfflineAudioContext" , js_name = createChannelMerger ) ]
    ///The `createChannelMerger()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/createChannelMerger)
    ///
    ///*This API requires the following crate features to be activated: `ChannelMergerNode`, `OfflineAudioContext`*
    pub fn create_channel_merger_with_number_of_inputs(
        this: &OfflineAudioContext,
        number_of_inputs: u32,
    ) -> Result<ChannelMergerNode, JsValue>;

    #[cfg(feature = "ChannelSplitterNode")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "OfflineAudioContext" , js_name = createChannelSplitter ) ]
    ///The `createChannelSplitter()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/createChannelSplitter)
    ///
    ///*This API requires the following crate features to be activated: `ChannelSplitterNode`, `OfflineAudioContext`*
    pub fn create_channel_splitter(
        this: &OfflineAudioContext,
    ) -> Result<ChannelSplitterNode, JsValue>;

    #[cfg(feature = "ChannelSplitterNode")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "OfflineAudioContext" , js_name = createChannelSplitter ) ]
    ///The `createChannelSplitter()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/createChannelSplitter)
    ///
    ///*This API requires the following crate features to be activated: `ChannelSplitterNode`, `OfflineAudioContext`*
    pub fn create_channel_splitter_with_number_of_outputs(
        this: &OfflineAudioContext,
        number_of_outputs: u32,
    ) -> Result<ChannelSplitterNode, JsValue>;

    #[cfg(feature = "ConstantSourceNode")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "OfflineAudioContext" , js_name = createConstantSource ) ]
    ///The `createConstantSource()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/createConstantSource)
    ///
    ///*This API requires the following crate features to be activated: `ConstantSourceNode`, `OfflineAudioContext`*
    pub fn create_constant_source(
        this: &OfflineAudioContext,
    ) -> Result<ConstantSourceNode, JsValue>;

    #[cfg(feature = "ConvolverNode")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "OfflineAudioContext" , js_name = createConvolver ) ]
    ///The `createConvolver()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/createConvolver)
    ///
    ///*This API requires the following crate features to be activated: `ConvolverNode`, `OfflineAudioContext`*
    pub fn create_convolver(this: &OfflineAudioContext) -> Result<ConvolverNode, JsValue>;

    #[cfg(feature = "DelayNode")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "OfflineAudioContext" , js_name = createDelay ) ]
    ///The `createDelay()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/createDelay)
    ///
    ///*This API requires the following crate features to be activated: `DelayNode`, `OfflineAudioContext`*
    pub fn create_delay(this: &OfflineAudioContext) -> Result<DelayNode, JsValue>;

    #[cfg(feature = "DelayNode")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "OfflineAudioContext" , js_name = createDelay ) ]
    ///The `createDelay()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/createDelay)
    ///
    ///*This API requires the following crate features to be activated: `DelayNode`, `OfflineAudioContext`*
    pub fn create_delay_with_max_delay_time(
        this: &OfflineAudioContext,
        max_delay_time: f64,
    ) -> Result<DelayNode, JsValue>;

    #[cfg(feature = "DynamicsCompressorNode")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "OfflineAudioContext" , js_name = createDynamicsCompressor ) ]
    ///The `createDynamicsCompressor()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/createDynamicsCompressor)
    ///
    ///*This API requires the following crate features to be activated: `DynamicsCompressorNode`, `OfflineAudioContext`*
    pub fn create_dynamics_compressor(
        this: &OfflineAudioContext,
    ) -> Result<DynamicsCompressorNode, JsValue>;

    #[cfg(feature = "GainNode")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "OfflineAudioContext" , js_name = createGain ) ]
    ///The `createGain()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/createGain)
    ///
    ///*This API requires the following crate features to be activated: `GainNode`, `OfflineAudioContext`*
    pub fn create_gain(this: &OfflineAudioContext) -> Result<GainNode, JsValue>;

    #[cfg(feature = "IirFilterNode")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "OfflineAudioContext" , js_name = createIIRFilter ) ]
    ///The `createIIRFilter()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/createIIRFilter)
    ///
    ///*This API requires the following crate features to be activated: `IirFilterNode`, `OfflineAudioContext`*
    pub fn create_iir_filter(
        this: &OfflineAudioContext,
        feedforward: &::wasm_bindgen::JsValue,
        feedback: &::wasm_bindgen::JsValue,
    ) -> Result<IirFilterNode, JsValue>;

    #[cfg(feature = "OscillatorNode")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "OfflineAudioContext" , js_name = createOscillator ) ]
    ///The `createOscillator()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/createOscillator)
    ///
    ///*This API requires the following crate features to be activated: `OfflineAudioContext`, `OscillatorNode`*
    pub fn create_oscillator(this: &OfflineAudioContext) -> Result<OscillatorNode, JsValue>;

    #[cfg(feature = "PannerNode")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "OfflineAudioContext" , js_name = createPanner ) ]
    ///The `createPanner()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/createPanner)
    ///
    ///*This API requires the following crate features to be activated: `OfflineAudioContext`, `PannerNode`*
    pub fn create_panner(this: &OfflineAudioContext) -> Result<PannerNode, JsValue>;

    #[cfg(feature = "PeriodicWave")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "OfflineAudioContext" , js_name = createPeriodicWave ) ]
    ///The `createPeriodicWave()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/createPeriodicWave)
    ///
    ///*This API requires the following crate features to be activated: `OfflineAudioContext`, `PeriodicWave`*
    pub fn create_periodic_wave(
        this: &OfflineAudioContext,
        real: &mut [f32],
        imag: &mut [f32],
    ) -> Result<PeriodicWave, JsValue>;

    #[cfg(all(feature = "PeriodicWave", feature = "PeriodicWaveConstraints",))]
    # [ wasm_bindgen ( catch , method , structural , js_class = "OfflineAudioContext" , js_name = createPeriodicWave ) ]
    ///The `createPeriodicWave()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/createPeriodicWave)
    ///
    ///*This API requires the following crate features to be activated: `OfflineAudioContext`, `PeriodicWave`, `PeriodicWaveConstraints`*
    pub fn create_periodic_wave_with_constraints(
        this: &OfflineAudioContext,
        real: &mut [f32],
        imag: &mut [f32],
        constraints: &PeriodicWaveConstraints,
    ) -> Result<PeriodicWave, JsValue>;

    #[cfg(feature = "ScriptProcessorNode")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "OfflineAudioContext" , js_name = createScriptProcessor ) ]
    ///The `createScriptProcessor()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/createScriptProcessor)
    ///
    ///*This API requires the following crate features to be activated: `OfflineAudioContext`, `ScriptProcessorNode`*
    pub fn create_script_processor(
        this: &OfflineAudioContext,
    ) -> Result<ScriptProcessorNode, JsValue>;

    #[cfg(feature = "ScriptProcessorNode")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "OfflineAudioContext" , js_name = createScriptProcessor ) ]
    ///The `createScriptProcessor()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/createScriptProcessor)
    ///
    ///*This API requires the following crate features to be activated: `OfflineAudioContext`, `ScriptProcessorNode`*
    pub fn create_script_processor_with_buffer_size(
        this: &OfflineAudioContext,
        buffer_size: u32,
    ) -> Result<ScriptProcessorNode, JsValue>;

    #[cfg(feature = "ScriptProcessorNode")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "OfflineAudioContext" , js_name = createScriptProcessor ) ]
    ///The `createScriptProcessor()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/createScriptProcessor)
    ///
    ///*This API requires the following crate features to be activated: `OfflineAudioContext`, `ScriptProcessorNode`*
    pub fn create_script_processor_with_buffer_size_and_number_of_input_channels(
        this: &OfflineAudioContext,
        buffer_size: u32,
        number_of_input_channels: u32,
    ) -> Result<ScriptProcessorNode, JsValue>;

    #[cfg(feature = "ScriptProcessorNode")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "OfflineAudioContext" , js_name = createScriptProcessor ) ]
    ///The `createScriptProcessor()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/createScriptProcessor)
    ///
    ///*This API requires the following crate features to be activated: `OfflineAudioContext`, `ScriptProcessorNode`*
    pub fn create_script_processor_with_buffer_size_and_number_of_input_channels_and_number_of_output_channels(
        this: &OfflineAudioContext,
        buffer_size: u32,
        number_of_input_channels: u32,
        number_of_output_channels: u32,
    ) -> Result<ScriptProcessorNode, JsValue>;

    #[cfg(feature = "StereoPannerNode")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "OfflineAudioContext" , js_name = createStereoPanner ) ]
    ///The `createStereoPanner()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/createStereoPanner)
    ///
    ///*This API requires the following crate features to be activated: `OfflineAudioContext`, `StereoPannerNode`*
    pub fn create_stereo_panner(this: &OfflineAudioContext) -> Result<StereoPannerNode, JsValue>;

    #[cfg(feature = "WaveShaperNode")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "OfflineAudioContext" , js_name = createWaveShaper ) ]
    ///The `createWaveShaper()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/createWaveShaper)
    ///
    ///*This API requires the following crate features to be activated: `OfflineAudioContext`, `WaveShaperNode`*
    pub fn create_wave_shaper(this: &OfflineAudioContext) -> Result<WaveShaperNode, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "OfflineAudioContext" , js_name = decodeAudioData ) ]
    ///The `decodeAudioData()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/decodeAudioData)
    ///
    ///*This API requires the following crate features to be activated: `OfflineAudioContext`*
    pub fn decode_audio_data(
        this: &OfflineAudioContext,
        audio_data: &::js_sys::ArrayBuffer,
    ) -> Result<::js_sys::Promise, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "OfflineAudioContext" , js_name = decodeAudioData ) ]
    ///The `decodeAudioData()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/decodeAudioData)
    ///
    ///*This API requires the following crate features to be activated: `OfflineAudioContext`*
    pub fn decode_audio_data_with_success_callback(
        this: &OfflineAudioContext,
        audio_data: &::js_sys::ArrayBuffer,
        success_callback: &::js_sys::Function,
    ) -> Result<::js_sys::Promise, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "OfflineAudioContext" , js_name = decodeAudioData ) ]
    ///The `decodeAudioData()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/decodeAudioData)
    ///
    ///*This API requires the following crate features to be activated: `OfflineAudioContext`*
    pub fn decode_audio_data_with_success_callback_and_error_callback(
        this: &OfflineAudioContext,
        audio_data: &::js_sys::ArrayBuffer,
        success_callback: &::js_sys::Function,
        error_callback: &::js_sys::Function,
    ) -> Result<::js_sys::Promise, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "OfflineAudioContext" , js_name = resume ) ]
    ///The `resume()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/resume)
    ///
    ///*This API requires the following crate features to be activated: `OfflineAudioContext`*
    pub fn resume(this: &OfflineAudioContext) -> Result<::js_sys::Promise, JsValue>;

}
