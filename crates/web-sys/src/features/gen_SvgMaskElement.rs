use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = SvgElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = SVGMaskElement , typescript_name = SVGMaskElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgMaskElement` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMaskElement)\n\n*This API requires the following crate features to be activated: `SvgMaskElement`*"]
    pub type SvgMaskElement;
    # [ wasm_bindgen ( structural , method , getter , js_name = maskUnits ) ]
    #[cfg(feature = "SvgAnimatedEnumeration")]
    #[doc = "Getter for the `maskUnits` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMaskElement/maskUnits)\n\n*This API requires the following crate features to be activated: `SvgAnimatedEnumeration`, `SvgMaskElement`*"]
    pub fn mask_units(this: &SvgMaskElement) -> SvgAnimatedEnumeration;
    # [ wasm_bindgen ( structural , method , getter , js_name = maskContentUnits ) ]
    #[cfg(feature = "SvgAnimatedEnumeration")]
    #[doc = "Getter for the `maskContentUnits` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMaskElement/maskContentUnits)\n\n*This API requires the following crate features to be activated: `SvgAnimatedEnumeration`, `SvgMaskElement`*"]
    pub fn mask_content_units(this: &SvgMaskElement) -> SvgAnimatedEnumeration;
    # [ wasm_bindgen ( structural , method , getter , js_name = x ) ]
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `x` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMaskElement/x)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgMaskElement`*"]
    pub fn x(this: &SvgMaskElement) -> SvgAnimatedLength;
    # [ wasm_bindgen ( structural , method , getter , js_name = y ) ]
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `y` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMaskElement/y)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgMaskElement`*"]
    pub fn y(this: &SvgMaskElement) -> SvgAnimatedLength;
    # [ wasm_bindgen ( structural , method , getter , js_name = width ) ]
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `width` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMaskElement/width)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgMaskElement`*"]
    pub fn width(this: &SvgMaskElement) -> SvgAnimatedLength;
    # [ wasm_bindgen ( structural , method , getter , js_name = height ) ]
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `height` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMaskElement/height)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgMaskElement`*"]
    pub fn height(this: &SvgMaskElement) -> SvgAnimatedLength;
}
impl SvgMaskElement {
    pub const SVG_MASKTYPE_LUMINANCE: u16 = 0i64 as u16;
    pub const SVG_MASKTYPE_ALPHA: u16 = 1u64 as u16;
}
