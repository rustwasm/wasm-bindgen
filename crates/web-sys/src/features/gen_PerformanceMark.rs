use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = PerformanceEntry , extends = :: js_sys :: Object , js_name = PerformanceMark , typescript_name = PerformanceMark ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `PerformanceMark` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceMark)
    ///
    ///*This API requires the following crate features to be activated: `PerformanceMark`*
    pub type PerformanceMark;

}
