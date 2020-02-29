use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = AudioParamMap , typescript_type = "AudioParamMap" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `AudioParamMap` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioParamMap)
    ///
    ///*This API requires the following crate features to be activated: `AudioParamMap`*
    pub type AudioParamMap;

}
