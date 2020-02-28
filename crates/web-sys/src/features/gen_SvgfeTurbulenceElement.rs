use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = SvgElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = SVGFETurbulenceElement , typescript_name = SVGFETurbulenceElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgfeTurbulenceElement` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFETurbulenceElement)\n\n*This API requires the following crate features to be activated: `SvgfeTurbulenceElement`*"]
    pub type SvgfeTurbulenceElement;
    # [ wasm_bindgen ( structural , method , getter , js_name = baseFrequencyX ) ]
    #[cfg(feature = "SvgAnimatedNumber")]
    #[doc = "Getter for the `baseFrequencyX` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFETurbulenceElement/baseFrequencyX)\n\n*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgfeTurbulenceElement`*"]
    pub fn base_frequency_x(this: &SvgfeTurbulenceElement) -> SvgAnimatedNumber;
    # [ wasm_bindgen ( structural , method , getter , js_name = baseFrequencyY ) ]
    #[cfg(feature = "SvgAnimatedNumber")]
    #[doc = "Getter for the `baseFrequencyY` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFETurbulenceElement/baseFrequencyY)\n\n*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgfeTurbulenceElement`*"]
    pub fn base_frequency_y(this: &SvgfeTurbulenceElement) -> SvgAnimatedNumber;
    # [ wasm_bindgen ( structural , method , getter , js_name = numOctaves ) ]
    #[cfg(feature = "SvgAnimatedInteger")]
    #[doc = "Getter for the `numOctaves` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFETurbulenceElement/numOctaves)\n\n*This API requires the following crate features to be activated: `SvgAnimatedInteger`, `SvgfeTurbulenceElement`*"]
    pub fn num_octaves(this: &SvgfeTurbulenceElement) -> SvgAnimatedInteger;
    # [ wasm_bindgen ( structural , method , getter , js_name = seed ) ]
    #[cfg(feature = "SvgAnimatedNumber")]
    #[doc = "Getter for the `seed` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFETurbulenceElement/seed)\n\n*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgfeTurbulenceElement`*"]
    pub fn seed(this: &SvgfeTurbulenceElement) -> SvgAnimatedNumber;
    # [ wasm_bindgen ( structural , method , getter , js_name = stitchTiles ) ]
    #[cfg(feature = "SvgAnimatedEnumeration")]
    #[doc = "Getter for the `stitchTiles` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFETurbulenceElement/stitchTiles)\n\n*This API requires the following crate features to be activated: `SvgAnimatedEnumeration`, `SvgfeTurbulenceElement`*"]
    pub fn stitch_tiles(this: &SvgfeTurbulenceElement) -> SvgAnimatedEnumeration;
    # [ wasm_bindgen ( structural , method , getter , js_name = type ) ]
    #[cfg(feature = "SvgAnimatedEnumeration")]
    #[doc = "Getter for the `type` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFETurbulenceElement/type)\n\n*This API requires the following crate features to be activated: `SvgAnimatedEnumeration`, `SvgfeTurbulenceElement`*"]
    pub fn type_(this: &SvgfeTurbulenceElement) -> SvgAnimatedEnumeration;
    # [ wasm_bindgen ( structural , method , getter , js_name = x ) ]
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `x` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFETurbulenceElement/x)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeTurbulenceElement`*"]
    pub fn x(this: &SvgfeTurbulenceElement) -> SvgAnimatedLength;
    # [ wasm_bindgen ( structural , method , getter , js_name = y ) ]
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `y` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFETurbulenceElement/y)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeTurbulenceElement`*"]
    pub fn y(this: &SvgfeTurbulenceElement) -> SvgAnimatedLength;
    # [ wasm_bindgen ( structural , method , getter , js_name = width ) ]
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `width` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFETurbulenceElement/width)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeTurbulenceElement`*"]
    pub fn width(this: &SvgfeTurbulenceElement) -> SvgAnimatedLength;
    # [ wasm_bindgen ( structural , method , getter , js_name = height ) ]
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `height` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFETurbulenceElement/height)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeTurbulenceElement`*"]
    pub fn height(this: &SvgfeTurbulenceElement) -> SvgAnimatedLength;
    # [ wasm_bindgen ( structural , method , getter , js_name = result ) ]
    #[cfg(feature = "SvgAnimatedString")]
    #[doc = "Getter for the `result` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFETurbulenceElement/result)\n\n*This API requires the following crate features to be activated: `SvgAnimatedString`, `SvgfeTurbulenceElement`*"]
    pub fn result(this: &SvgfeTurbulenceElement) -> SvgAnimatedString;
}
impl SvgfeTurbulenceElement {
    pub const SVG_TURBULENCE_TYPE_UNKNOWN: u16 = 0i64 as u16;
    pub const SVG_TURBULENCE_TYPE_FRACTALNOISE: u16 = 1u64 as u16;
    pub const SVG_TURBULENCE_TYPE_TURBULENCE: u16 = 2u64 as u16;
    pub const SVG_STITCHTYPE_UNKNOWN: u16 = 0i64 as u16;
    pub const SVG_STITCHTYPE_STITCH: u16 = 1u64 as u16;
    pub const SVG_STITCHTYPE_NOSTITCH: u16 = 2u64 as u16;
}
