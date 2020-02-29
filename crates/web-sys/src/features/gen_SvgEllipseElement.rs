use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = SvgGeometryElement , extends = SvgGraphicsElement , extends = SvgElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = SVGEllipseElement , typescript_type = "SVGEllipseElement" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `SvgEllipseElement` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGEllipseElement)
    ///
    ///*This API requires the following crate features to be activated: `SvgEllipseElement`*
    pub type SvgEllipseElement;

    #[cfg(feature = "SvgAnimatedLength")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGEllipseElement" , js_name = cx ) ]
    ///Getter for the `cx` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGEllipseElement/cx)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgEllipseElement`*
    pub fn cx(this: &SvgEllipseElement) -> SvgAnimatedLength;

    #[cfg(feature = "SvgAnimatedLength")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGEllipseElement" , js_name = cy ) ]
    ///Getter for the `cy` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGEllipseElement/cy)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgEllipseElement`*
    pub fn cy(this: &SvgEllipseElement) -> SvgAnimatedLength;

    #[cfg(feature = "SvgAnimatedLength")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGEllipseElement" , js_name = rx ) ]
    ///Getter for the `rx` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGEllipseElement/rx)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgEllipseElement`*
    pub fn rx(this: &SvgEllipseElement) -> SvgAnimatedLength;

    #[cfg(feature = "SvgAnimatedLength")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGEllipseElement" , js_name = ry ) ]
    ///Getter for the `ry` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGEllipseElement/ry)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgEllipseElement`*
    pub fn ry(this: &SvgEllipseElement) -> SvgAnimatedLength;

}
