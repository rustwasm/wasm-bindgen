use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = PerformanceTiming , typescript_name = PerformanceTiming ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PerformanceTiming` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceTiming)\n\n*This API requires the following crate features to be activated: `PerformanceTiming`*"]
    pub type PerformanceTiming;
    # [ wasm_bindgen ( structural , method , getter , js_class = "PerformanceTiming" , js_name = navigationStart ) ]
    #[doc = "Getter for the `navigationStart` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceTiming/navigationStart)\n\n*This API requires the following crate features to be activated: `PerformanceTiming`*"]
    pub fn navigation_start(this: &PerformanceTiming) -> f64;
    # [ wasm_bindgen ( structural , method , getter , js_class = "PerformanceTiming" , js_name = unloadEventStart ) ]
    #[doc = "Getter for the `unloadEventStart` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceTiming/unloadEventStart)\n\n*This API requires the following crate features to be activated: `PerformanceTiming`*"]
    pub fn unload_event_start(this: &PerformanceTiming) -> f64;
    # [ wasm_bindgen ( structural , method , getter , js_class = "PerformanceTiming" , js_name = unloadEventEnd ) ]
    #[doc = "Getter for the `unloadEventEnd` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceTiming/unloadEventEnd)\n\n*This API requires the following crate features to be activated: `PerformanceTiming`*"]
    pub fn unload_event_end(this: &PerformanceTiming) -> f64;
    # [ wasm_bindgen ( structural , method , getter , js_class = "PerformanceTiming" , js_name = redirectStart ) ]
    #[doc = "Getter for the `redirectStart` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceTiming/redirectStart)\n\n*This API requires the following crate features to be activated: `PerformanceTiming`*"]
    pub fn redirect_start(this: &PerformanceTiming) -> f64;
    # [ wasm_bindgen ( structural , method , getter , js_class = "PerformanceTiming" , js_name = redirectEnd ) ]
    #[doc = "Getter for the `redirectEnd` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceTiming/redirectEnd)\n\n*This API requires the following crate features to be activated: `PerformanceTiming`*"]
    pub fn redirect_end(this: &PerformanceTiming) -> f64;
    # [ wasm_bindgen ( structural , method , getter , js_class = "PerformanceTiming" , js_name = fetchStart ) ]
    #[doc = "Getter for the `fetchStart` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceTiming/fetchStart)\n\n*This API requires the following crate features to be activated: `PerformanceTiming`*"]
    pub fn fetch_start(this: &PerformanceTiming) -> f64;
    # [ wasm_bindgen ( structural , method , getter , js_class = "PerformanceTiming" , js_name = domainLookupStart ) ]
    #[doc = "Getter for the `domainLookupStart` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceTiming/domainLookupStart)\n\n*This API requires the following crate features to be activated: `PerformanceTiming`*"]
    pub fn domain_lookup_start(this: &PerformanceTiming) -> f64;
    # [ wasm_bindgen ( structural , method , getter , js_class = "PerformanceTiming" , js_name = domainLookupEnd ) ]
    #[doc = "Getter for the `domainLookupEnd` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceTiming/domainLookupEnd)\n\n*This API requires the following crate features to be activated: `PerformanceTiming`*"]
    pub fn domain_lookup_end(this: &PerformanceTiming) -> f64;
    # [ wasm_bindgen ( structural , method , getter , js_class = "PerformanceTiming" , js_name = connectStart ) ]
    #[doc = "Getter for the `connectStart` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceTiming/connectStart)\n\n*This API requires the following crate features to be activated: `PerformanceTiming`*"]
    pub fn connect_start(this: &PerformanceTiming) -> f64;
    # [ wasm_bindgen ( structural , method , getter , js_class = "PerformanceTiming" , js_name = connectEnd ) ]
    #[doc = "Getter for the `connectEnd` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceTiming/connectEnd)\n\n*This API requires the following crate features to be activated: `PerformanceTiming`*"]
    pub fn connect_end(this: &PerformanceTiming) -> f64;
    # [ wasm_bindgen ( structural , method , getter , js_class = "PerformanceTiming" , js_name = secureConnectionStart ) ]
    #[doc = "Getter for the `secureConnectionStart` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceTiming/secureConnectionStart)\n\n*This API requires the following crate features to be activated: `PerformanceTiming`*"]
    pub fn secure_connection_start(this: &PerformanceTiming) -> f64;
    # [ wasm_bindgen ( structural , method , getter , js_class = "PerformanceTiming" , js_name = requestStart ) ]
    #[doc = "Getter for the `requestStart` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceTiming/requestStart)\n\n*This API requires the following crate features to be activated: `PerformanceTiming`*"]
    pub fn request_start(this: &PerformanceTiming) -> f64;
    # [ wasm_bindgen ( structural , method , getter , js_class = "PerformanceTiming" , js_name = responseStart ) ]
    #[doc = "Getter for the `responseStart` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceTiming/responseStart)\n\n*This API requires the following crate features to be activated: `PerformanceTiming`*"]
    pub fn response_start(this: &PerformanceTiming) -> f64;
    # [ wasm_bindgen ( structural , method , getter , js_class = "PerformanceTiming" , js_name = responseEnd ) ]
    #[doc = "Getter for the `responseEnd` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceTiming/responseEnd)\n\n*This API requires the following crate features to be activated: `PerformanceTiming`*"]
    pub fn response_end(this: &PerformanceTiming) -> f64;
    # [ wasm_bindgen ( structural , method , getter , js_class = "PerformanceTiming" , js_name = domLoading ) ]
    #[doc = "Getter for the `domLoading` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceTiming/domLoading)\n\n*This API requires the following crate features to be activated: `PerformanceTiming`*"]
    pub fn dom_loading(this: &PerformanceTiming) -> f64;
    # [ wasm_bindgen ( structural , method , getter , js_class = "PerformanceTiming" , js_name = domInteractive ) ]
    #[doc = "Getter for the `domInteractive` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceTiming/domInteractive)\n\n*This API requires the following crate features to be activated: `PerformanceTiming`*"]
    pub fn dom_interactive(this: &PerformanceTiming) -> f64;
    # [ wasm_bindgen ( structural , method , getter , js_class = "PerformanceTiming" , js_name = domContentLoadedEventStart ) ]
    #[doc = "Getter for the `domContentLoadedEventStart` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceTiming/domContentLoadedEventStart)\n\n*This API requires the following crate features to be activated: `PerformanceTiming`*"]
    pub fn dom_content_loaded_event_start(this: &PerformanceTiming) -> f64;
    # [ wasm_bindgen ( structural , method , getter , js_class = "PerformanceTiming" , js_name = domContentLoadedEventEnd ) ]
    #[doc = "Getter for the `domContentLoadedEventEnd` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceTiming/domContentLoadedEventEnd)\n\n*This API requires the following crate features to be activated: `PerformanceTiming`*"]
    pub fn dom_content_loaded_event_end(this: &PerformanceTiming) -> f64;
    # [ wasm_bindgen ( structural , method , getter , js_class = "PerformanceTiming" , js_name = domComplete ) ]
    #[doc = "Getter for the `domComplete` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceTiming/domComplete)\n\n*This API requires the following crate features to be activated: `PerformanceTiming`*"]
    pub fn dom_complete(this: &PerformanceTiming) -> f64;
    # [ wasm_bindgen ( structural , method , getter , js_class = "PerformanceTiming" , js_name = loadEventStart ) ]
    #[doc = "Getter for the `loadEventStart` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceTiming/loadEventStart)\n\n*This API requires the following crate features to be activated: `PerformanceTiming`*"]
    pub fn load_event_start(this: &PerformanceTiming) -> f64;
    # [ wasm_bindgen ( structural , method , getter , js_class = "PerformanceTiming" , js_name = loadEventEnd ) ]
    #[doc = "Getter for the `loadEventEnd` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceTiming/loadEventEnd)\n\n*This API requires the following crate features to be activated: `PerformanceTiming`*"]
    pub fn load_event_end(this: &PerformanceTiming) -> f64;
    # [ wasm_bindgen ( structural , method , getter , js_class = "PerformanceTiming" , js_name = timeToNonBlankPaint ) ]
    #[doc = "Getter for the `timeToNonBlankPaint` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceTiming/timeToNonBlankPaint)\n\n*This API requires the following crate features to be activated: `PerformanceTiming`*"]
    pub fn time_to_non_blank_paint(this: &PerformanceTiming) -> f64;
    # [ wasm_bindgen ( structural , method , getter , js_class = "PerformanceTiming" , js_name = timeToDOMContentFlushed ) ]
    #[doc = "Getter for the `timeToDOMContentFlushed` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceTiming/timeToDOMContentFlushed)\n\n*This API requires the following crate features to be activated: `PerformanceTiming`*"]
    pub fn time_to_dom_content_flushed(this: &PerformanceTiming) -> f64;
    # [ wasm_bindgen ( method , structural , js_class = "PerformanceTiming" , js_name = toJSON ) ]
    #[doc = "The `toJSON()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceTiming/toJSON)\n\n*This API requires the following crate features to be activated: `PerformanceTiming`*"]
    pub fn to_json(this: &PerformanceTiming) -> ::js_sys::Object;
}
