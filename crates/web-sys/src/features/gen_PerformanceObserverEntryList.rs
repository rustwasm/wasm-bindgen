use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = PerformanceObserverEntryList , typescript_name = PerformanceObserverEntryList ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `PerformanceObserverEntryList` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceObserverEntryList)
    ///
    ///*This API requires the following crate features to be activated: `PerformanceObserverEntryList`*
    pub type PerformanceObserverEntryList;

    # [ wasm_bindgen ( method , structural , js_class = "PerformanceObserverEntryList" , js_name = getEntries ) ]
    ///The `getEntries()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceObserverEntryList/getEntries)
    ///
    ///*This API requires the following crate features to be activated: `PerformanceObserverEntryList`*
    pub fn get_entries(this: &PerformanceObserverEntryList) -> ::js_sys::Array;

    #[cfg(feature = "PerformanceEntryFilterOptions")]
    # [ wasm_bindgen ( method , structural , js_class = "PerformanceObserverEntryList" , js_name = getEntries ) ]
    ///The `getEntries()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceObserverEntryList/getEntries)
    ///
    ///*This API requires the following crate features to be activated: `PerformanceEntryFilterOptions`, `PerformanceObserverEntryList`*
    pub fn get_entries_with_filter(
        this: &PerformanceObserverEntryList,
        filter: &PerformanceEntryFilterOptions,
    ) -> ::js_sys::Array;

    # [ wasm_bindgen ( method , structural , js_class = "PerformanceObserverEntryList" , js_name = getEntriesByName ) ]
    ///The `getEntriesByName()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceObserverEntryList/getEntriesByName)
    ///
    ///*This API requires the following crate features to be activated: `PerformanceObserverEntryList`*
    pub fn get_entries_by_name(this: &PerformanceObserverEntryList, name: &str) -> ::js_sys::Array;

    # [ wasm_bindgen ( method , structural , js_class = "PerformanceObserverEntryList" , js_name = getEntriesByName ) ]
    ///The `getEntriesByName()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceObserverEntryList/getEntriesByName)
    ///
    ///*This API requires the following crate features to be activated: `PerformanceObserverEntryList`*
    pub fn get_entries_by_name_with_entry_type(
        this: &PerformanceObserverEntryList,
        name: &str,
        entry_type: &str,
    ) -> ::js_sys::Array;

    # [ wasm_bindgen ( method , structural , js_class = "PerformanceObserverEntryList" , js_name = getEntriesByType ) ]
    ///The `getEntriesByType()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceObserverEntryList/getEntriesByType)
    ///
    ///*This API requires the following crate features to be activated: `PerformanceObserverEntryList`*
    pub fn get_entries_by_type(
        this: &PerformanceObserverEntryList,
        entry_type: &str,
    ) -> ::js_sys::Array;

}
