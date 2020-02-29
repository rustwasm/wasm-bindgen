use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = SvgGeometryElement , extends = SvgGraphicsElement , extends = SvgElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = SVGCircleElement , typescript_type = "SVGCircleElement" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `SvgCircleElement` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGCircleElement)
    ///
    ///*This API requires the following crate features to be activated: `SvgCircleElement`*
    pub type SvgCircleElement;

    #[cfg(feature = "SvgAnimatedLength")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGCircleElement" , js_name = cx ) ]
    ///Getter for the `cx` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGCircleElement/cx)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgCircleElement`*
    pub fn cx(this: &SvgCircleElement) -> SvgAnimatedLength;

    #[cfg(feature = "SvgAnimatedLength")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGCircleElement" , js_name = cy ) ]
    ///Getter for the `cy` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGCircleElement/cy)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgCircleElement`*
    pub fn cy(this: &SvgCircleElement) -> SvgAnimatedLength;

    #[cfg(feature = "SvgAnimatedLength")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGCircleElement" , js_name = r ) ]
    ///Getter for the `r` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGCircleElement/r)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgCircleElement`*
    pub fn r(this: &SvgCircleElement) -> SvgAnimatedLength;

}
