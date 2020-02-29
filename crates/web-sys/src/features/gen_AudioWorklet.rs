use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = Worklet , extends = :: js_sys :: Object , js_name = AudioWorklet , typescript_name = AudioWorklet ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `AudioWorklet` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioWorklet)
    ///
    ///*This API requires the following crate features to be activated: `AudioWorklet`*
    pub type AudioWorklet;

}
