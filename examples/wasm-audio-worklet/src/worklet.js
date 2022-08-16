import './polyfill.js';
import * as bindgen from '$wbg_main';

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
