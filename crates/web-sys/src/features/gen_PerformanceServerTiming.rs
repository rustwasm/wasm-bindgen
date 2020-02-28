use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = PerformanceServerTiming , typescript_name = PerformanceServerTiming ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PerformanceServerTiming` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceServerTiming)\n\n*This API requires the following crate features to be activated: `PerformanceServerTiming`*"]
    pub type PerformanceServerTiming;
    # [ wasm_bindgen ( structural , method , getter , js_name = name ) ]
    #[doc = "Getter for the `name` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceServerTiming/name)\n\n*This API requires the following crate features to be activated: `PerformanceServerTiming`*"]
    pub fn name(this: &PerformanceServerTiming) -> String;
    # [ wasm_bindgen ( structural , method , getter , js_name = duration ) ]
    #[doc = "Getter for the `duration` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceServerTiming/duration)\n\n*This API requires the following crate features to be activated: `PerformanceServerTiming`*"]
    pub fn duration(this: &PerformanceServerTiming) -> f64;
    # [ wasm_bindgen ( structural , method , getter , js_name = description ) ]
    #[doc = "Getter for the `description` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceServerTiming/description)\n\n*This API requires the following crate features to be activated: `PerformanceServerTiming`*"]
    pub fn description(this: &PerformanceServerTiming) -> String;
    # [ wasm_bindgen ( method , structural , js_name = toJSON ) ]
    #[doc = "The `toJSON()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceServerTiming/toJSON)\n\n*This API requires the following crate features to be activated: `PerformanceServerTiming`*"]
    pub fn to_json(this: &PerformanceServerTiming) -> ::js_sys::Object;
}
