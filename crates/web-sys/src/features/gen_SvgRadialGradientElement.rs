use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = SvgGradientElement , extends = SvgElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = SVGRadialGradientElement , typescript_name = SVGRadialGradientElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgRadialGradientElement` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGRadialGradientElement)\n\n*This API requires the following crate features to be activated: `SvgRadialGradientElement`*"]
    pub type SvgRadialGradientElement;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGRadialGradientElement" , js_name = cx ) ]
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `cx` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGRadialGradientElement/cx)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgRadialGradientElement`*"]
    pub fn cx(this: &SvgRadialGradientElement) -> SvgAnimatedLength;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGRadialGradientElement" , js_name = cy ) ]
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `cy` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGRadialGradientElement/cy)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgRadialGradientElement`*"]
    pub fn cy(this: &SvgRadialGradientElement) -> SvgAnimatedLength;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGRadialGradientElement" , js_name = r ) ]
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `r` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGRadialGradientElement/r)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgRadialGradientElement`*"]
    pub fn r(this: &SvgRadialGradientElement) -> SvgAnimatedLength;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGRadialGradientElement" , js_name = fx ) ]
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `fx` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGRadialGradientElement/fx)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgRadialGradientElement`*"]
    pub fn fx(this: &SvgRadialGradientElement) -> SvgAnimatedLength;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGRadialGradientElement" , js_name = fy ) ]
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `fy` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGRadialGradientElement/fy)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgRadialGradientElement`*"]
    pub fn fy(this: &SvgRadialGradientElement) -> SvgAnimatedLength;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGRadialGradientElement" , js_name = fr ) ]
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `fr` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGRadialGradientElement/fr)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgRadialGradientElement`*"]
    pub fn fr(this: &SvgRadialGradientElement) -> SvgAnimatedLength;
}
