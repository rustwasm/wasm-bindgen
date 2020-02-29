use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = SvgElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = SVGMarkerElement , typescript_type = "SVGMarkerElement" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `SvgMarkerElement` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMarkerElement)
    ///
    ///*This API requires the following crate features to be activated: `SvgMarkerElement`*
    pub type SvgMarkerElement;

    #[cfg(feature = "SvgAnimatedLength")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGMarkerElement" , js_name = refX ) ]
    ///Getter for the `refX` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMarkerElement/refX)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgMarkerElement`*
    pub fn ref_x(this: &SvgMarkerElement) -> SvgAnimatedLength;

    #[cfg(feature = "SvgAnimatedLength")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGMarkerElement" , js_name = refY ) ]
    ///Getter for the `refY` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMarkerElement/refY)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgMarkerElement`*
    pub fn ref_y(this: &SvgMarkerElement) -> SvgAnimatedLength;

    #[cfg(feature = "SvgAnimatedEnumeration")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGMarkerElement" , js_name = markerUnits ) ]
    ///Getter for the `markerUnits` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMarkerElement/markerUnits)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedEnumeration`, `SvgMarkerElement`*
    pub fn marker_units(this: &SvgMarkerElement) -> SvgAnimatedEnumeration;

    #[cfg(feature = "SvgAnimatedLength")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGMarkerElement" , js_name = markerWidth ) ]
    ///Getter for the `markerWidth` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMarkerElement/markerWidth)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgMarkerElement`*
    pub fn marker_width(this: &SvgMarkerElement) -> SvgAnimatedLength;

    #[cfg(feature = "SvgAnimatedLength")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGMarkerElement" , js_name = markerHeight ) ]
    ///Getter for the `markerHeight` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMarkerElement/markerHeight)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgMarkerElement`*
    pub fn marker_height(this: &SvgMarkerElement) -> SvgAnimatedLength;

    #[cfg(feature = "SvgAnimatedEnumeration")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGMarkerElement" , js_name = orientType ) ]
    ///Getter for the `orientType` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMarkerElement/orientType)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedEnumeration`, `SvgMarkerElement`*
    pub fn orient_type(this: &SvgMarkerElement) -> SvgAnimatedEnumeration;

    #[cfg(feature = "SvgAnimatedAngle")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGMarkerElement" , js_name = orientAngle ) ]
    ///Getter for the `orientAngle` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMarkerElement/orientAngle)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedAngle`, `SvgMarkerElement`*
    pub fn orient_angle(this: &SvgMarkerElement) -> SvgAnimatedAngle;

    #[cfg(feature = "SvgAnimatedRect")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGMarkerElement" , js_name = viewBox ) ]
    ///Getter for the `viewBox` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMarkerElement/viewBox)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedRect`, `SvgMarkerElement`*
    pub fn view_box(this: &SvgMarkerElement) -> SvgAnimatedRect;

    #[cfg(feature = "SvgAnimatedPreserveAspectRatio")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGMarkerElement" , js_name = preserveAspectRatio ) ]
    ///Getter for the `preserveAspectRatio` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMarkerElement/preserveAspectRatio)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedPreserveAspectRatio`, `SvgMarkerElement`*
    pub fn preserve_aspect_ratio(this: &SvgMarkerElement) -> SvgAnimatedPreserveAspectRatio;

    #[cfg(feature = "SvgAngle")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "SVGMarkerElement" , js_name = setOrientToAngle ) ]
    ///The `setOrientToAngle()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMarkerElement/setOrientToAngle)
    ///
    ///*This API requires the following crate features to be activated: `SvgAngle`, `SvgMarkerElement`*
    pub fn set_orient_to_angle(this: &SvgMarkerElement, angle: &SvgAngle) -> Result<(), JsValue>;

    # [ wasm_bindgen ( method , structural , js_class = "SVGMarkerElement" , js_name = setOrientToAuto ) ]
    ///The `setOrientToAuto()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMarkerElement/setOrientToAuto)
    ///
    ///*This API requires the following crate features to be activated: `SvgMarkerElement`*
    pub fn set_orient_to_auto(this: &SvgMarkerElement);

}

impl SvgMarkerElement {
    ///The `SVGMarkerElement.SVG_MARKERUNITS_UNKNOWN` const.
    ///
    ///*This API requires the following crate features to be activated: `SvgMarkerElement`*

    pub const SVG_MARKERUNITS_UNKNOWN: u16 = 0i64 as u16;

    ///The `SVGMarkerElement.SVG_MARKERUNITS_USERSPACEONUSE` const.
    ///
    ///*This API requires the following crate features to be activated: `SvgMarkerElement`*

    pub const SVG_MARKERUNITS_USERSPACEONUSE: u16 = 1u64 as u16;

    ///The `SVGMarkerElement.SVG_MARKERUNITS_STROKEWIDTH` const.
    ///
    ///*This API requires the following crate features to be activated: `SvgMarkerElement`*

    pub const SVG_MARKERUNITS_STROKEWIDTH: u16 = 2u64 as u16;

    ///The `SVGMarkerElement.SVG_MARKER_ORIENT_UNKNOWN` const.
    ///
    ///*This API requires the following crate features to be activated: `SvgMarkerElement`*

    pub const SVG_MARKER_ORIENT_UNKNOWN: u16 = 0i64 as u16;

    ///The `SVGMarkerElement.SVG_MARKER_ORIENT_AUTO` const.
    ///
    ///*This API requires the following crate features to be activated: `SvgMarkerElement`*

    pub const SVG_MARKER_ORIENT_AUTO: u16 = 1u64 as u16;

    ///The `SVGMarkerElement.SVG_MARKER_ORIENT_ANGLE` const.
    ///
    ///*This API requires the following crate features to be activated: `SvgMarkerElement`*

    pub const SVG_MARKER_ORIENT_ANGLE: u16 = 2u64 as u16;
}
