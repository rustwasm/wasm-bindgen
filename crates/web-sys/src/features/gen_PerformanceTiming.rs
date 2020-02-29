use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = PerformanceTiming , typescript_type = "PerformanceTiming" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `PerformanceTiming` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceTiming)
    ///
    ///*This API requires the following crate features to be activated: `PerformanceTiming`*
    pub type PerformanceTiming;

    # [ wasm_bindgen ( structural , method , getter , js_class = "PerformanceTiming" , js_name = navigationStart ) ]
    ///Getter for the `navigationStart` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceTiming/navigationStart)
    ///
    ///*This API requires the following crate features to be activated: `PerformanceTiming`*
    pub fn navigation_start(this: &PerformanceTiming) -> f64;

    # [ wasm_bindgen ( structural , method , getter , js_class = "PerformanceTiming" , js_name = unloadEventStart ) ]
    ///Getter for the `unloadEventStart` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceTiming/unloadEventStart)
    ///
    ///*This API requires the following crate features to be activated: `PerformanceTiming`*
    pub fn unload_event_start(this: &PerformanceTiming) -> f64;

    # [ wasm_bindgen ( structural , method , getter , js_class = "PerformanceTiming" , js_name = unloadEventEnd ) ]
    ///Getter for the `unloadEventEnd` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceTiming/unloadEventEnd)
    ///
    ///*This API requires the following crate features to be activated: `PerformanceTiming`*
    pub fn unload_event_end(this: &PerformanceTiming) -> f64;

    # [ wasm_bindgen ( structural , method , getter , js_class = "PerformanceTiming" , js_name = redirectStart ) ]
    ///Getter for the `redirectStart` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceTiming/redirectStart)
    ///
    ///*This API requires the following crate features to be activated: `PerformanceTiming`*
    pub fn redirect_start(this: &PerformanceTiming) -> f64;

    # [ wasm_bindgen ( structural , method , getter , js_class = "PerformanceTiming" , js_name = redirectEnd ) ]
    ///Getter for the `redirectEnd` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceTiming/redirectEnd)
    ///
    ///*This API requires the following crate features to be activated: `PerformanceTiming`*
    pub fn redirect_end(this: &PerformanceTiming) -> f64;

    # [ wasm_bindgen ( structural , method , getter , js_class = "PerformanceTiming" , js_name = fetchStart ) ]
    ///Getter for the `fetchStart` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceTiming/fetchStart)
    ///
    ///*This API requires the following crate features to be activated: `PerformanceTiming`*
    pub fn fetch_start(this: &PerformanceTiming) -> f64;

    # [ wasm_bindgen ( structural , method , getter , js_class = "PerformanceTiming" , js_name = domainLookupStart ) ]
    ///Getter for the `domainLookupStart` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceTiming/domainLookupStart)
    ///
    ///*This API requires the following crate features to be activated: `PerformanceTiming`*
    pub fn domain_lookup_start(this: &PerformanceTiming) -> f64;

    # [ wasm_bindgen ( structural , method , getter , js_class = "PerformanceTiming" , js_name = domainLookupEnd ) ]
    ///Getter for the `domainLookupEnd` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceTiming/domainLookupEnd)
    ///
    ///*This API requires the following crate features to be activated: `PerformanceTiming`*
    pub fn domain_lookup_end(this: &PerformanceTiming) -> f64;

    # [ wasm_bindgen ( structural , method , getter , js_class = "PerformanceTiming" , js_name = connectStart ) ]
    ///Getter for the `connectStart` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceTiming/connectStart)
    ///
    ///*This API requires the following crate features to be activated: `PerformanceTiming`*
    pub fn connect_start(this: &PerformanceTiming) -> f64;

    # [ wasm_bindgen ( structural , method , getter , js_class = "PerformanceTiming" , js_name = connectEnd ) ]
    ///Getter for the `connectEnd` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceTiming/connectEnd)
    ///
    ///*This API requires the following crate features to be activated: `PerformanceTiming`*
    pub fn connect_end(this: &PerformanceTiming) -> f64;

    # [ wasm_bindgen ( structural , method , getter , js_class = "PerformanceTiming" , js_name = secureConnectionStart ) ]
    ///Getter for the `secureConnectionStart` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceTiming/secureConnectionStart)
    ///
    ///*This API requires the following crate features to be activated: `PerformanceTiming`*
    pub fn secure_connection_start(this: &PerformanceTiming) -> f64;

    # [ wasm_bindgen ( structural , method , getter , js_class = "PerformanceTiming" , js_name = requestStart ) ]
    ///Getter for the `requestStart` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceTiming/requestStart)
    ///
    ///*This API requires the following crate features to be activated: `PerformanceTiming`*
    pub fn request_start(this: &PerformanceTiming) -> f64;

    # [ wasm_bindgen ( structural , method , getter , js_class = "PerformanceTiming" , js_name = responseStart ) ]
    ///Getter for the `responseStart` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceTiming/responseStart)
    ///
    ///*This API requires the following crate features to be activated: `PerformanceTiming`*
    pub fn response_start(this: &PerformanceTiming) -> f64;

    # [ wasm_bindgen ( structural , method , getter , js_class = "PerformanceTiming" , js_name = responseEnd ) ]
    ///Getter for the `responseEnd` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceTiming/responseEnd)
    ///
    ///*This API requires the following crate features to be activated: `PerformanceTiming`*
    pub fn response_end(this: &PerformanceTiming) -> f64;

    # [ wasm_bindgen ( structural , method , getter , js_class = "PerformanceTiming" , js_name = domLoading ) ]
    ///Getter for the `domLoading` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceTiming/domLoading)
    ///
    ///*This API requires the following crate features to be activated: `PerformanceTiming`*
    pub fn dom_loading(this: &PerformanceTiming) -> f64;

    # [ wasm_bindgen ( structural , method , getter , js_class = "PerformanceTiming" , js_name = domInteractive ) ]
    ///Getter for the `domInteractive` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceTiming/domInteractive)
    ///
    ///*This API requires the following crate features to be activated: `PerformanceTiming`*
    pub fn dom_interactive(this: &PerformanceTiming) -> f64;

    # [ wasm_bindgen ( structural , method , getter , js_class = "PerformanceTiming" , js_name = domContentLoadedEventStart ) ]
    ///Getter for the `domContentLoadedEventStart` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceTiming/domContentLoadedEventStart)
    ///
    ///*This API requires the following crate features to be activated: `PerformanceTiming`*
    pub fn dom_content_loaded_event_start(this: &PerformanceTiming) -> f64;

    # [ wasm_bindgen ( structural , method , getter , js_class = "PerformanceTiming" , js_name = domContentLoadedEventEnd ) ]
    ///Getter for the `domContentLoadedEventEnd` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceTiming/domContentLoadedEventEnd)
    ///
    ///*This API requires the following crate features to be activated: `PerformanceTiming`*
    pub fn dom_content_loaded_event_end(this: &PerformanceTiming) -> f64;

    # [ wasm_bindgen ( structural , method , getter , js_class = "PerformanceTiming" , js_name = domComplete ) ]
    ///Getter for the `domComplete` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceTiming/domComplete)
    ///
    ///*This API requires the following crate features to be activated: `PerformanceTiming`*
    pub fn dom_complete(this: &PerformanceTiming) -> f64;

    # [ wasm_bindgen ( structural , method , getter , js_class = "PerformanceTiming" , js_name = loadEventStart ) ]
    ///Getter for the `loadEventStart` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceTiming/loadEventStart)
    ///
    ///*This API requires the following crate features to be activated: `PerformanceTiming`*
    pub fn load_event_start(this: &PerformanceTiming) -> f64;

    # [ wasm_bindgen ( structural , method , getter , js_class = "PerformanceTiming" , js_name = loadEventEnd ) ]
    ///Getter for the `loadEventEnd` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceTiming/loadEventEnd)
    ///
    ///*This API requires the following crate features to be activated: `PerformanceTiming`*
    pub fn load_event_end(this: &PerformanceTiming) -> f64;

    # [ wasm_bindgen ( structural , method , getter , js_class = "PerformanceTiming" , js_name = timeToNonBlankPaint ) ]
    ///Getter for the `timeToNonBlankPaint` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceTiming/timeToNonBlankPaint)
    ///
    ///*This API requires the following crate features to be activated: `PerformanceTiming`*
    pub fn time_to_non_blank_paint(this: &PerformanceTiming) -> f64;

    # [ wasm_bindgen ( structural , method , getter , js_class = "PerformanceTiming" , js_name = timeToDOMContentFlushed ) ]
    ///Getter for the `timeToDOMContentFlushed` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceTiming/timeToDOMContentFlushed)
    ///
    ///*This API requires the following crate features to be activated: `PerformanceTiming`*
    pub fn time_to_dom_content_flushed(this: &PerformanceTiming) -> f64;

    # [ wasm_bindgen ( method , structural , js_class = "PerformanceTiming" , js_name = toJSON ) ]
    ///The `toJSON()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceTiming/toJSON)
    ///
    ///*This API requires the following crate features to be activated: `PerformanceTiming`*
    pub fn to_json(this: &PerformanceTiming) -> ::js_sys::Object;

}
