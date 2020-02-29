use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = TimeRanges , typescript_type = "TimeRanges" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `TimeRanges` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TimeRanges)
    ///
    ///*This API requires the following crate features to be activated: `TimeRanges`*
    pub type TimeRanges;

    # [ wasm_bindgen ( structural , method , getter , js_class = "TimeRanges" , js_name = length ) ]
    ///Getter for the `length` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TimeRanges/length)
    ///
    ///*This API requires the following crate features to be activated: `TimeRanges`*
    pub fn length(this: &TimeRanges) -> u32;

    # [ wasm_bindgen ( catch , method , structural , js_class = "TimeRanges" , js_name = end ) ]
    ///The `end()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TimeRanges/end)
    ///
    ///*This API requires the following crate features to be activated: `TimeRanges`*
    pub fn end(this: &TimeRanges, index: u32) -> Result<f64, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "TimeRanges" , js_name = start ) ]
    ///The `start()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TimeRanges/start)
    ///
    ///*This API requires the following crate features to be activated: `TimeRanges`*
    pub fn start(this: &TimeRanges, index: u32) -> Result<f64, JsValue>;

}
