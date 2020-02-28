use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = SvgElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = SVGMarkerElement , typescript_name = SVGMarkerElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgMarkerElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMarkerElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgMarkerElement`*"]
    pub type SvgMarkerElement;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGMarkerElement" , js_name = refX ) ]
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `refX` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMarkerElement/refX)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgMarkerElement`*"]
    pub fn ref_x(this: &SvgMarkerElement) -> SvgAnimatedLength;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGMarkerElement" , js_name = refY ) ]
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `refY` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMarkerElement/refY)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgMarkerElement`*"]
    pub fn ref_y(this: &SvgMarkerElement) -> SvgAnimatedLength;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGMarkerElement" , js_name = markerUnits ) ]
    #[cfg(feature = "SvgAnimatedEnumeration")]
    #[doc = "Getter for the `markerUnits` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMarkerElement/markerUnits)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedEnumeration`, `SvgMarkerElement`*"]
    pub fn marker_units(this: &SvgMarkerElement) -> SvgAnimatedEnumeration;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGMarkerElement" , js_name = markerWidth ) ]
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `markerWidth` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMarkerElement/markerWidth)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgMarkerElement`*"]
    pub fn marker_width(this: &SvgMarkerElement) -> SvgAnimatedLength;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGMarkerElement" , js_name = markerHeight ) ]
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `markerHeight` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMarkerElement/markerHeight)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgMarkerElement`*"]
    pub fn marker_height(this: &SvgMarkerElement) -> SvgAnimatedLength;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGMarkerElement" , js_name = orientType ) ]
    #[cfg(feature = "SvgAnimatedEnumeration")]
    #[doc = "Getter for the `orientType` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMarkerElement/orientType)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedEnumeration`, `SvgMarkerElement`*"]
    pub fn orient_type(this: &SvgMarkerElement) -> SvgAnimatedEnumeration;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGMarkerElement" , js_name = orientAngle ) ]
    #[cfg(feature = "SvgAnimatedAngle")]
    #[doc = "Getter for the `orientAngle` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMarkerElement/orientAngle)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedAngle`, `SvgMarkerElement`*"]
    pub fn orient_angle(this: &SvgMarkerElement) -> SvgAnimatedAngle;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGMarkerElement" , js_name = viewBox ) ]
    #[cfg(feature = "SvgAnimatedRect")]
    #[doc = "Getter for the `viewBox` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMarkerElement/viewBox)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedRect`, `SvgMarkerElement`*"]
    pub fn view_box(this: &SvgMarkerElement) -> SvgAnimatedRect;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGMarkerElement" , js_name = preserveAspectRatio ) ]
    #[cfg(feature = "SvgAnimatedPreserveAspectRatio")]
    #[doc = "Getter for the `preserveAspectRatio` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMarkerElement/preserveAspectRatio)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedPreserveAspectRatio`, `SvgMarkerElement`*"]
    pub fn preserve_aspect_ratio(this: &SvgMarkerElement) -> SvgAnimatedPreserveAspectRatio;
    #[cfg(feature = "SvgAngle")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "SVGMarkerElement" , js_name = setOrientToAngle ) ]
    #[doc = "The `setOrientToAngle()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMarkerElement/setOrientToAngle)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAngle`, `SvgMarkerElement`*"]
    pub fn set_orient_to_angle(this: &SvgMarkerElement, angle: &SvgAngle) -> Result<(), JsValue>;
    # [ wasm_bindgen ( method , structural , js_class = "SVGMarkerElement" , js_name = setOrientToAuto ) ]
    #[doc = "The `setOrientToAuto()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMarkerElement/setOrientToAuto)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgMarkerElement`*"]
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
