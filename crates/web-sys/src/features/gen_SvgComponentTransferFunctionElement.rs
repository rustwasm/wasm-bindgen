use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = SvgElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = SVGComponentTransferFunctionElement , typescript_name = SVGComponentTransferFunctionElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgComponentTransferFunctionElement` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGComponentTransferFunctionElement)\n\n*This API requires the following crate features to be activated: `SvgComponentTransferFunctionElement`*"]
    pub type SvgComponentTransferFunctionElement;
    # [ wasm_bindgen ( structural , method , getter , js_name = type ) ]
    #[cfg(feature = "SvgAnimatedEnumeration")]
    #[doc = "Getter for the `type` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGComponentTransferFunctionElement/type)\n\n*This API requires the following crate features to be activated: `SvgAnimatedEnumeration`, `SvgComponentTransferFunctionElement`*"]
    pub fn type_(this: &SvgComponentTransferFunctionElement) -> SvgAnimatedEnumeration;
    # [ wasm_bindgen ( structural , method , getter , js_name = tableValues ) ]
    #[cfg(feature = "SvgAnimatedNumberList")]
    #[doc = "Getter for the `tableValues` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGComponentTransferFunctionElement/tableValues)\n\n*This API requires the following crate features to be activated: `SvgAnimatedNumberList`, `SvgComponentTransferFunctionElement`*"]
    pub fn table_values(this: &SvgComponentTransferFunctionElement) -> SvgAnimatedNumberList;
    # [ wasm_bindgen ( structural , method , getter , js_name = slope ) ]
    #[cfg(feature = "SvgAnimatedNumber")]
    #[doc = "Getter for the `slope` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGComponentTransferFunctionElement/slope)\n\n*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgComponentTransferFunctionElement`*"]
    pub fn slope(this: &SvgComponentTransferFunctionElement) -> SvgAnimatedNumber;
    # [ wasm_bindgen ( structural , method , getter , js_name = intercept ) ]
    #[cfg(feature = "SvgAnimatedNumber")]
    #[doc = "Getter for the `intercept` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGComponentTransferFunctionElement/intercept)\n\n*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgComponentTransferFunctionElement`*"]
    pub fn intercept(this: &SvgComponentTransferFunctionElement) -> SvgAnimatedNumber;
    # [ wasm_bindgen ( structural , method , getter , js_name = amplitude ) ]
    #[cfg(feature = "SvgAnimatedNumber")]
    #[doc = "Getter for the `amplitude` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGComponentTransferFunctionElement/amplitude)\n\n*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgComponentTransferFunctionElement`*"]
    pub fn amplitude(this: &SvgComponentTransferFunctionElement) -> SvgAnimatedNumber;
    # [ wasm_bindgen ( structural , method , getter , js_name = exponent ) ]
    #[cfg(feature = "SvgAnimatedNumber")]
    #[doc = "Getter for the `exponent` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGComponentTransferFunctionElement/exponent)\n\n*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgComponentTransferFunctionElement`*"]
    pub fn exponent(this: &SvgComponentTransferFunctionElement) -> SvgAnimatedNumber;
    # [ wasm_bindgen ( structural , method , getter , js_name = offset ) ]
    #[cfg(feature = "SvgAnimatedNumber")]
    #[doc = "Getter for the `offset` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGComponentTransferFunctionElement/offset)\n\n*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgComponentTransferFunctionElement`*"]
    pub fn offset(this: &SvgComponentTransferFunctionElement) -> SvgAnimatedNumber;
}
impl SvgComponentTransferFunctionElement {
    pub const SVG_FECOMPONENTTRANSFER_TYPE_UNKNOWN: u16 = 0i64 as u16;
    pub const SVG_FECOMPONENTTRANSFER_TYPE_IDENTITY: u16 = 1u64 as u16;
    pub const SVG_FECOMPONENTTRANSFER_TYPE_TABLE: u16 = 2u64 as u16;
    pub const SVG_FECOMPONENTTRANSFER_TYPE_DISCRETE: u16 = 3u64 as u16;
    pub const SVG_FECOMPONENTTRANSFER_TYPE_LINEAR: u16 = 4u64 as u16;
    pub const SVG_FECOMPONENTTRANSFER_TYPE_GAMMA: u16 = 5u64 as u16;
}
