use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = SvgGraphicsElement , extends = SvgElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = SVGImageElement , typescript_name = SVGImageElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgImageElement` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGImageElement)\n\n*This API requires the following crate features to be activated: `SvgImageElement`*"]
    pub type SvgImageElement;
    # [ wasm_bindgen ( structural , method , getter , js_name = x ) ]
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `x` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGImageElement/x)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgImageElement`*"]
    pub fn x(this: &SvgImageElement) -> SvgAnimatedLength;
    # [ wasm_bindgen ( structural , method , getter , js_name = y ) ]
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `y` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGImageElement/y)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgImageElement`*"]
    pub fn y(this: &SvgImageElement) -> SvgAnimatedLength;
    # [ wasm_bindgen ( structural , method , getter , js_name = width ) ]
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `width` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGImageElement/width)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgImageElement`*"]
    pub fn width(this: &SvgImageElement) -> SvgAnimatedLength;
    # [ wasm_bindgen ( structural , method , getter , js_name = height ) ]
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `height` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGImageElement/height)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgImageElement`*"]
    pub fn height(this: &SvgImageElement) -> SvgAnimatedLength;
    # [ wasm_bindgen ( structural , method , getter , js_name = preserveAspectRatio ) ]
    #[cfg(feature = "SvgAnimatedPreserveAspectRatio")]
    #[doc = "Getter for the `preserveAspectRatio` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGImageElement/preserveAspectRatio)\n\n*This API requires the following crate features to be activated: `SvgAnimatedPreserveAspectRatio`, `SvgImageElement`*"]
    pub fn preserve_aspect_ratio(this: &SvgImageElement) -> SvgAnimatedPreserveAspectRatio;
    # [ wasm_bindgen ( structural , method , getter , js_name = href ) ]
    #[cfg(feature = "SvgAnimatedString")]
    #[doc = "Getter for the `href` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGImageElement/href)\n\n*This API requires the following crate features to be activated: `SvgAnimatedString`, `SvgImageElement`*"]
    pub fn href(this: &SvgImageElement) -> SvgAnimatedString;
}
