use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = PerformanceObserver , typescript_type = "PerformanceObserver" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `PerformanceObserver` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceObserver)
    ///
    ///*This API requires the following crate features to be activated: `PerformanceObserver`*
    pub type PerformanceObserver;

    #[wasm_bindgen(catch, constructor, js_class = "PerformanceObserver")]
    ///The `new PerformanceObserver(..)` constructor, creating a new instance of `PerformanceObserver`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceObserver/PerformanceObserver)
    ///
    ///*This API requires the following crate features to be activated: `PerformanceObserver`*
    pub fn new(callback: &::js_sys::Function) -> Result<PerformanceObserver, JsValue>;

    # [ wasm_bindgen ( method , structural , js_class = "PerformanceObserver" , js_name = disconnect ) ]
    ///The `disconnect()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceObserver/disconnect)
    ///
    ///*This API requires the following crate features to be activated: `PerformanceObserver`*
    pub fn disconnect(this: &PerformanceObserver);

    #[cfg(feature = "PerformanceObserverInit")]
    # [ wasm_bindgen ( method , structural , js_class = "PerformanceObserver" , js_name = observe ) ]
    ///The `observe()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceObserver/observe)
    ///
    ///*This API requires the following crate features to be activated: `PerformanceObserver`, `PerformanceObserverInit`*
    pub fn observe(this: &PerformanceObserver, options: &PerformanceObserverInit);

    # [ wasm_bindgen ( method , structural , js_class = "PerformanceObserver" , js_name = takeRecords ) ]
    ///The `takeRecords()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceObserver/takeRecords)
    ///
    ///*This API requires the following crate features to be activated: `PerformanceObserver`*
    pub fn take_records(this: &PerformanceObserver) -> ::js_sys::Array;

}
