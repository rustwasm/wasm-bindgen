use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = PerformanceEntry , typescript_type = "PerformanceEntry" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `PerformanceEntry` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceEntry)
    ///
    ///*This API requires the following crate features to be activated: `PerformanceEntry`*
    pub type PerformanceEntry;

    # [ wasm_bindgen ( structural , method , getter , js_class = "PerformanceEntry" , js_name = name ) ]
    ///Getter for the `name` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceEntry/name)
    ///
    ///*This API requires the following crate features to be activated: `PerformanceEntry`*
    pub fn name(this: &PerformanceEntry) -> String;

    # [ wasm_bindgen ( structural , method , getter , js_class = "PerformanceEntry" , js_name = entryType ) ]
    ///Getter for the `entryType` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceEntry/entryType)
    ///
    ///*This API requires the following crate features to be activated: `PerformanceEntry`*
    pub fn entry_type(this: &PerformanceEntry) -> String;

    # [ wasm_bindgen ( structural , method , getter , js_class = "PerformanceEntry" , js_name = startTime ) ]
    ///Getter for the `startTime` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceEntry/startTime)
    ///
    ///*This API requires the following crate features to be activated: `PerformanceEntry`*
    pub fn start_time(this: &PerformanceEntry) -> f64;

    # [ wasm_bindgen ( structural , method , getter , js_class = "PerformanceEntry" , js_name = duration ) ]
    ///Getter for the `duration` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceEntry/duration)
    ///
    ///*This API requires the following crate features to be activated: `PerformanceEntry`*
    pub fn duration(this: &PerformanceEntry) -> f64;

    # [ wasm_bindgen ( method , structural , js_class = "PerformanceEntry" , js_name = toJSON ) ]
    ///The `toJSON()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceEntry/toJSON)
    ///
    ///*This API requires the following crate features to be activated: `PerformanceEntry`*
    pub fn to_json(this: &PerformanceEntry) -> ::js_sys::Object;

}
