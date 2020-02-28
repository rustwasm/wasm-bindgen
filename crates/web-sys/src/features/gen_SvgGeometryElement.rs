use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = SvgGraphicsElement , extends = SvgElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = SVGGeometryElement , typescript_name = SVGGeometryElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgGeometryElement` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGGeometryElement)\n\n*This API requires the following crate features to be activated: `SvgGeometryElement`*"]
    pub type SvgGeometryElement;
    # [ wasm_bindgen ( structural , method , getter , js_name = pathLength ) ]
    #[cfg(feature = "SvgAnimatedNumber")]
    #[doc = "Getter for the `pathLength` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGGeometryElement/pathLength)\n\n*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgGeometryElement`*"]
    pub fn path_length(this: &SvgGeometryElement) -> SvgAnimatedNumber;
    #[cfg(feature = "SvgPoint")]
    # [ wasm_bindgen ( catch , method , structural , js_name = getPointAtLength ) ]
    #[doc = "The `getPointAtLength()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGGeometryElement/getPointAtLength)\n\n*This API requires the following crate features to be activated: `SvgGeometryElement`, `SvgPoint`*"]
    pub fn get_point_at_length(
        this: &SvgGeometryElement,
        distance: f32,
    ) -> Result<SvgPoint, JsValue>;
    # [ wasm_bindgen ( method , structural , js_name = getTotalLength ) ]
    #[doc = "The `getTotalLength()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGGeometryElement/getTotalLength)\n\n*This API requires the following crate features to be activated: `SvgGeometryElement`*"]
    pub fn get_total_length(this: &SvgGeometryElement) -> f32;
}
