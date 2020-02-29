use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = SvgElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = SVGFETurbulenceElement , typescript_type = "SVGFETurbulenceElement" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `SvgfeTurbulenceElement` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFETurbulenceElement)
    ///
    ///*This API requires the following crate features to be activated: `SvgfeTurbulenceElement`*
    pub type SvgfeTurbulenceElement;

    #[cfg(feature = "SvgAnimatedNumber")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFETurbulenceElement" , js_name = baseFrequencyX ) ]
    ///Getter for the `baseFrequencyX` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFETurbulenceElement/baseFrequencyX)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgfeTurbulenceElement`*
    pub fn base_frequency_x(this: &SvgfeTurbulenceElement) -> SvgAnimatedNumber;

    #[cfg(feature = "SvgAnimatedNumber")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFETurbulenceElement" , js_name = baseFrequencyY ) ]
    ///Getter for the `baseFrequencyY` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFETurbulenceElement/baseFrequencyY)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgfeTurbulenceElement`*
    pub fn base_frequency_y(this: &SvgfeTurbulenceElement) -> SvgAnimatedNumber;

    #[cfg(feature = "SvgAnimatedInteger")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFETurbulenceElement" , js_name = numOctaves ) ]
    ///Getter for the `numOctaves` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFETurbulenceElement/numOctaves)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedInteger`, `SvgfeTurbulenceElement`*
    pub fn num_octaves(this: &SvgfeTurbulenceElement) -> SvgAnimatedInteger;

    #[cfg(feature = "SvgAnimatedNumber")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFETurbulenceElement" , js_name = seed ) ]
    ///Getter for the `seed` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFETurbulenceElement/seed)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgfeTurbulenceElement`*
    pub fn seed(this: &SvgfeTurbulenceElement) -> SvgAnimatedNumber;

    #[cfg(feature = "SvgAnimatedEnumeration")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFETurbulenceElement" , js_name = stitchTiles ) ]
    ///Getter for the `stitchTiles` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFETurbulenceElement/stitchTiles)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedEnumeration`, `SvgfeTurbulenceElement`*
    pub fn stitch_tiles(this: &SvgfeTurbulenceElement) -> SvgAnimatedEnumeration;

    #[cfg(feature = "SvgAnimatedEnumeration")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFETurbulenceElement" , js_name = type ) ]
    ///Getter for the `type` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFETurbulenceElement/type)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedEnumeration`, `SvgfeTurbulenceElement`*
    pub fn type_(this: &SvgfeTurbulenceElement) -> SvgAnimatedEnumeration;

    #[cfg(feature = "SvgAnimatedLength")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFETurbulenceElement" , js_name = x ) ]
    ///Getter for the `x` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFETurbulenceElement/x)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeTurbulenceElement`*
    pub fn x(this: &SvgfeTurbulenceElement) -> SvgAnimatedLength;

    #[cfg(feature = "SvgAnimatedLength")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFETurbulenceElement" , js_name = y ) ]
    ///Getter for the `y` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFETurbulenceElement/y)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeTurbulenceElement`*
    pub fn y(this: &SvgfeTurbulenceElement) -> SvgAnimatedLength;

    #[cfg(feature = "SvgAnimatedLength")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFETurbulenceElement" , js_name = width ) ]
    ///Getter for the `width` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFETurbulenceElement/width)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeTurbulenceElement`*
    pub fn width(this: &SvgfeTurbulenceElement) -> SvgAnimatedLength;

    #[cfg(feature = "SvgAnimatedLength")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFETurbulenceElement" , js_name = height ) ]
    ///Getter for the `height` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFETurbulenceElement/height)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeTurbulenceElement`*
    pub fn height(this: &SvgfeTurbulenceElement) -> SvgAnimatedLength;

    #[cfg(feature = "SvgAnimatedString")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFETurbulenceElement" , js_name = result ) ]
    ///Getter for the `result` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFETurbulenceElement/result)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedString`, `SvgfeTurbulenceElement`*
    pub fn result(this: &SvgfeTurbulenceElement) -> SvgAnimatedString;

}

impl SvgfeTurbulenceElement {
    ///The `SVGFETurbulenceElement.SVG_TURBULENCE_TYPE_UNKNOWN` const.
    ///
    ///*This API requires the following crate features to be activated: `SvgfeTurbulenceElement`*

    pub const SVG_TURBULENCE_TYPE_UNKNOWN: u16 = 0i64 as u16;

    ///The `SVGFETurbulenceElement.SVG_TURBULENCE_TYPE_FRACTALNOISE` const.
    ///
    ///*This API requires the following crate features to be activated: `SvgfeTurbulenceElement`*

    pub const SVG_TURBULENCE_TYPE_FRACTALNOISE: u16 = 1u64 as u16;

    ///The `SVGFETurbulenceElement.SVG_TURBULENCE_TYPE_TURBULENCE` const.
    ///
    ///*This API requires the following crate features to be activated: `SvgfeTurbulenceElement`*

    pub const SVG_TURBULENCE_TYPE_TURBULENCE: u16 = 2u64 as u16;

    ///The `SVGFETurbulenceElement.SVG_STITCHTYPE_UNKNOWN` const.
    ///
    ///*This API requires the following crate features to be activated: `SvgfeTurbulenceElement`*

    pub const SVG_STITCHTYPE_UNKNOWN: u16 = 0i64 as u16;

    ///The `SVGFETurbulenceElement.SVG_STITCHTYPE_STITCH` const.
    ///
    ///*This API requires the following crate features to be activated: `SvgfeTurbulenceElement`*

    pub const SVG_STITCHTYPE_STITCH: u16 = 1u64 as u16;

    ///The `SVGFETurbulenceElement.SVG_STITCHTYPE_NOSTITCH` const.
    ///
    ///*This API requires the following crate features to be activated: `SvgfeTurbulenceElement`*

    pub const SVG_STITCHTYPE_NOSTITCH: u16 = 2u64 as u16;
}
