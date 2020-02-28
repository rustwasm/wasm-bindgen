use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = PerformanceEntry , extends = :: js_sys :: Object , js_name = PerformanceResourceTiming , typescript_name = PerformanceResourceTiming ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PerformanceResourceTiming` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceResourceTiming)\n\n*This API requires the following crate features to be activated: `PerformanceResourceTiming`*"]
    pub type PerformanceResourceTiming;
    # [ wasm_bindgen ( structural , method , getter , js_name = initiatorType ) ]
    #[doc = "Getter for the `initiatorType` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceResourceTiming/initiatorType)\n\n*This API requires the following crate features to be activated: `PerformanceResourceTiming`*"]
    pub fn initiator_type(this: &PerformanceResourceTiming) -> String;
    # [ wasm_bindgen ( structural , method , getter , js_name = nextHopProtocol ) ]
    #[doc = "Getter for the `nextHopProtocol` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceResourceTiming/nextHopProtocol)\n\n*This API requires the following crate features to be activated: `PerformanceResourceTiming`*"]
    pub fn next_hop_protocol(this: &PerformanceResourceTiming) -> String;
    # [ wasm_bindgen ( structural , method , getter , js_name = workerStart ) ]
    #[doc = "Getter for the `workerStart` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceResourceTiming/workerStart)\n\n*This API requires the following crate features to be activated: `PerformanceResourceTiming`*"]
    pub fn worker_start(this: &PerformanceResourceTiming) -> f64;
    # [ wasm_bindgen ( structural , method , getter , js_name = redirectStart ) ]
    #[doc = "Getter for the `redirectStart` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceResourceTiming/redirectStart)\n\n*This API requires the following crate features to be activated: `PerformanceResourceTiming`*"]
    pub fn redirect_start(this: &PerformanceResourceTiming) -> f64;
    # [ wasm_bindgen ( structural , method , getter , js_name = redirectEnd ) ]
    #[doc = "Getter for the `redirectEnd` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceResourceTiming/redirectEnd)\n\n*This API requires the following crate features to be activated: `PerformanceResourceTiming`*"]
    pub fn redirect_end(this: &PerformanceResourceTiming) -> f64;
    # [ wasm_bindgen ( structural , method , getter , js_name = fetchStart ) ]
    #[doc = "Getter for the `fetchStart` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceResourceTiming/fetchStart)\n\n*This API requires the following crate features to be activated: `PerformanceResourceTiming`*"]
    pub fn fetch_start(this: &PerformanceResourceTiming) -> f64;
    # [ wasm_bindgen ( structural , method , getter , js_name = domainLookupStart ) ]
    #[doc = "Getter for the `domainLookupStart` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceResourceTiming/domainLookupStart)\n\n*This API requires the following crate features to be activated: `PerformanceResourceTiming`*"]
    pub fn domain_lookup_start(this: &PerformanceResourceTiming) -> f64;
    # [ wasm_bindgen ( structural , method , getter , js_name = domainLookupEnd ) ]
    #[doc = "Getter for the `domainLookupEnd` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceResourceTiming/domainLookupEnd)\n\n*This API requires the following crate features to be activated: `PerformanceResourceTiming`*"]
    pub fn domain_lookup_end(this: &PerformanceResourceTiming) -> f64;
    # [ wasm_bindgen ( structural , method , getter , js_name = connectStart ) ]
    #[doc = "Getter for the `connectStart` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceResourceTiming/connectStart)\n\n*This API requires the following crate features to be activated: `PerformanceResourceTiming`*"]
    pub fn connect_start(this: &PerformanceResourceTiming) -> f64;
    # [ wasm_bindgen ( structural , method , getter , js_name = connectEnd ) ]
    #[doc = "Getter for the `connectEnd` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceResourceTiming/connectEnd)\n\n*This API requires the following crate features to be activated: `PerformanceResourceTiming`*"]
    pub fn connect_end(this: &PerformanceResourceTiming) -> f64;
    # [ wasm_bindgen ( structural , method , getter , js_name = secureConnectionStart ) ]
    #[doc = "Getter for the `secureConnectionStart` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceResourceTiming/secureConnectionStart)\n\n*This API requires the following crate features to be activated: `PerformanceResourceTiming`*"]
    pub fn secure_connection_start(this: &PerformanceResourceTiming) -> f64;
    # [ wasm_bindgen ( structural , method , getter , js_name = requestStart ) ]
    #[doc = "Getter for the `requestStart` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceResourceTiming/requestStart)\n\n*This API requires the following crate features to be activated: `PerformanceResourceTiming`*"]
    pub fn request_start(this: &PerformanceResourceTiming) -> f64;
    # [ wasm_bindgen ( structural , method , getter , js_name = responseStart ) ]
    #[doc = "Getter for the `responseStart` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceResourceTiming/responseStart)\n\n*This API requires the following crate features to be activated: `PerformanceResourceTiming`*"]
    pub fn response_start(this: &PerformanceResourceTiming) -> f64;
    # [ wasm_bindgen ( structural , method , getter , js_name = responseEnd ) ]
    #[doc = "Getter for the `responseEnd` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceResourceTiming/responseEnd)\n\n*This API requires the following crate features to be activated: `PerformanceResourceTiming`*"]
    pub fn response_end(this: &PerformanceResourceTiming) -> f64;
    # [ wasm_bindgen ( structural , method , getter , js_name = transferSize ) ]
    #[doc = "Getter for the `transferSize` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceResourceTiming/transferSize)\n\n*This API requires the following crate features to be activated: `PerformanceResourceTiming`*"]
    pub fn transfer_size(this: &PerformanceResourceTiming) -> f64;
    # [ wasm_bindgen ( structural , method , getter , js_name = encodedBodySize ) ]
    #[doc = "Getter for the `encodedBodySize` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceResourceTiming/encodedBodySize)\n\n*This API requires the following crate features to be activated: `PerformanceResourceTiming`*"]
    pub fn encoded_body_size(this: &PerformanceResourceTiming) -> f64;
    # [ wasm_bindgen ( structural , method , getter , js_name = decodedBodySize ) ]
    #[doc = "Getter for the `decodedBodySize` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceResourceTiming/decodedBodySize)\n\n*This API requires the following crate features to be activated: `PerformanceResourceTiming`*"]
    pub fn decoded_body_size(this: &PerformanceResourceTiming) -> f64;
    # [ wasm_bindgen ( structural , method , getter , js_name = serverTiming ) ]
    #[doc = "Getter for the `serverTiming` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceResourceTiming/serverTiming)\n\n*This API requires the following crate features to be activated: `PerformanceResourceTiming`*"]
    pub fn server_timing(this: &PerformanceResourceTiming) -> ::js_sys::Array;
    # [ wasm_bindgen ( method , structural , js_name = toJSON ) ]
    #[doc = "The `toJSON()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceResourceTiming/toJSON)\n\n*This API requires the following crate features to be activated: `PerformanceResourceTiming`*"]
    pub fn to_json(this: &PerformanceResourceTiming) -> ::js_sys::Object;
}
