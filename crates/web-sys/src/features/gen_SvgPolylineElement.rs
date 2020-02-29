use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = SvgGeometryElement , extends = SvgGraphicsElement , extends = SvgElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = SVGPolylineElement , typescript_name = SVGPolylineElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `SvgPolylineElement` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPolylineElement)
    ///
    ///*This API requires the following crate features to be activated: `SvgPolylineElement`*
    pub type SvgPolylineElement;

    #[cfg(feature = "SvgPointList")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGPolylineElement" , js_name = points ) ]
    ///Getter for the `points` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPolylineElement/points)
    ///
    ///*This API requires the following crate features to be activated: `SvgPointList`, `SvgPolylineElement`*
    pub fn points(this: &SvgPolylineElement) -> SvgPointList;

    #[cfg(feature = "SvgPointList")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGPolylineElement" , js_name = animatedPoints ) ]
    ///Getter for the `animatedPoints` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPolylineElement/animatedPoints)
    ///
    ///*This API requires the following crate features to be activated: `SvgPointList`, `SvgPolylineElement`*
    pub fn animated_points(this: &SvgPolylineElement) -> SvgPointList;

}
