use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = SvgGeometryElement , extends = SvgGraphicsElement , extends = SvgElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = SVGCircleElement , typescript_name = SVGCircleElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgCircleElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGCircleElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgCircleElement`*"]
    pub type SvgCircleElement;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGCircleElement" , js_name = cx ) ]
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `cx` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGCircleElement/cx)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgCircleElement`*"]
    pub fn cx(this: &SvgCircleElement) -> SvgAnimatedLength;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGCircleElement" , js_name = cy ) ]
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `cy` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGCircleElement/cy)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgCircleElement`*"]
    pub fn cy(this: &SvgCircleElement) -> SvgAnimatedLength;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGCircleElement" , js_name = r ) ]
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `r` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGCircleElement/r)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgCircleElement`*"]
    pub fn r(this: &SvgCircleElement) -> SvgAnimatedLength;
}
