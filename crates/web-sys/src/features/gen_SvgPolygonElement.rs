use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = SvgGeometryElement , extends = SvgGraphicsElement , extends = SvgElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = SVGPolygonElement , typescript_name = SVGPolygonElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `SvgPolygonElement` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPolygonElement)
    ///
    ///*This API requires the following crate features to be activated: `SvgPolygonElement`*
    pub type SvgPolygonElement;

    #[cfg(feature = "SvgPointList")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGPolygonElement" , js_name = points ) ]
    ///Getter for the `points` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPolygonElement/points)
    ///
    ///*This API requires the following crate features to be activated: `SvgPointList`, `SvgPolygonElement`*
    pub fn points(this: &SvgPolygonElement) -> SvgPointList;

    #[cfg(feature = "SvgPointList")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGPolygonElement" , js_name = animatedPoints ) ]
    ///Getter for the `animatedPoints` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPolygonElement/animatedPoints)
    ///
    ///*This API requires the following crate features to be activated: `SvgPointList`, `SvgPolygonElement`*
    pub fn animated_points(this: &SvgPolygonElement) -> SvgPointList;

}
