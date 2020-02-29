use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = PerformanceServerTiming , typescript_type = "PerformanceServerTiming" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `PerformanceServerTiming` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceServerTiming)
    ///
    ///*This API requires the following crate features to be activated: `PerformanceServerTiming`*
    pub type PerformanceServerTiming;

    # [ wasm_bindgen ( structural , method , getter , js_class = "PerformanceServerTiming" , js_name = name ) ]
    ///Getter for the `name` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceServerTiming/name)
    ///
    ///*This API requires the following crate features to be activated: `PerformanceServerTiming`*
    pub fn name(this: &PerformanceServerTiming) -> String;

    # [ wasm_bindgen ( structural , method , getter , js_class = "PerformanceServerTiming" , js_name = duration ) ]
    ///Getter for the `duration` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceServerTiming/duration)
    ///
    ///*This API requires the following crate features to be activated: `PerformanceServerTiming`*
    pub fn duration(this: &PerformanceServerTiming) -> f64;

    # [ wasm_bindgen ( structural , method , getter , js_class = "PerformanceServerTiming" , js_name = description ) ]
    ///Getter for the `description` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceServerTiming/description)
    ///
    ///*This API requires the following crate features to be activated: `PerformanceServerTiming`*
    pub fn description(this: &PerformanceServerTiming) -> String;

    # [ wasm_bindgen ( method , structural , js_class = "PerformanceServerTiming" , js_name = toJSON ) ]
    ///The `toJSON()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceServerTiming/toJSON)
    ///
    ///*This API requires the following crate features to be activated: `PerformanceServerTiming`*
    pub fn to_json(this: &PerformanceServerTiming) -> ::js_sys::Object;

}
