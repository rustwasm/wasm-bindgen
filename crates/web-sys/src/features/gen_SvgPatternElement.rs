use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = SvgElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = SVGPatternElement , typescript_type = "SVGPatternElement" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `SvgPatternElement` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPatternElement)
    ///
    ///*This API requires the following crate features to be activated: `SvgPatternElement`*
    pub type SvgPatternElement;

    #[cfg(feature = "SvgAnimatedEnumeration")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGPatternElement" , js_name = patternUnits ) ]
    ///Getter for the `patternUnits` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPatternElement/patternUnits)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedEnumeration`, `SvgPatternElement`*
    pub fn pattern_units(this: &SvgPatternElement) -> SvgAnimatedEnumeration;

    #[cfg(feature = "SvgAnimatedEnumeration")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGPatternElement" , js_name = patternContentUnits ) ]
    ///Getter for the `patternContentUnits` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPatternElement/patternContentUnits)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedEnumeration`, `SvgPatternElement`*
    pub fn pattern_content_units(this: &SvgPatternElement) -> SvgAnimatedEnumeration;

    #[cfg(feature = "SvgAnimatedTransformList")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGPatternElement" , js_name = patternTransform ) ]
    ///Getter for the `patternTransform` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPatternElement/patternTransform)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedTransformList`, `SvgPatternElement`*
    pub fn pattern_transform(this: &SvgPatternElement) -> SvgAnimatedTransformList;

    #[cfg(feature = "SvgAnimatedLength")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGPatternElement" , js_name = x ) ]
    ///Getter for the `x` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPatternElement/x)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgPatternElement`*
    pub fn x(this: &SvgPatternElement) -> SvgAnimatedLength;

    #[cfg(feature = "SvgAnimatedLength")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGPatternElement" , js_name = y ) ]
    ///Getter for the `y` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPatternElement/y)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgPatternElement`*
    pub fn y(this: &SvgPatternElement) -> SvgAnimatedLength;

    #[cfg(feature = "SvgAnimatedLength")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGPatternElement" , js_name = width ) ]
    ///Getter for the `width` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPatternElement/width)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgPatternElement`*
    pub fn width(this: &SvgPatternElement) -> SvgAnimatedLength;

    #[cfg(feature = "SvgAnimatedLength")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGPatternElement" , js_name = height ) ]
    ///Getter for the `height` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPatternElement/height)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgPatternElement`*
    pub fn height(this: &SvgPatternElement) -> SvgAnimatedLength;

    #[cfg(feature = "SvgAnimatedRect")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGPatternElement" , js_name = viewBox ) ]
    ///Getter for the `viewBox` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPatternElement/viewBox)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedRect`, `SvgPatternElement`*
    pub fn view_box(this: &SvgPatternElement) -> SvgAnimatedRect;

    #[cfg(feature = "SvgAnimatedPreserveAspectRatio")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGPatternElement" , js_name = preserveAspectRatio ) ]
    ///Getter for the `preserveAspectRatio` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPatternElement/preserveAspectRatio)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedPreserveAspectRatio`, `SvgPatternElement`*
    pub fn preserve_aspect_ratio(this: &SvgPatternElement) -> SvgAnimatedPreserveAspectRatio;

    #[cfg(feature = "SvgAnimatedString")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGPatternElement" , js_name = href ) ]
    ///Getter for the `href` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPatternElement/href)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedString`, `SvgPatternElement`*
    pub fn href(this: &SvgPatternElement) -> SvgAnimatedString;

}
