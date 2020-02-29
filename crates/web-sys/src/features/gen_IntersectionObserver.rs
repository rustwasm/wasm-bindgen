use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = IntersectionObserver , typescript_type = "IntersectionObserver" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `IntersectionObserver` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IntersectionObserver)
    ///
    ///*This API requires the following crate features to be activated: `IntersectionObserver`*
    pub type IntersectionObserver;

    #[cfg(feature = "Element")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "IntersectionObserver" , js_name = root ) ]
    ///Getter for the `root` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IntersectionObserver/root)
    ///
    ///*This API requires the following crate features to be activated: `Element`, `IntersectionObserver`*
    pub fn root(this: &IntersectionObserver) -> Option<Element>;

    # [ wasm_bindgen ( structural , method , getter , js_class = "IntersectionObserver" , js_name = rootMargin ) ]
    ///Getter for the `rootMargin` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IntersectionObserver/rootMargin)
    ///
    ///*This API requires the following crate features to be activated: `IntersectionObserver`*
    pub fn root_margin(this: &IntersectionObserver) -> String;

    # [ wasm_bindgen ( structural , method , getter , js_class = "IntersectionObserver" , js_name = thresholds ) ]
    ///Getter for the `thresholds` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IntersectionObserver/thresholds)
    ///
    ///*This API requires the following crate features to be activated: `IntersectionObserver`*
    pub fn thresholds(this: &IntersectionObserver) -> ::js_sys::Array;

    #[wasm_bindgen(catch, constructor, js_class = "IntersectionObserver")]
    ///The `new IntersectionObserver(..)` constructor, creating a new instance of `IntersectionObserver`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IntersectionObserver/IntersectionObserver)
    ///
    ///*This API requires the following crate features to be activated: `IntersectionObserver`*
    pub fn new(intersection_callback: &::js_sys::Function)
        -> Result<IntersectionObserver, JsValue>;

    #[cfg(feature = "IntersectionObserverInit")]
    #[wasm_bindgen(catch, constructor, js_class = "IntersectionObserver")]
    ///The `new IntersectionObserver(..)` constructor, creating a new instance of `IntersectionObserver`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IntersectionObserver/IntersectionObserver)
    ///
    ///*This API requires the following crate features to be activated: `IntersectionObserver`, `IntersectionObserverInit`*
    pub fn new_with_options(
        intersection_callback: &::js_sys::Function,
        options: &IntersectionObserverInit,
    ) -> Result<IntersectionObserver, JsValue>;

    # [ wasm_bindgen ( method , structural , js_class = "IntersectionObserver" , js_name = disconnect ) ]
    ///The `disconnect()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IntersectionObserver/disconnect)
    ///
    ///*This API requires the following crate features to be activated: `IntersectionObserver`*
    pub fn disconnect(this: &IntersectionObserver);

    #[cfg(feature = "Element")]
    # [ wasm_bindgen ( method , structural , js_class = "IntersectionObserver" , js_name = observe ) ]
    ///The `observe()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IntersectionObserver/observe)
    ///
    ///*This API requires the following crate features to be activated: `Element`, `IntersectionObserver`*
    pub fn observe(this: &IntersectionObserver, target: &Element);

    # [ wasm_bindgen ( method , structural , js_class = "IntersectionObserver" , js_name = takeRecords ) ]
    ///The `takeRecords()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IntersectionObserver/takeRecords)
    ///
    ///*This API requires the following crate features to be activated: `IntersectionObserver`*
    pub fn take_records(this: &IntersectionObserver) -> ::js_sys::Array;

    #[cfg(feature = "Element")]
    # [ wasm_bindgen ( method , structural , js_class = "IntersectionObserver" , js_name = unobserve ) ]
    ///The `unobserve()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IntersectionObserver/unobserve)
    ///
    ///*This API requires the following crate features to be activated: `Element`, `IntersectionObserver`*
    pub fn unobserve(this: &IntersectionObserver, target: &Element);

}
