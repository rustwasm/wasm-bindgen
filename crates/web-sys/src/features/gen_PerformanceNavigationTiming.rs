use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = PerformanceResourceTiming , extends = PerformanceEntry , extends = :: js_sys :: Object , js_name = PerformanceNavigationTiming , typescript_name = PerformanceNavigationTiming ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PerformanceNavigationTiming` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceNavigationTiming)\n\n*This API requires the following crate features to be activated: `PerformanceNavigationTiming`*"]
    pub type PerformanceNavigationTiming;
    # [ wasm_bindgen ( structural , method , getter , js_class = "PerformanceNavigationTiming" , js_name = unloadEventStart ) ]
    #[doc = "Getter for the `unloadEventStart` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceNavigationTiming/unloadEventStart)\n\n*This API requires the following crate features to be activated: `PerformanceNavigationTiming`*"]
    pub fn unload_event_start(this: &PerformanceNavigationTiming) -> f64;
    # [ wasm_bindgen ( structural , method , getter , js_class = "PerformanceNavigationTiming" , js_name = unloadEventEnd ) ]
    #[doc = "Getter for the `unloadEventEnd` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceNavigationTiming/unloadEventEnd)\n\n*This API requires the following crate features to be activated: `PerformanceNavigationTiming`*"]
    pub fn unload_event_end(this: &PerformanceNavigationTiming) -> f64;
    # [ wasm_bindgen ( structural , method , getter , js_class = "PerformanceNavigationTiming" , js_name = domInteractive ) ]
    #[doc = "Getter for the `domInteractive` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceNavigationTiming/domInteractive)\n\n*This API requires the following crate features to be activated: `PerformanceNavigationTiming`*"]
    pub fn dom_interactive(this: &PerformanceNavigationTiming) -> f64;
    # [ wasm_bindgen ( structural , method , getter , js_class = "PerformanceNavigationTiming" , js_name = domContentLoadedEventStart ) ]
    #[doc = "Getter for the `domContentLoadedEventStart` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceNavigationTiming/domContentLoadedEventStart)\n\n*This API requires the following crate features to be activated: `PerformanceNavigationTiming`*"]
    pub fn dom_content_loaded_event_start(this: &PerformanceNavigationTiming) -> f64;
    # [ wasm_bindgen ( structural , method , getter , js_class = "PerformanceNavigationTiming" , js_name = domContentLoadedEventEnd ) ]
    #[doc = "Getter for the `domContentLoadedEventEnd` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceNavigationTiming/domContentLoadedEventEnd)\n\n*This API requires the following crate features to be activated: `PerformanceNavigationTiming`*"]
    pub fn dom_content_loaded_event_end(this: &PerformanceNavigationTiming) -> f64;
    # [ wasm_bindgen ( structural , method , getter , js_class = "PerformanceNavigationTiming" , js_name = domComplete ) ]
    #[doc = "Getter for the `domComplete` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceNavigationTiming/domComplete)\n\n*This API requires the following crate features to be activated: `PerformanceNavigationTiming`*"]
    pub fn dom_complete(this: &PerformanceNavigationTiming) -> f64;
    # [ wasm_bindgen ( structural , method , getter , js_class = "PerformanceNavigationTiming" , js_name = loadEventStart ) ]
    #[doc = "Getter for the `loadEventStart` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceNavigationTiming/loadEventStart)\n\n*This API requires the following crate features to be activated: `PerformanceNavigationTiming`*"]
    pub fn load_event_start(this: &PerformanceNavigationTiming) -> f64;
    # [ wasm_bindgen ( structural , method , getter , js_class = "PerformanceNavigationTiming" , js_name = loadEventEnd ) ]
    #[doc = "Getter for the `loadEventEnd` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceNavigationTiming/loadEventEnd)\n\n*This API requires the following crate features to be activated: `PerformanceNavigationTiming`*"]
    pub fn load_event_end(this: &PerformanceNavigationTiming) -> f64;
    # [ wasm_bindgen ( structural , method , getter , js_class = "PerformanceNavigationTiming" , js_name = type ) ]
    #[cfg(feature = "NavigationType")]
    #[doc = "Getter for the `type` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceNavigationTiming/type)\n\n*This API requires the following crate features to be activated: `NavigationType`, `PerformanceNavigationTiming`*"]
    pub fn type_(this: &PerformanceNavigationTiming) -> NavigationType;
    # [ wasm_bindgen ( structural , method , getter , js_class = "PerformanceNavigationTiming" , js_name = redirectCount ) ]
    #[doc = "Getter for the `redirectCount` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceNavigationTiming/redirectCount)\n\n*This API requires the following crate features to be activated: `PerformanceNavigationTiming`*"]
    pub fn redirect_count(this: &PerformanceNavigationTiming) -> u16;
    # [ wasm_bindgen ( method , structural , js_class = "PerformanceNavigationTiming" , js_name = toJSON ) ]
    #[doc = "The `toJSON()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceNavigationTiming/toJSON)\n\n*This API requires the following crate features to be activated: `PerformanceNavigationTiming`*"]
    pub fn to_json(this: &PerformanceNavigationTiming) -> ::js_sys::Object;
}
