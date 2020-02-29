use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = PerformanceResourceTiming , extends = PerformanceEntry , extends = :: js_sys :: Object , js_name = PerformanceNavigationTiming , typescript_type = "PerformanceNavigationTiming" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `PerformanceNavigationTiming` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceNavigationTiming)
    ///
    ///*This API requires the following crate features to be activated: `PerformanceNavigationTiming`*
    pub type PerformanceNavigationTiming;

    # [ wasm_bindgen ( structural , method , getter , js_class = "PerformanceNavigationTiming" , js_name = unloadEventStart ) ]
    ///Getter for the `unloadEventStart` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceNavigationTiming/unloadEventStart)
    ///
    ///*This API requires the following crate features to be activated: `PerformanceNavigationTiming`*
    pub fn unload_event_start(this: &PerformanceNavigationTiming) -> f64;

    # [ wasm_bindgen ( structural , method , getter , js_class = "PerformanceNavigationTiming" , js_name = unloadEventEnd ) ]
    ///Getter for the `unloadEventEnd` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceNavigationTiming/unloadEventEnd)
    ///
    ///*This API requires the following crate features to be activated: `PerformanceNavigationTiming`*
    pub fn unload_event_end(this: &PerformanceNavigationTiming) -> f64;

    # [ wasm_bindgen ( structural , method , getter , js_class = "PerformanceNavigationTiming" , js_name = domInteractive ) ]
    ///Getter for the `domInteractive` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceNavigationTiming/domInteractive)
    ///
    ///*This API requires the following crate features to be activated: `PerformanceNavigationTiming`*
    pub fn dom_interactive(this: &PerformanceNavigationTiming) -> f64;

    # [ wasm_bindgen ( structural , method , getter , js_class = "PerformanceNavigationTiming" , js_name = domContentLoadedEventStart ) ]
    ///Getter for the `domContentLoadedEventStart` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceNavigationTiming/domContentLoadedEventStart)
    ///
    ///*This API requires the following crate features to be activated: `PerformanceNavigationTiming`*
    pub fn dom_content_loaded_event_start(this: &PerformanceNavigationTiming) -> f64;

    # [ wasm_bindgen ( structural , method , getter , js_class = "PerformanceNavigationTiming" , js_name = domContentLoadedEventEnd ) ]
    ///Getter for the `domContentLoadedEventEnd` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceNavigationTiming/domContentLoadedEventEnd)
    ///
    ///*This API requires the following crate features to be activated: `PerformanceNavigationTiming`*
    pub fn dom_content_loaded_event_end(this: &PerformanceNavigationTiming) -> f64;

    # [ wasm_bindgen ( structural , method , getter , js_class = "PerformanceNavigationTiming" , js_name = domComplete ) ]
    ///Getter for the `domComplete` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceNavigationTiming/domComplete)
    ///
    ///*This API requires the following crate features to be activated: `PerformanceNavigationTiming`*
    pub fn dom_complete(this: &PerformanceNavigationTiming) -> f64;

    # [ wasm_bindgen ( structural , method , getter , js_class = "PerformanceNavigationTiming" , js_name = loadEventStart ) ]
    ///Getter for the `loadEventStart` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceNavigationTiming/loadEventStart)
    ///
    ///*This API requires the following crate features to be activated: `PerformanceNavigationTiming`*
    pub fn load_event_start(this: &PerformanceNavigationTiming) -> f64;

    # [ wasm_bindgen ( structural , method , getter , js_class = "PerformanceNavigationTiming" , js_name = loadEventEnd ) ]
    ///Getter for the `loadEventEnd` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceNavigationTiming/loadEventEnd)
    ///
    ///*This API requires the following crate features to be activated: `PerformanceNavigationTiming`*
    pub fn load_event_end(this: &PerformanceNavigationTiming) -> f64;

    #[cfg(feature = "NavigationType")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "PerformanceNavigationTiming" , js_name = type ) ]
    ///Getter for the `type` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceNavigationTiming/type)
    ///
    ///*This API requires the following crate features to be activated: `NavigationType`, `PerformanceNavigationTiming`*
    pub fn type_(this: &PerformanceNavigationTiming) -> NavigationType;

    # [ wasm_bindgen ( structural , method , getter , js_class = "PerformanceNavigationTiming" , js_name = redirectCount ) ]
    ///Getter for the `redirectCount` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceNavigationTiming/redirectCount)
    ///
    ///*This API requires the following crate features to be activated: `PerformanceNavigationTiming`*
    pub fn redirect_count(this: &PerformanceNavigationTiming) -> u16;

    # [ wasm_bindgen ( method , structural , js_class = "PerformanceNavigationTiming" , js_name = toJSON ) ]
    ///The `toJSON()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceNavigationTiming/toJSON)
    ///
    ///*This API requires the following crate features to be activated: `PerformanceNavigationTiming`*
    pub fn to_json(this: &PerformanceNavigationTiming) -> ::js_sys::Object;

}
