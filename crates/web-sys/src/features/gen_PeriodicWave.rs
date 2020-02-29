use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = PeriodicWave , typescript_name = PeriodicWave ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `PeriodicWave` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PeriodicWave)
    ///
    ///*This API requires the following crate features to be activated: `PeriodicWave`*
    pub type PeriodicWave;

    #[cfg(feature = "BaseAudioContext")]
    #[wasm_bindgen(catch, constructor, js_class = "PeriodicWave")]
    ///The `new PeriodicWave(..)` constructor, creating a new instance of `PeriodicWave`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PeriodicWave/PeriodicWave)
    ///
    ///*This API requires the following crate features to be activated: `BaseAudioContext`, `PeriodicWave`*
    pub fn new(context: &BaseAudioContext) -> Result<PeriodicWave, JsValue>;

    #[cfg(all(feature = "BaseAudioContext", feature = "PeriodicWaveOptions",))]
    #[wasm_bindgen(catch, constructor, js_class = "PeriodicWave")]
    ///The `new PeriodicWave(..)` constructor, creating a new instance of `PeriodicWave`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PeriodicWave/PeriodicWave)
    ///
    ///*This API requires the following crate features to be activated: `BaseAudioContext`, `PeriodicWave`, `PeriodicWaveOptions`*
    pub fn new_with_options(
        context: &BaseAudioContext,
        options: &PeriodicWaveOptions,
    ) -> Result<PeriodicWave, JsValue>;

}
