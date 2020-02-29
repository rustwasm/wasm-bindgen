use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = SvgGraphicsElement , extends = SvgElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = SVGGeometryElement , typescript_name = SVGGeometryElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `SvgGeometryElement` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGGeometryElement)
    ///
    ///*This API requires the following crate features to be activated: `SvgGeometryElement`*
    pub type SvgGeometryElement;

    #[cfg(feature = "SvgAnimatedNumber")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGGeometryElement" , js_name = pathLength ) ]
    ///Getter for the `pathLength` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGGeometryElement/pathLength)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgGeometryElement`*
    pub fn path_length(this: &SvgGeometryElement) -> SvgAnimatedNumber;

    #[cfg(feature = "SvgPoint")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "SVGGeometryElement" , js_name = getPointAtLength ) ]
    ///The `getPointAtLength()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGGeometryElement/getPointAtLength)
    ///
    ///*This API requires the following crate features to be activated: `SvgGeometryElement`, `SvgPoint`*
    pub fn get_point_at_length(
        this: &SvgGeometryElement,
        distance: f32,
    ) -> Result<SvgPoint, JsValue>;

    # [ wasm_bindgen ( method , structural , js_class = "SVGGeometryElement" , js_name = getTotalLength ) ]
    ///The `getTotalLength()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGGeometryElement/getTotalLength)
    ///
    ///*This API requires the following crate features to be activated: `SvgGeometryElement`*
    pub fn get_total_length(this: &SvgGeometryElement) -> f32;

}
