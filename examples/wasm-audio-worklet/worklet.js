import init, * as bindgen from "./pkg/wasm_audio_worklet.js"

registerProcessor("WasmProcessor", class WasmProcessor extends AudioWorkletProcessor {
    constructor(options) {
        super();
        let [module, memory, handle] = options.processorOptions;
        bindgen.initSync(module, memory);
        this.processor = bindgen.WasmAudioProcessor.unpack(handle);
    }
    process(inputs, outputs) {
        return this.processor.process(outputs[0][0]);
    }
});
