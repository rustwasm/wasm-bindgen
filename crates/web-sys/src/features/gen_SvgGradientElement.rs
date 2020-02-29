use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = SvgElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = SVGGradientElement , typescript_name = SVGGradientElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `SvgGradientElement` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGGradientElement)
    ///
    ///*This API requires the following crate features to be activated: `SvgGradientElement`*
    pub type SvgGradientElement;

    #[cfg(feature = "SvgAnimatedEnumeration")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGGradientElement" , js_name = gradientUnits ) ]
    ///Getter for the `gradientUnits` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGGradientElement/gradientUnits)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedEnumeration`, `SvgGradientElement`*
    pub fn gradient_units(this: &SvgGradientElement) -> SvgAnimatedEnumeration;

    #[cfg(feature = "SvgAnimatedTransformList")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGGradientElement" , js_name = gradientTransform ) ]
    ///Getter for the `gradientTransform` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGGradientElement/gradientTransform)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedTransformList`, `SvgGradientElement`*
    pub fn gradient_transform(this: &SvgGradientElement) -> SvgAnimatedTransformList;

    #[cfg(feature = "SvgAnimatedEnumeration")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGGradientElement" , js_name = spreadMethod ) ]
    ///Getter for the `spreadMethod` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGGradientElement/spreadMethod)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedEnumeration`, `SvgGradientElement`*
    pub fn spread_method(this: &SvgGradientElement) -> SvgAnimatedEnumeration;

    #[cfg(feature = "SvgAnimatedString")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGGradientElement" , js_name = href ) ]
    ///Getter for the `href` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGGradientElement/href)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedString`, `SvgGradientElement`*
    pub fn href(this: &SvgGradientElement) -> SvgAnimatedString;

}

impl SvgGradientElement {
    ///The `SVGGradientElement.SVG_SPREADMETHOD_UNKNOWN` const.
    ///
    ///*This API requires the following crate features to be activated: `SvgGradientElement`*

    pub const SVG_SPREADMETHOD_UNKNOWN: u16 = 0i64 as u16;

    ///The `SVGGradientElement.SVG_SPREADMETHOD_PAD` const.
    ///
    ///*This API requires the following crate features to be activated: `SvgGradientElement`*

    pub const SVG_SPREADMETHOD_PAD: u16 = 1u64 as u16;

    ///The `SVGGradientElement.SVG_SPREADMETHOD_REFLECT` const.
    ///
    ///*This API requires the following crate features to be activated: `SvgGradientElement`*

    pub const SVG_SPREADMETHOD_REFLECT: u16 = 2u64 as u16;

    ///The `SVGGradientElement.SVG_SPREADMETHOD_REPEAT` const.
    ///
    ///*This API requires the following crate features to be activated: `SvgGradientElement`*

    pub const SVG_SPREADMETHOD_REPEAT: u16 = 3u64 as u16;
}
