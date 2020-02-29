use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = SvgElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = SVGComponentTransferFunctionElement , typescript_name = SVGComponentTransferFunctionElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `SvgComponentTransferFunctionElement` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGComponentTransferFunctionElement)
    ///
    ///*This API requires the following crate features to be activated: `SvgComponentTransferFunctionElement`*
    pub type SvgComponentTransferFunctionElement;

    #[cfg(feature = "SvgAnimatedEnumeration")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGComponentTransferFunctionElement" , js_name = type ) ]
    ///Getter for the `type` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGComponentTransferFunctionElement/type)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedEnumeration`, `SvgComponentTransferFunctionElement`*
    pub fn type_(this: &SvgComponentTransferFunctionElement) -> SvgAnimatedEnumeration;

    #[cfg(feature = "SvgAnimatedNumberList")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGComponentTransferFunctionElement" , js_name = tableValues ) ]
    ///Getter for the `tableValues` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGComponentTransferFunctionElement/tableValues)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedNumberList`, `SvgComponentTransferFunctionElement`*
    pub fn table_values(this: &SvgComponentTransferFunctionElement) -> SvgAnimatedNumberList;

    #[cfg(feature = "SvgAnimatedNumber")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGComponentTransferFunctionElement" , js_name = slope ) ]
    ///Getter for the `slope` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGComponentTransferFunctionElement/slope)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgComponentTransferFunctionElement`*
    pub fn slope(this: &SvgComponentTransferFunctionElement) -> SvgAnimatedNumber;

    #[cfg(feature = "SvgAnimatedNumber")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGComponentTransferFunctionElement" , js_name = intercept ) ]
    ///Getter for the `intercept` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGComponentTransferFunctionElement/intercept)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgComponentTransferFunctionElement`*
    pub fn intercept(this: &SvgComponentTransferFunctionElement) -> SvgAnimatedNumber;

    #[cfg(feature = "SvgAnimatedNumber")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGComponentTransferFunctionElement" , js_name = amplitude ) ]
    ///Getter for the `amplitude` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGComponentTransferFunctionElement/amplitude)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgComponentTransferFunctionElement`*
    pub fn amplitude(this: &SvgComponentTransferFunctionElement) -> SvgAnimatedNumber;

    #[cfg(feature = "SvgAnimatedNumber")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGComponentTransferFunctionElement" , js_name = exponent ) ]
    ///Getter for the `exponent` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGComponentTransferFunctionElement/exponent)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgComponentTransferFunctionElement`*
    pub fn exponent(this: &SvgComponentTransferFunctionElement) -> SvgAnimatedNumber;

    #[cfg(feature = "SvgAnimatedNumber")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGComponentTransferFunctionElement" , js_name = offset ) ]
    ///Getter for the `offset` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGComponentTransferFunctionElement/offset)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgComponentTransferFunctionElement`*
    pub fn offset(this: &SvgComponentTransferFunctionElement) -> SvgAnimatedNumber;

}

impl SvgComponentTransferFunctionElement {
    ///The `SVGComponentTransferFunctionElement.SVG_FECOMPONENTTRANSFER_TYPE_UNKNOWN` const.
    ///
    ///*This API requires the following crate features to be activated: `SvgComponentTransferFunctionElement`*

    pub const SVG_FECOMPONENTTRANSFER_TYPE_UNKNOWN: u16 = 0i64 as u16;

    ///The `SVGComponentTransferFunctionElement.SVG_FECOMPONENTTRANSFER_TYPE_IDENTITY` const.
    ///
    ///*This API requires the following crate features to be activated: `SvgComponentTransferFunctionElement`*

    pub const SVG_FECOMPONENTTRANSFER_TYPE_IDENTITY: u16 = 1u64 as u16;

    ///The `SVGComponentTransferFunctionElement.SVG_FECOMPONENTTRANSFER_TYPE_TABLE` const.
    ///
    ///*This API requires the following crate features to be activated: `SvgComponentTransferFunctionElement`*

    pub const SVG_FECOMPONENTTRANSFER_TYPE_TABLE: u16 = 2u64 as u16;

    ///The `SVGComponentTransferFunctionElement.SVG_FECOMPONENTTRANSFER_TYPE_DISCRETE` const.
    ///
    ///*This API requires the following crate features to be activated: `SvgComponentTransferFunctionElement`*

    pub const SVG_FECOMPONENTTRANSFER_TYPE_DISCRETE: u16 = 3u64 as u16;

    ///The `SVGComponentTransferFunctionElement.SVG_FECOMPONENTTRANSFER_TYPE_LINEAR` const.
    ///
    ///*This API requires the following crate features to be activated: `SvgComponentTransferFunctionElement`*

    pub const SVG_FECOMPONENTTRANSFER_TYPE_LINEAR: u16 = 4u64 as u16;

    ///The `SVGComponentTransferFunctionElement.SVG_FECOMPONENTTRANSFER_TYPE_GAMMA` const.
    ///
    ///*This API requires the following crate features to be activated: `SvgComponentTransferFunctionElement`*

    pub const SVG_FECOMPONENTTRANSFER_TYPE_GAMMA: u16 = 5u64 as u16;
}
