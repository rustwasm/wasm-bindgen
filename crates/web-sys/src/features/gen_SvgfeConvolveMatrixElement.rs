use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = SvgElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = SVGFEConvolveMatrixElement , typescript_type = "SVGFEConvolveMatrixElement" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `SvgfeConvolveMatrixElement` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEConvolveMatrixElement)
    ///
    ///*This API requires the following crate features to be activated: `SvgfeConvolveMatrixElement`*
    pub type SvgfeConvolveMatrixElement;

    #[cfg(feature = "SvgAnimatedString")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFEConvolveMatrixElement" , js_name = in1 ) ]
    ///Getter for the `in1` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEConvolveMatrixElement/in1)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedString`, `SvgfeConvolveMatrixElement`*
    pub fn in1(this: &SvgfeConvolveMatrixElement) -> SvgAnimatedString;

    #[cfg(feature = "SvgAnimatedInteger")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFEConvolveMatrixElement" , js_name = orderX ) ]
    ///Getter for the `orderX` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEConvolveMatrixElement/orderX)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedInteger`, `SvgfeConvolveMatrixElement`*
    pub fn order_x(this: &SvgfeConvolveMatrixElement) -> SvgAnimatedInteger;

    #[cfg(feature = "SvgAnimatedInteger")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFEConvolveMatrixElement" , js_name = orderY ) ]
    ///Getter for the `orderY` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEConvolveMatrixElement/orderY)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedInteger`, `SvgfeConvolveMatrixElement`*
    pub fn order_y(this: &SvgfeConvolveMatrixElement) -> SvgAnimatedInteger;

    #[cfg(feature = "SvgAnimatedNumberList")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFEConvolveMatrixElement" , js_name = kernelMatrix ) ]
    ///Getter for the `kernelMatrix` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEConvolveMatrixElement/kernelMatrix)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedNumberList`, `SvgfeConvolveMatrixElement`*
    pub fn kernel_matrix(this: &SvgfeConvolveMatrixElement) -> SvgAnimatedNumberList;

    #[cfg(feature = "SvgAnimatedNumber")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFEConvolveMatrixElement" , js_name = divisor ) ]
    ///Getter for the `divisor` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEConvolveMatrixElement/divisor)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgfeConvolveMatrixElement`*
    pub fn divisor(this: &SvgfeConvolveMatrixElement) -> SvgAnimatedNumber;

    #[cfg(feature = "SvgAnimatedNumber")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFEConvolveMatrixElement" , js_name = bias ) ]
    ///Getter for the `bias` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEConvolveMatrixElement/bias)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgfeConvolveMatrixElement`*
    pub fn bias(this: &SvgfeConvolveMatrixElement) -> SvgAnimatedNumber;

    #[cfg(feature = "SvgAnimatedInteger")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFEConvolveMatrixElement" , js_name = targetX ) ]
    ///Getter for the `targetX` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEConvolveMatrixElement/targetX)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedInteger`, `SvgfeConvolveMatrixElement`*
    pub fn target_x(this: &SvgfeConvolveMatrixElement) -> SvgAnimatedInteger;

    #[cfg(feature = "SvgAnimatedInteger")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFEConvolveMatrixElement" , js_name = targetY ) ]
    ///Getter for the `targetY` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEConvolveMatrixElement/targetY)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedInteger`, `SvgfeConvolveMatrixElement`*
    pub fn target_y(this: &SvgfeConvolveMatrixElement) -> SvgAnimatedInteger;

    #[cfg(feature = "SvgAnimatedEnumeration")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFEConvolveMatrixElement" , js_name = edgeMode ) ]
    ///Getter for the `edgeMode` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEConvolveMatrixElement/edgeMode)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedEnumeration`, `SvgfeConvolveMatrixElement`*
    pub fn edge_mode(this: &SvgfeConvolveMatrixElement) -> SvgAnimatedEnumeration;

    #[cfg(feature = "SvgAnimatedNumber")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFEConvolveMatrixElement" , js_name = kernelUnitLengthX ) ]
    ///Getter for the `kernelUnitLengthX` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEConvolveMatrixElement/kernelUnitLengthX)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgfeConvolveMatrixElement`*
    pub fn kernel_unit_length_x(this: &SvgfeConvolveMatrixElement) -> SvgAnimatedNumber;

    #[cfg(feature = "SvgAnimatedNumber")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFEConvolveMatrixElement" , js_name = kernelUnitLengthY ) ]
    ///Getter for the `kernelUnitLengthY` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEConvolveMatrixElement/kernelUnitLengthY)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgfeConvolveMatrixElement`*
    pub fn kernel_unit_length_y(this: &SvgfeConvolveMatrixElement) -> SvgAnimatedNumber;

    #[cfg(feature = "SvgAnimatedBoolean")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFEConvolveMatrixElement" , js_name = preserveAlpha ) ]
    ///Getter for the `preserveAlpha` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEConvolveMatrixElement/preserveAlpha)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedBoolean`, `SvgfeConvolveMatrixElement`*
    pub fn preserve_alpha(this: &SvgfeConvolveMatrixElement) -> SvgAnimatedBoolean;

    #[cfg(feature = "SvgAnimatedLength")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFEConvolveMatrixElement" , js_name = x ) ]
    ///Getter for the `x` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEConvolveMatrixElement/x)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeConvolveMatrixElement`*
    pub fn x(this: &SvgfeConvolveMatrixElement) -> SvgAnimatedLength;

    #[cfg(feature = "SvgAnimatedLength")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFEConvolveMatrixElement" , js_name = y ) ]
    ///Getter for the `y` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEConvolveMatrixElement/y)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeConvolveMatrixElement`*
    pub fn y(this: &SvgfeConvolveMatrixElement) -> SvgAnimatedLength;

    #[cfg(feature = "SvgAnimatedLength")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFEConvolveMatrixElement" , js_name = width ) ]
    ///Getter for the `width` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEConvolveMatrixElement/width)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeConvolveMatrixElement`*
    pub fn width(this: &SvgfeConvolveMatrixElement) -> SvgAnimatedLength;

    #[cfg(feature = "SvgAnimatedLength")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFEConvolveMatrixElement" , js_name = height ) ]
    ///Getter for the `height` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEConvolveMatrixElement/height)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeConvolveMatrixElement`*
    pub fn height(this: &SvgfeConvolveMatrixElement) -> SvgAnimatedLength;

    #[cfg(feature = "SvgAnimatedString")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFEConvolveMatrixElement" , js_name = result ) ]
    ///Getter for the `result` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEConvolveMatrixElement/result)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedString`, `SvgfeConvolveMatrixElement`*
    pub fn result(this: &SvgfeConvolveMatrixElement) -> SvgAnimatedString;

}

impl SvgfeConvolveMatrixElement {
    ///The `SVGFEConvolveMatrixElement.SVG_EDGEMODE_UNKNOWN` const.
    ///
    ///*This API requires the following crate features to be activated: `SvgfeConvolveMatrixElement`*

    pub const SVG_EDGEMODE_UNKNOWN: u16 = 0i64 as u16;

    ///The `SVGFEConvolveMatrixElement.SVG_EDGEMODE_DUPLICATE` const.
    ///
    ///*This API requires the following crate features to be activated: `SvgfeConvolveMatrixElement`*

    pub const SVG_EDGEMODE_DUPLICATE: u16 = 1u64 as u16;

    ///The `SVGFEConvolveMatrixElement.SVG_EDGEMODE_WRAP` const.
    ///
    ///*This API requires the following crate features to be activated: `SvgfeConvolveMatrixElement`*

    pub const SVG_EDGEMODE_WRAP: u16 = 2u64 as u16;

    ///The `SVGFEConvolveMatrixElement.SVG_EDGEMODE_NONE` const.
    ///
    ///*This API requires the following crate features to be activated: `SvgfeConvolveMatrixElement`*

    pub const SVG_EDGEMODE_NONE: u16 = 3u64 as u16;
}
