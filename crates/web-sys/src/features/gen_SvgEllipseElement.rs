use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = SvgGeometryElement , extends = SvgGraphicsElement , extends = SvgElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = SVGEllipseElement , typescript_name = SVGEllipseElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgEllipseElement` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGEllipseElement)\n\n*This API requires the following crate features to be activated: `SvgEllipseElement`*"]
    pub type SvgEllipseElement;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGEllipseElement" , js_name = cx ) ]
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `cx` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGEllipseElement/cx)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgEllipseElement`*"]
    pub fn cx(this: &SvgEllipseElement) -> SvgAnimatedLength;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGEllipseElement" , js_name = cy ) ]
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `cy` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGEllipseElement/cy)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgEllipseElement`*"]
    pub fn cy(this: &SvgEllipseElement) -> SvgAnimatedLength;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGEllipseElement" , js_name = rx ) ]
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `rx` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGEllipseElement/rx)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgEllipseElement`*"]
    pub fn rx(this: &SvgEllipseElement) -> SvgAnimatedLength;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGEllipseElement" , js_name = ry ) ]
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `ry` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGEllipseElement/ry)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgEllipseElement`*"]
    pub fn ry(this: &SvgEllipseElement) -> SvgAnimatedLength;
}
