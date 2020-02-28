use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = EventTarget , extends = :: js_sys :: Object , js_name = Performance , typescript_name = Performance ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `Performance` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Performance)\n\n*This API requires the following crate features to be activated: `Performance`*"]
    pub type Performance;
    # [ wasm_bindgen ( structural , method , getter , js_name = timeOrigin ) ]
    #[doc = "Getter for the `timeOrigin` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Performance/timeOrigin)\n\n*This API requires the following crate features to be activated: `Performance`*"]
    pub fn time_origin(this: &Performance) -> f64;
    # [ wasm_bindgen ( structural , method , getter , js_name = timing ) ]
    #[cfg(feature = "PerformanceTiming")]
    #[doc = "Getter for the `timing` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Performance/timing)\n\n*This API requires the following crate features to be activated: `Performance`, `PerformanceTiming`*"]
    pub fn timing(this: &Performance) -> PerformanceTiming;
    # [ wasm_bindgen ( structural , method , getter , js_name = navigation ) ]
    #[cfg(feature = "PerformanceNavigation")]
    #[doc = "Getter for the `navigation` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Performance/navigation)\n\n*This API requires the following crate features to be activated: `Performance`, `PerformanceNavigation`*"]
    pub fn navigation(this: &Performance) -> PerformanceNavigation;
    # [ wasm_bindgen ( structural , method , getter , js_name = onresourcetimingbufferfull ) ]
    #[doc = "Getter for the `onresourcetimingbufferfull` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Performance/onresourcetimingbufferfull)\n\n*This API requires the following crate features to be activated: `Performance`*"]
    pub fn onresourcetimingbufferfull(this: &Performance) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onresourcetimingbufferfull ) ]
    #[doc = "Setter for the `onresourcetimingbufferfull` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Performance/onresourcetimingbufferfull)\n\n*This API requires the following crate features to be activated: `Performance`*"]
    pub fn set_onresourcetimingbufferfull(this: &Performance, value: Option<::js_sys::Function>);
    # [ wasm_bindgen ( method , structural , js_name = clearMarks ) ]
    #[doc = "The `clearMarks()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Performance/clearMarks)\n\n*This API requires the following crate features to be activated: `Performance`*"]
    pub fn clear_marks(this: &Performance);
    # [ wasm_bindgen ( method , structural , js_name = clearMarks ) ]
    #[doc = "The `clearMarks()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Performance/clearMarks)\n\n*This API requires the following crate features to be activated: `Performance`*"]
    pub fn clear_marks_with_mark_name(this: &Performance, mark_name: &str);
    # [ wasm_bindgen ( method , structural , js_name = clearMeasures ) ]
    #[doc = "The `clearMeasures()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Performance/clearMeasures)\n\n*This API requires the following crate features to be activated: `Performance`*"]
    pub fn clear_measures(this: &Performance);
    # [ wasm_bindgen ( method , structural , js_name = clearMeasures ) ]
    #[doc = "The `clearMeasures()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Performance/clearMeasures)\n\n*This API requires the following crate features to be activated: `Performance`*"]
    pub fn clear_measures_with_measure_name(this: &Performance, measure_name: &str);
    # [ wasm_bindgen ( method , structural , js_name = clearResourceTimings ) ]
    #[doc = "The `clearResourceTimings()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Performance/clearResourceTimings)\n\n*This API requires the following crate features to be activated: `Performance`*"]
    pub fn clear_resource_timings(this: &Performance);
    # [ wasm_bindgen ( method , structural , js_name = getEntries ) ]
    #[doc = "The `getEntries()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Performance/getEntries)\n\n*This API requires the following crate features to be activated: `Performance`*"]
    pub fn get_entries(this: &Performance) -> ::js_sys::Array;
    # [ wasm_bindgen ( method , structural , js_name = getEntriesByName ) ]
    #[doc = "The `getEntriesByName()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Performance/getEntriesByName)\n\n*This API requires the following crate features to be activated: `Performance`*"]
    pub fn get_entries_by_name(this: &Performance, name: &str) -> ::js_sys::Array;
    # [ wasm_bindgen ( method , structural , js_name = getEntriesByName ) ]
    #[doc = "The `getEntriesByName()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Performance/getEntriesByName)\n\n*This API requires the following crate features to be activated: `Performance`*"]
    pub fn get_entries_by_name_with_entry_type(
        this: &Performance,
        name: &str,
        entry_type: &str,
    ) -> ::js_sys::Array;
    # [ wasm_bindgen ( method , structural , js_name = getEntriesByType ) ]
    #[doc = "The `getEntriesByType()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Performance/getEntriesByType)\n\n*This API requires the following crate features to be activated: `Performance`*"]
    pub fn get_entries_by_type(this: &Performance, entry_type: &str) -> ::js_sys::Array;
    # [ wasm_bindgen ( catch , method , structural , js_name = mark ) ]
    #[doc = "The `mark()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Performance/mark)\n\n*This API requires the following crate features to be activated: `Performance`*"]
    pub fn mark(this: &Performance, mark_name: &str) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = measure ) ]
    #[doc = "The `measure()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Performance/measure)\n\n*This API requires the following crate features to be activated: `Performance`*"]
    pub fn measure(this: &Performance, measure_name: &str) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = measure ) ]
    #[doc = "The `measure()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Performance/measure)\n\n*This API requires the following crate features to be activated: `Performance`*"]
    pub fn measure_with_start_mark(
        this: &Performance,
        measure_name: &str,
        start_mark: &str,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = measure ) ]
    #[doc = "The `measure()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Performance/measure)\n\n*This API requires the following crate features to be activated: `Performance`*"]
    pub fn measure_with_start_mark_and_end_mark(
        this: &Performance,
        measure_name: &str,
        start_mark: &str,
        end_mark: &str,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( method , structural , js_name = now ) ]
    #[doc = "The `now()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Performance/now)\n\n*This API requires the following crate features to be activated: `Performance`*"]
    pub fn now(this: &Performance) -> f64;
    # [ wasm_bindgen ( method , structural , js_name = setResourceTimingBufferSize ) ]
    #[doc = "The `setResourceTimingBufferSize()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Performance/setResourceTimingBufferSize)\n\n*This API requires the following crate features to be activated: `Performance`*"]
    pub fn set_resource_timing_buffer_size(this: &Performance, max_size: u32);
    # [ wasm_bindgen ( method , structural , js_name = toJSON ) ]
    #[doc = "The `toJSON()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Performance/toJSON)\n\n*This API requires the following crate features to be activated: `Performance`*"]
    pub fn to_json(this: &Performance) -> ::js_sys::Object;
}
