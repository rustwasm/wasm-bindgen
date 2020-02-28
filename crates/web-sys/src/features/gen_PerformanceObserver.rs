use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = PerformanceObserver , typescript_name = PerformanceObserver ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PerformanceObserver` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceObserver)\n\n*This API requires the following crate features to be activated: `PerformanceObserver`*"]
    pub type PerformanceObserver;
    #[wasm_bindgen(catch, js_class = "PerformanceObserver", constructor)]
    #[doc = "The `new PerformanceObserver(..)` constructor, creating a new instance of `PerformanceObserver`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceObserver/PerformanceObserver)\n\n*This API requires the following crate features to be activated: `PerformanceObserver`*"]
    pub fn new(
        this: &PerformanceObserver,
        callback: &::js_sys::Function,
    ) -> Result<PerformanceObserver, JsValue>;
    # [ wasm_bindgen ( method , structural , js_class = "PerformanceObserver" , js_name = disconnect ) ]
    #[doc = "The `disconnect()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceObserver/disconnect)\n\n*This API requires the following crate features to be activated: `PerformanceObserver`*"]
    pub fn disconnect(this: &PerformanceObserver);
    #[cfg(feature = "PerformanceObserverInit")]
    # [ wasm_bindgen ( method , structural , js_class = "PerformanceObserver" , js_name = observe ) ]
    #[doc = "The `observe()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceObserver/observe)\n\n*This API requires the following crate features to be activated: `PerformanceObserver`, `PerformanceObserverInit`*"]
    pub fn observe(this: &PerformanceObserver, options: &PerformanceObserverInit);
    # [ wasm_bindgen ( method , structural , js_class = "PerformanceObserver" , js_name = takeRecords ) ]
    #[doc = "The `takeRecords()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceObserver/takeRecords)\n\n*This API requires the following crate features to be activated: `PerformanceObserver`*"]
    pub fn take_records(this: &PerformanceObserver) -> ::js_sys::Array;
}
