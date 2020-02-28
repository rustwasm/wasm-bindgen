use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = IntersectionObserverEntry , typescript_name = IntersectionObserverEntry ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `IntersectionObserverEntry` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IntersectionObserverEntry)\n\n*This API requires the following crate features to be activated: `IntersectionObserverEntry`*"]
    pub type IntersectionObserverEntry;
    # [ wasm_bindgen ( structural , method , getter , js_name = time ) ]
    #[doc = "Getter for the `time` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IntersectionObserverEntry/time)\n\n*This API requires the following crate features to be activated: `IntersectionObserverEntry`*"]
    pub fn time(this: &IntersectionObserverEntry) -> f64;
    # [ wasm_bindgen ( structural , method , getter , js_name = rootBounds ) ]
    #[cfg(feature = "DomRectReadOnly")]
    #[doc = "Getter for the `rootBounds` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IntersectionObserverEntry/rootBounds)\n\n*This API requires the following crate features to be activated: `DomRectReadOnly`, `IntersectionObserverEntry`*"]
    pub fn root_bounds(this: &IntersectionObserverEntry) -> Option<DomRectReadOnly>;
    # [ wasm_bindgen ( structural , method , getter , js_name = boundingClientRect ) ]
    #[cfg(feature = "DomRectReadOnly")]
    #[doc = "Getter for the `boundingClientRect` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IntersectionObserverEntry/boundingClientRect)\n\n*This API requires the following crate features to be activated: `DomRectReadOnly`, `IntersectionObserverEntry`*"]
    pub fn bounding_client_rect(this: &IntersectionObserverEntry) -> DomRectReadOnly;
    # [ wasm_bindgen ( structural , method , getter , js_name = intersectionRect ) ]
    #[cfg(feature = "DomRectReadOnly")]
    #[doc = "Getter for the `intersectionRect` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IntersectionObserverEntry/intersectionRect)\n\n*This API requires the following crate features to be activated: `DomRectReadOnly`, `IntersectionObserverEntry`*"]
    pub fn intersection_rect(this: &IntersectionObserverEntry) -> DomRectReadOnly;
    # [ wasm_bindgen ( structural , method , getter , js_name = isIntersecting ) ]
    #[doc = "Getter for the `isIntersecting` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IntersectionObserverEntry/isIntersecting)\n\n*This API requires the following crate features to be activated: `IntersectionObserverEntry`*"]
    pub fn is_intersecting(this: &IntersectionObserverEntry) -> bool;
    # [ wasm_bindgen ( structural , method , getter , js_name = intersectionRatio ) ]
    #[doc = "Getter for the `intersectionRatio` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IntersectionObserverEntry/intersectionRatio)\n\n*This API requires the following crate features to be activated: `IntersectionObserverEntry`*"]
    pub fn intersection_ratio(this: &IntersectionObserverEntry) -> f64;
    # [ wasm_bindgen ( structural , method , getter , js_name = target ) ]
    #[cfg(feature = "Element")]
    #[doc = "Getter for the `target` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IntersectionObserverEntry/target)\n\n*This API requires the following crate features to be activated: `Element`, `IntersectionObserverEntry`*"]
    pub fn target(this: &IntersectionObserverEntry) -> Element;
}
