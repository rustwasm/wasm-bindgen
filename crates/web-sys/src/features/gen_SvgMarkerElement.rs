use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = SvgElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = SVGMarkerElement , typescript_name = SVGMarkerElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgMarkerElement` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMarkerElement)\n\n*This API requires the following crate features to be activated: `SvgMarkerElement`*"]
    pub type SvgMarkerElement;
    # [ wasm_bindgen ( structural , method , getter , js_name = refX ) ]
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `refX` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMarkerElement/refX)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgMarkerElement`*"]
    pub fn ref_x(this: &SvgMarkerElement) -> SvgAnimatedLength;
    # [ wasm_bindgen ( structural , method , getter , js_name = refY ) ]
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `refY` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMarkerElement/refY)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgMarkerElement`*"]
    pub fn ref_y(this: &SvgMarkerElement) -> SvgAnimatedLength;
    # [ wasm_bindgen ( structural , method , getter , js_name = markerUnits ) ]
    #[cfg(feature = "SvgAnimatedEnumeration")]
    #[doc = "Getter for the `markerUnits` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMarkerElement/markerUnits)\n\n*This API requires the following crate features to be activated: `SvgAnimatedEnumeration`, `SvgMarkerElement`*"]
    pub fn marker_units(this: &SvgMarkerElement) -> SvgAnimatedEnumeration;
    # [ wasm_bindgen ( structural , method , getter , js_name = markerWidth ) ]
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `markerWidth` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMarkerElement/markerWidth)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgMarkerElement`*"]
    pub fn marker_width(this: &SvgMarkerElement) -> SvgAnimatedLength;
    # [ wasm_bindgen ( structural , method , getter , js_name = markerHeight ) ]
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `markerHeight` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMarkerElement/markerHeight)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgMarkerElement`*"]
    pub fn marker_height(this: &SvgMarkerElement) -> SvgAnimatedLength;
    # [ wasm_bindgen ( structural , method , getter , js_name = orientType ) ]
    #[cfg(feature = "SvgAnimatedEnumeration")]
    #[doc = "Getter for the `orientType` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMarkerElement/orientType)\n\n*This API requires the following crate features to be activated: `SvgAnimatedEnumeration`, `SvgMarkerElement`*"]
    pub fn orient_type(this: &SvgMarkerElement) -> SvgAnimatedEnumeration;
    # [ wasm_bindgen ( structural , method , getter , js_name = orientAngle ) ]
    #[cfg(feature = "SvgAnimatedAngle")]
    #[doc = "Getter for the `orientAngle` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMarkerElement/orientAngle)\n\n*This API requires the following crate features to be activated: `SvgAnimatedAngle`, `SvgMarkerElement`*"]
    pub fn orient_angle(this: &SvgMarkerElement) -> SvgAnimatedAngle;
    # [ wasm_bindgen ( structural , method , getter , js_name = viewBox ) ]
    #[cfg(feature = "SvgAnimatedRect")]
    #[doc = "Getter for the `viewBox` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMarkerElement/viewBox)\n\n*This API requires the following crate features to be activated: `SvgAnimatedRect`, `SvgMarkerElement`*"]
    pub fn view_box(this: &SvgMarkerElement) -> SvgAnimatedRect;
    # [ wasm_bindgen ( structural , method , getter , js_name = preserveAspectRatio ) ]
    #[cfg(feature = "SvgAnimatedPreserveAspectRatio")]
    #[doc = "Getter for the `preserveAspectRatio` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMarkerElement/preserveAspectRatio)\n\n*This API requires the following crate features to be activated: `SvgAnimatedPreserveAspectRatio`, `SvgMarkerElement`*"]
    pub fn preserve_aspect_ratio(this: &SvgMarkerElement) -> SvgAnimatedPreserveAspectRatio;
    #[cfg(feature = "SvgAngle")]
    # [ wasm_bindgen ( catch , method , structural , js_name = setOrientToAngle ) ]
    #[doc = "The `setOrientToAngle()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMarkerElement/setOrientToAngle)\n\n*This API requires the following crate features to be activated: `SvgAngle`, `SvgMarkerElement`*"]
    pub fn set_orient_to_angle(this: &SvgMarkerElement, angle: &SvgAngle) -> Result<(), JsValue>;
    # [ wasm_bindgen ( method , structural , js_name = setOrientToAuto ) ]
    #[doc = "The `setOrientToAuto()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMarkerElement/setOrientToAuto)\n\n*This API requires the following crate features to be activated: `SvgMarkerElement`*"]
    pub fn set_orient_to_auto(this: &SvgMarkerElement);
}
impl SvgMarkerElement {
    pub const SVG_MARKERUNITS_UNKNOWN: u16 = 0i64 as u16;
    pub const SVG_MARKERUNITS_USERSPACEONUSE: u16 = 1u64 as u16;
    pub const SVG_MARKERUNITS_STROKEWIDTH: u16 = 2u64 as u16;
    pub const SVG_MARKER_ORIENT_UNKNOWN: u16 = 0i64 as u16;
    pub const SVG_MARKER_ORIENT_AUTO: u16 = 1u64 as u16;
    pub const SVG_MARKER_ORIENT_ANGLE: u16 = 2u64 as u16;
}
