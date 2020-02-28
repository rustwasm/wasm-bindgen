use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = SvgGeometryElement , extends = SvgGraphicsElement , extends = SvgElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = SVGRectElement , typescript_name = SVGRectElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgRectElement` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGRectElement)\n\n*This API requires the following crate features to be activated: `SvgRectElement`*"]
    pub type SvgRectElement;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGRectElement" , js_name = x ) ]
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `x` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGRectElement/x)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgRectElement`*"]
    pub fn x(this: &SvgRectElement) -> SvgAnimatedLength;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGRectElement" , js_name = y ) ]
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `y` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGRectElement/y)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgRectElement`*"]
    pub fn y(this: &SvgRectElement) -> SvgAnimatedLength;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGRectElement" , js_name = width ) ]
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `width` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGRectElement/width)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgRectElement`*"]
    pub fn width(this: &SvgRectElement) -> SvgAnimatedLength;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGRectElement" , js_name = height ) ]
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `height` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGRectElement/height)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgRectElement`*"]
    pub fn height(this: &SvgRectElement) -> SvgAnimatedLength;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGRectElement" , js_name = rx ) ]
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `rx` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGRectElement/rx)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgRectElement`*"]
    pub fn rx(this: &SvgRectElement) -> SvgAnimatedLength;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGRectElement" , js_name = ry ) ]
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `ry` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGRectElement/ry)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgRectElement`*"]
    pub fn ry(this: &SvgRectElement) -> SvgAnimatedLength;
}
