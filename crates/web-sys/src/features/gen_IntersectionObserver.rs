use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = IntersectionObserver , typescript_name = IntersectionObserver ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `IntersectionObserver` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IntersectionObserver)\n\n*This API requires the following crate features to be activated: `IntersectionObserver`*"]
    pub type IntersectionObserver;
    # [ wasm_bindgen ( structural , method , getter , js_name = root ) ]
    #[cfg(feature = "Element")]
    #[doc = "Getter for the `root` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IntersectionObserver/root)\n\n*This API requires the following crate features to be activated: `Element`, `IntersectionObserver`*"]
    pub fn root(this: &IntersectionObserver) -> Option<Element>;
    # [ wasm_bindgen ( structural , method , getter , js_name = rootMargin ) ]
    #[doc = "Getter for the `rootMargin` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IntersectionObserver/rootMargin)\n\n*This API requires the following crate features to be activated: `IntersectionObserver`*"]
    pub fn root_margin(this: &IntersectionObserver) -> String;
    # [ wasm_bindgen ( structural , method , getter , js_name = thresholds ) ]
    #[doc = "Getter for the `thresholds` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IntersectionObserver/thresholds)\n\n*This API requires the following crate features to be activated: `IntersectionObserver`*"]
    pub fn thresholds(this: &IntersectionObserver) -> ::js_sys::Array;
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new IntersectionObserver(..)` constructor, creating a new instance of `IntersectionObserver`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IntersectionObserver/IntersectionObserver)\n\n*This API requires the following crate features to be activated: `IntersectionObserver`*"]
    pub fn new(
        this: &IntersectionObserver,
        intersection_callback: &::js_sys::Function,
    ) -> Result<IntersectionObserver, JsValue>;
    #[cfg(feature = "IntersectionObserverInit")]
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new IntersectionObserver(..)` constructor, creating a new instance of `IntersectionObserver`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IntersectionObserver/IntersectionObserver)\n\n*This API requires the following crate features to be activated: `IntersectionObserver`, `IntersectionObserverInit`*"]
    pub fn new_with_options(
        this: &IntersectionObserver,
        intersection_callback: &::js_sys::Function,
        options: &IntersectionObserverInit,
    ) -> Result<IntersectionObserver, JsValue>;
    # [ wasm_bindgen ( method , structural , js_name = disconnect ) ]
    #[doc = "The `disconnect()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IntersectionObserver/disconnect)\n\n*This API requires the following crate features to be activated: `IntersectionObserver`*"]
    pub fn disconnect(this: &IntersectionObserver);
    #[cfg(feature = "Element")]
    # [ wasm_bindgen ( method , structural , js_name = observe ) ]
    #[doc = "The `observe()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IntersectionObserver/observe)\n\n*This API requires the following crate features to be activated: `Element`, `IntersectionObserver`*"]
    pub fn observe(this: &IntersectionObserver, target: &Element);
    # [ wasm_bindgen ( method , structural , js_name = takeRecords ) ]
    #[doc = "The `takeRecords()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IntersectionObserver/takeRecords)\n\n*This API requires the following crate features to be activated: `IntersectionObserver`*"]
    pub fn take_records(this: &IntersectionObserver) -> ::js_sys::Array;
    #[cfg(feature = "Element")]
    # [ wasm_bindgen ( method , structural , js_name = unobserve ) ]
    #[doc = "The `unobserve()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IntersectionObserver/unobserve)\n\n*This API requires the following crate features to be activated: `Element`, `IntersectionObserver`*"]
    pub fn unobserve(this: &IntersectionObserver, target: &Element);
}
