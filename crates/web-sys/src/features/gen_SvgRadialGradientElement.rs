use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = SvgGradientElement , extends = SvgElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = SVGRadialGradientElement , typescript_name = SVGRadialGradientElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `SvgRadialGradientElement` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGRadialGradientElement)
    ///
    ///*This API requires the following crate features to be activated: `SvgRadialGradientElement`*
    pub type SvgRadialGradientElement;

    #[cfg(feature = "SvgAnimatedLength")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGRadialGradientElement" , js_name = cx ) ]
    ///Getter for the `cx` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGRadialGradientElement/cx)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgRadialGradientElement`*
    pub fn cx(this: &SvgRadialGradientElement) -> SvgAnimatedLength;

    #[cfg(feature = "SvgAnimatedLength")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGRadialGradientElement" , js_name = cy ) ]
    ///Getter for the `cy` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGRadialGradientElement/cy)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgRadialGradientElement`*
    pub fn cy(this: &SvgRadialGradientElement) -> SvgAnimatedLength;

    #[cfg(feature = "SvgAnimatedLength")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGRadialGradientElement" , js_name = r ) ]
    ///Getter for the `r` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGRadialGradientElement/r)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgRadialGradientElement`*
    pub fn r(this: &SvgRadialGradientElement) -> SvgAnimatedLength;

    #[cfg(feature = "SvgAnimatedLength")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGRadialGradientElement" , js_name = fx ) ]
    ///Getter for the `fx` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGRadialGradientElement/fx)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgRadialGradientElement`*
    pub fn fx(this: &SvgRadialGradientElement) -> SvgAnimatedLength;

    #[cfg(feature = "SvgAnimatedLength")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGRadialGradientElement" , js_name = fy ) ]
    ///Getter for the `fy` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGRadialGradientElement/fy)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgRadialGradientElement`*
    pub fn fy(this: &SvgRadialGradientElement) -> SvgAnimatedLength;

    #[cfg(feature = "SvgAnimatedLength")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGRadialGradientElement" , js_name = fr ) ]
    ///Getter for the `fr` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGRadialGradientElement/fr)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgRadialGradientElement`*
    pub fn fr(this: &SvgRadialGradientElement) -> SvgAnimatedLength;

}
