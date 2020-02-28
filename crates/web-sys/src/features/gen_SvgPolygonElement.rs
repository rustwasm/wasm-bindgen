use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = SvgGeometryElement , extends = SvgGraphicsElement , extends = SvgElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = SVGPolygonElement , typescript_name = SVGPolygonElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgPolygonElement` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPolygonElement)\n\n*This API requires the following crate features to be activated: `SvgPolygonElement`*"]
    pub type SvgPolygonElement;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGPolygonElement" , js_name = points ) ]
    #[cfg(feature = "SvgPointList")]
    #[doc = "Getter for the `points` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPolygonElement/points)\n\n*This API requires the following crate features to be activated: `SvgPointList`, `SvgPolygonElement`*"]
    pub fn points(this: &SvgPolygonElement) -> SvgPointList;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGPolygonElement" , js_name = animatedPoints ) ]
    #[cfg(feature = "SvgPointList")]
    #[doc = "Getter for the `animatedPoints` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPolygonElement/animatedPoints)\n\n*This API requires the following crate features to be activated: `SvgPointList`, `SvgPolygonElement`*"]
    pub fn animated_points(this: &SvgPolygonElement) -> SvgPointList;
}
