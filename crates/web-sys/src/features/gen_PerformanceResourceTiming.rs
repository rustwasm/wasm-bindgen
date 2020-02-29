use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = PerformanceEntry , extends = :: js_sys :: Object , js_name = PerformanceResourceTiming , typescript_type = "PerformanceResourceTiming" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `PerformanceResourceTiming` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceResourceTiming)
    ///
    ///*This API requires the following crate features to be activated: `PerformanceResourceTiming`*
    pub type PerformanceResourceTiming;

    # [ wasm_bindgen ( structural , method , getter , js_class = "PerformanceResourceTiming" , js_name = initiatorType ) ]
    ///Getter for the `initiatorType` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceResourceTiming/initiatorType)
    ///
    ///*This API requires the following crate features to be activated: `PerformanceResourceTiming`*
    pub fn initiator_type(this: &PerformanceResourceTiming) -> String;

    # [ wasm_bindgen ( structural , method , getter , js_class = "PerformanceResourceTiming" , js_name = nextHopProtocol ) ]
    ///Getter for the `nextHopProtocol` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceResourceTiming/nextHopProtocol)
    ///
    ///*This API requires the following crate features to be activated: `PerformanceResourceTiming`*
    pub fn next_hop_protocol(this: &PerformanceResourceTiming) -> String;

    # [ wasm_bindgen ( structural , method , getter , js_class = "PerformanceResourceTiming" , js_name = workerStart ) ]
    ///Getter for the `workerStart` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceResourceTiming/workerStart)
    ///
    ///*This API requires the following crate features to be activated: `PerformanceResourceTiming`*
    pub fn worker_start(this: &PerformanceResourceTiming) -> f64;

    # [ wasm_bindgen ( structural , method , getter , js_class = "PerformanceResourceTiming" , js_name = redirectStart ) ]
    ///Getter for the `redirectStart` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceResourceTiming/redirectStart)
    ///
    ///*This API requires the following crate features to be activated: `PerformanceResourceTiming`*
    pub fn redirect_start(this: &PerformanceResourceTiming) -> f64;

    # [ wasm_bindgen ( structural , method , getter , js_class = "PerformanceResourceTiming" , js_name = redirectEnd ) ]
    ///Getter for the `redirectEnd` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceResourceTiming/redirectEnd)
    ///
    ///*This API requires the following crate features to be activated: `PerformanceResourceTiming`*
    pub fn redirect_end(this: &PerformanceResourceTiming) -> f64;

    # [ wasm_bindgen ( structural , method , getter , js_class = "PerformanceResourceTiming" , js_name = fetchStart ) ]
    ///Getter for the `fetchStart` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceResourceTiming/fetchStart)
    ///
    ///*This API requires the following crate features to be activated: `PerformanceResourceTiming`*
    pub fn fetch_start(this: &PerformanceResourceTiming) -> f64;

    # [ wasm_bindgen ( structural , method , getter , js_class = "PerformanceResourceTiming" , js_name = domainLookupStart ) ]
    ///Getter for the `domainLookupStart` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceResourceTiming/domainLookupStart)
    ///
    ///*This API requires the following crate features to be activated: `PerformanceResourceTiming`*
    pub fn domain_lookup_start(this: &PerformanceResourceTiming) -> f64;

    # [ wasm_bindgen ( structural , method , getter , js_class = "PerformanceResourceTiming" , js_name = domainLookupEnd ) ]
    ///Getter for the `domainLookupEnd` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceResourceTiming/domainLookupEnd)
    ///
    ///*This API requires the following crate features to be activated: `PerformanceResourceTiming`*
    pub fn domain_lookup_end(this: &PerformanceResourceTiming) -> f64;

    # [ wasm_bindgen ( structural , method , getter , js_class = "PerformanceResourceTiming" , js_name = connectStart ) ]
    ///Getter for the `connectStart` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceResourceTiming/connectStart)
    ///
    ///*This API requires the following crate features to be activated: `PerformanceResourceTiming`*
    pub fn connect_start(this: &PerformanceResourceTiming) -> f64;

    # [ wasm_bindgen ( structural , method , getter , js_class = "PerformanceResourceTiming" , js_name = connectEnd ) ]
    ///Getter for the `connectEnd` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceResourceTiming/connectEnd)
    ///
    ///*This API requires the following crate features to be activated: `PerformanceResourceTiming`*
    pub fn connect_end(this: &PerformanceResourceTiming) -> f64;

    # [ wasm_bindgen ( structural , method , getter , js_class = "PerformanceResourceTiming" , js_name = secureConnectionStart ) ]
    ///Getter for the `secureConnectionStart` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceResourceTiming/secureConnectionStart)
    ///
    ///*This API requires the following crate features to be activated: `PerformanceResourceTiming`*
    pub fn secure_connection_start(this: &PerformanceResourceTiming) -> f64;

    # [ wasm_bindgen ( structural , method , getter , js_class = "PerformanceResourceTiming" , js_name = requestStart ) ]
    ///Getter for the `requestStart` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceResourceTiming/requestStart)
    ///
    ///*This API requires the following crate features to be activated: `PerformanceResourceTiming`*
    pub fn request_start(this: &PerformanceResourceTiming) -> f64;

    # [ wasm_bindgen ( structural , method , getter , js_class = "PerformanceResourceTiming" , js_name = responseStart ) ]
    ///Getter for the `responseStart` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceResourceTiming/responseStart)
    ///
    ///*This API requires the following crate features to be activated: `PerformanceResourceTiming`*
    pub fn response_start(this: &PerformanceResourceTiming) -> f64;

    # [ wasm_bindgen ( structural , method , getter , js_class = "PerformanceResourceTiming" , js_name = responseEnd ) ]
    ///Getter for the `responseEnd` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceResourceTiming/responseEnd)
    ///
    ///*This API requires the following crate features to be activated: `PerformanceResourceTiming`*
    pub fn response_end(this: &PerformanceResourceTiming) -> f64;

    # [ wasm_bindgen ( structural , method , getter , js_class = "PerformanceResourceTiming" , js_name = transferSize ) ]
    ///Getter for the `transferSize` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceResourceTiming/transferSize)
    ///
    ///*This API requires the following crate features to be activated: `PerformanceResourceTiming`*
    pub fn transfer_size(this: &PerformanceResourceTiming) -> f64;

    # [ wasm_bindgen ( structural , method , getter , js_class = "PerformanceResourceTiming" , js_name = encodedBodySize ) ]
    ///Getter for the `encodedBodySize` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceResourceTiming/encodedBodySize)
    ///
    ///*This API requires the following crate features to be activated: `PerformanceResourceTiming`*
    pub fn encoded_body_size(this: &PerformanceResourceTiming) -> f64;

    # [ wasm_bindgen ( structural , method , getter , js_class = "PerformanceResourceTiming" , js_name = decodedBodySize ) ]
    ///Getter for the `decodedBodySize` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceResourceTiming/decodedBodySize)
    ///
    ///*This API requires the following crate features to be activated: `PerformanceResourceTiming`*
    pub fn decoded_body_size(this: &PerformanceResourceTiming) -> f64;

    # [ wasm_bindgen ( structural , method , getter , js_class = "PerformanceResourceTiming" , js_name = serverTiming ) ]
    ///Getter for the `serverTiming` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceResourceTiming/serverTiming)
    ///
    ///*This API requires the following crate features to be activated: `PerformanceResourceTiming`*
    pub fn server_timing(this: &PerformanceResourceTiming) -> ::js_sys::Array;

    # [ wasm_bindgen ( method , structural , js_class = "PerformanceResourceTiming" , js_name = toJSON ) ]
    ///The `toJSON()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceResourceTiming/toJSON)
    ///
    ///*This API requires the following crate features to be activated: `PerformanceResourceTiming`*
    pub fn to_json(this: &PerformanceResourceTiming) -> ::js_sys::Object;

}
