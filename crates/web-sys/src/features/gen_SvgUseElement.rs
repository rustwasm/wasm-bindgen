use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = SvgGraphicsElement , extends = SvgElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = SVGUseElement , typescript_name = SVGUseElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgUseElement` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGUseElement)\n\n*This API requires the following crate features to be activated: `SvgUseElement`*"]
    pub type SvgUseElement;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGUseElement" , js_name = x ) ]
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `x` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGUseElement/x)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgUseElement`*"]
    pub fn x(this: &SvgUseElement) -> SvgAnimatedLength;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGUseElement" , js_name = y ) ]
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `y` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGUseElement/y)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgUseElement`*"]
    pub fn y(this: &SvgUseElement) -> SvgAnimatedLength;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGUseElement" , js_name = width ) ]
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `width` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGUseElement/width)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgUseElement`*"]
    pub fn width(this: &SvgUseElement) -> SvgAnimatedLength;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGUseElement" , js_name = height ) ]
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `height` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGUseElement/height)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgUseElement`*"]
    pub fn height(this: &SvgUseElement) -> SvgAnimatedLength;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGUseElement" , js_name = href ) ]
    #[cfg(feature = "SvgAnimatedString")]
    #[doc = "Getter for the `href` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGUseElement/href)\n\n*This API requires the following crate features to be activated: `SvgAnimatedString`, `SvgUseElement`*"]
    pub fn href(this: &SvgUseElement) -> SvgAnimatedString;
}
