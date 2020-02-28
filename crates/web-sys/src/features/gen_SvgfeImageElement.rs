use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = SvgElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = SVGFEImageElement , typescript_name = SVGFEImageElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgfeImageElement` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEImageElement)\n\n*This API requires the following crate features to be activated: `SvgfeImageElement`*"]
    pub type SvgfeImageElement;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFEImageElement" , js_name = preserveAspectRatio ) ]
    #[cfg(feature = "SvgAnimatedPreserveAspectRatio")]
    #[doc = "Getter for the `preserveAspectRatio` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEImageElement/preserveAspectRatio)\n\n*This API requires the following crate features to be activated: `SvgAnimatedPreserveAspectRatio`, `SvgfeImageElement`*"]
    pub fn preserve_aspect_ratio(this: &SvgfeImageElement) -> SvgAnimatedPreserveAspectRatio;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFEImageElement" , js_name = x ) ]
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `x` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEImageElement/x)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeImageElement`*"]
    pub fn x(this: &SvgfeImageElement) -> SvgAnimatedLength;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFEImageElement" , js_name = y ) ]
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `y` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEImageElement/y)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeImageElement`*"]
    pub fn y(this: &SvgfeImageElement) -> SvgAnimatedLength;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFEImageElement" , js_name = width ) ]
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `width` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEImageElement/width)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeImageElement`*"]
    pub fn width(this: &SvgfeImageElement) -> SvgAnimatedLength;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFEImageElement" , js_name = height ) ]
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `height` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEImageElement/height)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeImageElement`*"]
    pub fn height(this: &SvgfeImageElement) -> SvgAnimatedLength;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFEImageElement" , js_name = result ) ]
    #[cfg(feature = "SvgAnimatedString")]
    #[doc = "Getter for the `result` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEImageElement/result)\n\n*This API requires the following crate features to be activated: `SvgAnimatedString`, `SvgfeImageElement`*"]
    pub fn result(this: &SvgfeImageElement) -> SvgAnimatedString;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFEImageElement" , js_name = href ) ]
    #[cfg(feature = "SvgAnimatedString")]
    #[doc = "Getter for the `href` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEImageElement/href)\n\n*This API requires the following crate features to be activated: `SvgAnimatedString`, `SvgfeImageElement`*"]
    pub fn href(this: &SvgfeImageElement) -> SvgAnimatedString;
}
