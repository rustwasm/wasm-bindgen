use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = SvgElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = SVGMaskElement , typescript_name = SVGMaskElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `SvgMaskElement` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMaskElement)
    ///
    ///*This API requires the following crate features to be activated: `SvgMaskElement`*
    pub type SvgMaskElement;

    #[cfg(feature = "SvgAnimatedEnumeration")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGMaskElement" , js_name = maskUnits ) ]
    ///Getter for the `maskUnits` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMaskElement/maskUnits)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedEnumeration`, `SvgMaskElement`*
    pub fn mask_units(this: &SvgMaskElement) -> SvgAnimatedEnumeration;

    #[cfg(feature = "SvgAnimatedEnumeration")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGMaskElement" , js_name = maskContentUnits ) ]
    ///Getter for the `maskContentUnits` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMaskElement/maskContentUnits)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedEnumeration`, `SvgMaskElement`*
    pub fn mask_content_units(this: &SvgMaskElement) -> SvgAnimatedEnumeration;

    #[cfg(feature = "SvgAnimatedLength")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGMaskElement" , js_name = x ) ]
    ///Getter for the `x` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMaskElement/x)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgMaskElement`*
    pub fn x(this: &SvgMaskElement) -> SvgAnimatedLength;

    #[cfg(feature = "SvgAnimatedLength")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGMaskElement" , js_name = y ) ]
    ///Getter for the `y` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMaskElement/y)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgMaskElement`*
    pub fn y(this: &SvgMaskElement) -> SvgAnimatedLength;

    #[cfg(feature = "SvgAnimatedLength")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGMaskElement" , js_name = width ) ]
    ///Getter for the `width` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMaskElement/width)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgMaskElement`*
    pub fn width(this: &SvgMaskElement) -> SvgAnimatedLength;

    #[cfg(feature = "SvgAnimatedLength")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGMaskElement" , js_name = height ) ]
    ///Getter for the `height` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMaskElement/height)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgMaskElement`*
    pub fn height(this: &SvgMaskElement) -> SvgAnimatedLength;

}

impl SvgMaskElement {
    ///The `SVGMaskElement.SVG_MASKTYPE_LUMINANCE` const.
    ///
    ///*This API requires the following crate features to be activated: `SvgMaskElement`*

    pub const SVG_MASKTYPE_LUMINANCE: u16 = 0i64 as u16;

    ///The `SVGMaskElement.SVG_MASKTYPE_ALPHA` const.
    ///
    ///*This API requires the following crate features to be activated: `SvgMaskElement`*

    pub const SVG_MASKTYPE_ALPHA: u16 = 1u64 as u16;
}
