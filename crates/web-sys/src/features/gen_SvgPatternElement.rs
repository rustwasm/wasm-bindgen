use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = SvgElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = SVGPatternElement , typescript_name = SVGPatternElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgPatternElement` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPatternElement)\n\n*This API requires the following crate features to be activated: `SvgPatternElement`*"]
    pub type SvgPatternElement;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGPatternElement" , js_name = patternUnits ) ]
    #[cfg(feature = "SvgAnimatedEnumeration")]
    #[doc = "Getter for the `patternUnits` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPatternElement/patternUnits)\n\n*This API requires the following crate features to be activated: `SvgAnimatedEnumeration`, `SvgPatternElement`*"]
    pub fn pattern_units(this: &SvgPatternElement) -> SvgAnimatedEnumeration;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGPatternElement" , js_name = patternContentUnits ) ]
    #[cfg(feature = "SvgAnimatedEnumeration")]
    #[doc = "Getter for the `patternContentUnits` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPatternElement/patternContentUnits)\n\n*This API requires the following crate features to be activated: `SvgAnimatedEnumeration`, `SvgPatternElement`*"]
    pub fn pattern_content_units(this: &SvgPatternElement) -> SvgAnimatedEnumeration;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGPatternElement" , js_name = patternTransform ) ]
    #[cfg(feature = "SvgAnimatedTransformList")]
    #[doc = "Getter for the `patternTransform` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPatternElement/patternTransform)\n\n*This API requires the following crate features to be activated: `SvgAnimatedTransformList`, `SvgPatternElement`*"]
    pub fn pattern_transform(this: &SvgPatternElement) -> SvgAnimatedTransformList;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGPatternElement" , js_name = x ) ]
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `x` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPatternElement/x)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgPatternElement`*"]
    pub fn x(this: &SvgPatternElement) -> SvgAnimatedLength;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGPatternElement" , js_name = y ) ]
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `y` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPatternElement/y)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgPatternElement`*"]
    pub fn y(this: &SvgPatternElement) -> SvgAnimatedLength;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGPatternElement" , js_name = width ) ]
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `width` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPatternElement/width)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgPatternElement`*"]
    pub fn width(this: &SvgPatternElement) -> SvgAnimatedLength;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGPatternElement" , js_name = height ) ]
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `height` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPatternElement/height)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgPatternElement`*"]
    pub fn height(this: &SvgPatternElement) -> SvgAnimatedLength;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGPatternElement" , js_name = viewBox ) ]
    #[cfg(feature = "SvgAnimatedRect")]
    #[doc = "Getter for the `viewBox` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPatternElement/viewBox)\n\n*This API requires the following crate features to be activated: `SvgAnimatedRect`, `SvgPatternElement`*"]
    pub fn view_box(this: &SvgPatternElement) -> SvgAnimatedRect;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGPatternElement" , js_name = preserveAspectRatio ) ]
    #[cfg(feature = "SvgAnimatedPreserveAspectRatio")]
    #[doc = "Getter for the `preserveAspectRatio` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPatternElement/preserveAspectRatio)\n\n*This API requires the following crate features to be activated: `SvgAnimatedPreserveAspectRatio`, `SvgPatternElement`*"]
    pub fn preserve_aspect_ratio(this: &SvgPatternElement) -> SvgAnimatedPreserveAspectRatio;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGPatternElement" , js_name = href ) ]
    #[cfg(feature = "SvgAnimatedString")]
    #[doc = "Getter for the `href` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPatternElement/href)\n\n*This API requires the following crate features to be activated: `SvgAnimatedString`, `SvgPatternElement`*"]
    pub fn href(this: &SvgPatternElement) -> SvgAnimatedString;
}
