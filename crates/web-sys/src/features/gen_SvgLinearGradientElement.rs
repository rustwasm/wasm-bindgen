use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = SvgGradientElement , extends = SvgElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = SVGLinearGradientElement , typescript_name = SVGLinearGradientElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgLinearGradientElement` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGLinearGradientElement)\n\n*This API requires the following crate features to be activated: `SvgLinearGradientElement`*"]
    pub type SvgLinearGradientElement;
    # [ wasm_bindgen ( structural , method , getter , js_name = x1 ) ]
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `x1` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGLinearGradientElement/x1)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgLinearGradientElement`*"]
    pub fn x1(this: &SvgLinearGradientElement) -> SvgAnimatedLength;
    # [ wasm_bindgen ( structural , method , getter , js_name = y1 ) ]
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `y1` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGLinearGradientElement/y1)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgLinearGradientElement`*"]
    pub fn y1(this: &SvgLinearGradientElement) -> SvgAnimatedLength;
    # [ wasm_bindgen ( structural , method , getter , js_name = x2 ) ]
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `x2` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGLinearGradientElement/x2)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgLinearGradientElement`*"]
    pub fn x2(this: &SvgLinearGradientElement) -> SvgAnimatedLength;
    # [ wasm_bindgen ( structural , method , getter , js_name = y2 ) ]
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `y2` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGLinearGradientElement/y2)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgLinearGradientElement`*"]
    pub fn y2(this: &SvgLinearGradientElement) -> SvgAnimatedLength;
}
