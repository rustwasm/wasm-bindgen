use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = IntersectionObserverEntry , typescript_name = IntersectionObserverEntry ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `IntersectionObserverEntry` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IntersectionObserverEntry)
    ///
    ///*This API requires the following crate features to be activated: `IntersectionObserverEntry`*
    pub type IntersectionObserverEntry;

    # [ wasm_bindgen ( structural , method , getter , js_class = "IntersectionObserverEntry" , js_name = time ) ]
    ///Getter for the `time` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IntersectionObserverEntry/time)
    ///
    ///*This API requires the following crate features to be activated: `IntersectionObserverEntry`*
    pub fn time(this: &IntersectionObserverEntry) -> f64;

    #[cfg(feature = "DomRectReadOnly")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "IntersectionObserverEntry" , js_name = rootBounds ) ]
    ///Getter for the `rootBounds` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IntersectionObserverEntry/rootBounds)
    ///
    ///*This API requires the following crate features to be activated: `DomRectReadOnly`, `IntersectionObserverEntry`*
    pub fn root_bounds(this: &IntersectionObserverEntry) -> Option<DomRectReadOnly>;

    #[cfg(feature = "DomRectReadOnly")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "IntersectionObserverEntry" , js_name = boundingClientRect ) ]
    ///Getter for the `boundingClientRect` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IntersectionObserverEntry/boundingClientRect)
    ///
    ///*This API requires the following crate features to be activated: `DomRectReadOnly`, `IntersectionObserverEntry`*
    pub fn bounding_client_rect(this: &IntersectionObserverEntry) -> DomRectReadOnly;

    #[cfg(feature = "DomRectReadOnly")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "IntersectionObserverEntry" , js_name = intersectionRect ) ]
    ///Getter for the `intersectionRect` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IntersectionObserverEntry/intersectionRect)
    ///
    ///*This API requires the following crate features to be activated: `DomRectReadOnly`, `IntersectionObserverEntry`*
    pub fn intersection_rect(this: &IntersectionObserverEntry) -> DomRectReadOnly;

    # [ wasm_bindgen ( structural , method , getter , js_class = "IntersectionObserverEntry" , js_name = isIntersecting ) ]
    ///Getter for the `isIntersecting` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IntersectionObserverEntry/isIntersecting)
    ///
    ///*This API requires the following crate features to be activated: `IntersectionObserverEntry`*
    pub fn is_intersecting(this: &IntersectionObserverEntry) -> bool;

    # [ wasm_bindgen ( structural , method , getter , js_class = "IntersectionObserverEntry" , js_name = intersectionRatio ) ]
    ///Getter for the `intersectionRatio` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IntersectionObserverEntry/intersectionRatio)
    ///
    ///*This API requires the following crate features to be activated: `IntersectionObserverEntry`*
    pub fn intersection_ratio(this: &IntersectionObserverEntry) -> f64;

    #[cfg(feature = "Element")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "IntersectionObserverEntry" , js_name = target ) ]
    ///Getter for the `target` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IntersectionObserverEntry/target)
    ///
    ///*This API requires the following crate features to be activated: `Element`, `IntersectionObserverEntry`*
    pub fn target(this: &IntersectionObserverEntry) -> Element;

}
