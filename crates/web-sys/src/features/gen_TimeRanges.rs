use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = TimeRanges , typescript_name = TimeRanges ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `TimeRanges` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TimeRanges)\n\n*This API requires the following crate features to be activated: `TimeRanges`*"]
    pub type TimeRanges;
    # [ wasm_bindgen ( structural , method , getter , js_class = "TimeRanges" , js_name = length ) ]
    #[doc = "Getter for the `length` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TimeRanges/length)\n\n*This API requires the following crate features to be activated: `TimeRanges`*"]
    pub fn length(this: &TimeRanges) -> u32;
    # [ wasm_bindgen ( catch , method , structural , js_class = "TimeRanges" , js_name = end ) ]
    #[doc = "The `end()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TimeRanges/end)\n\n*This API requires the following crate features to be activated: `TimeRanges`*"]
    pub fn end(this: &TimeRanges, index: u32) -> Result<f64, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_class = "TimeRanges" , js_name = start ) ]
    #[doc = "The `start()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TimeRanges/start)\n\n*This API requires the following crate features to be activated: `TimeRanges`*"]
    pub fn start(this: &TimeRanges, index: u32) -> Result<f64, JsValue>;
}
