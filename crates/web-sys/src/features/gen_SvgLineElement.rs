use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = SvgGeometryElement , extends = SvgGraphicsElement , extends = SvgElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = SVGLineElement , typescript_name = SVGLineElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgLineElement` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGLineElement)\n\n*This API requires the following crate features to be activated: `SvgLineElement`*"]
    pub type SvgLineElement;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGLineElement" , js_name = x1 ) ]
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `x1` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGLineElement/x1)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgLineElement`*"]
    pub fn x1(this: &SvgLineElement) -> SvgAnimatedLength;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGLineElement" , js_name = y1 ) ]
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `y1` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGLineElement/y1)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgLineElement`*"]
    pub fn y1(this: &SvgLineElement) -> SvgAnimatedLength;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGLineElement" , js_name = x2 ) ]
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `x2` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGLineElement/x2)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgLineElement`*"]
    pub fn x2(this: &SvgLineElement) -> SvgAnimatedLength;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGLineElement" , js_name = y2 ) ]
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `y2` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGLineElement/y2)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgLineElement`*"]
    pub fn y2(this: &SvgLineElement) -> SvgAnimatedLength;
}
