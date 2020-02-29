use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = PerformanceEntry , extends = :: js_sys :: Object , js_name = PerformanceMeasure , typescript_name = PerformanceMeasure ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `PerformanceMeasure` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceMeasure)
    ///
    ///*This API requires the following crate features to be activated: `PerformanceMeasure`*
    pub type PerformanceMeasure;

}
