use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = SvgElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = SVGFEConvolveMatrixElement , typescript_name = SVGFEConvolveMatrixElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgfeConvolveMatrixElement` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEConvolveMatrixElement)\n\n*This API requires the following crate features to be activated: `SvgfeConvolveMatrixElement`*"]
    pub type SvgfeConvolveMatrixElement;
    # [ wasm_bindgen ( structural , method , getter , js_name = in1 ) ]
    #[cfg(feature = "SvgAnimatedString")]
    #[doc = "Getter for the `in1` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEConvolveMatrixElement/in1)\n\n*This API requires the following crate features to be activated: `SvgAnimatedString`, `SvgfeConvolveMatrixElement`*"]
    pub fn in1(this: &SvgfeConvolveMatrixElement) -> SvgAnimatedString;
    # [ wasm_bindgen ( structural , method , getter , js_name = orderX ) ]
    #[cfg(feature = "SvgAnimatedInteger")]
    #[doc = "Getter for the `orderX` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEConvolveMatrixElement/orderX)\n\n*This API requires the following crate features to be activated: `SvgAnimatedInteger`, `SvgfeConvolveMatrixElement`*"]
    pub fn order_x(this: &SvgfeConvolveMatrixElement) -> SvgAnimatedInteger;
    # [ wasm_bindgen ( structural , method , getter , js_name = orderY ) ]
    #[cfg(feature = "SvgAnimatedInteger")]
    #[doc = "Getter for the `orderY` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEConvolveMatrixElement/orderY)\n\n*This API requires the following crate features to be activated: `SvgAnimatedInteger`, `SvgfeConvolveMatrixElement`*"]
    pub fn order_y(this: &SvgfeConvolveMatrixElement) -> SvgAnimatedInteger;
    # [ wasm_bindgen ( structural , method , getter , js_name = kernelMatrix ) ]
    #[cfg(feature = "SvgAnimatedNumberList")]
    #[doc = "Getter for the `kernelMatrix` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEConvolveMatrixElement/kernelMatrix)\n\n*This API requires the following crate features to be activated: `SvgAnimatedNumberList`, `SvgfeConvolveMatrixElement`*"]
    pub fn kernel_matrix(this: &SvgfeConvolveMatrixElement) -> SvgAnimatedNumberList;
    # [ wasm_bindgen ( structural , method , getter , js_name = divisor ) ]
    #[cfg(feature = "SvgAnimatedNumber")]
    #[doc = "Getter for the `divisor` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEConvolveMatrixElement/divisor)\n\n*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgfeConvolveMatrixElement`*"]
    pub fn divisor(this: &SvgfeConvolveMatrixElement) -> SvgAnimatedNumber;
    # [ wasm_bindgen ( structural , method , getter , js_name = bias ) ]
    #[cfg(feature = "SvgAnimatedNumber")]
    #[doc = "Getter for the `bias` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEConvolveMatrixElement/bias)\n\n*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgfeConvolveMatrixElement`*"]
    pub fn bias(this: &SvgfeConvolveMatrixElement) -> SvgAnimatedNumber;
    # [ wasm_bindgen ( structural , method , getter , js_name = targetX ) ]
    #[cfg(feature = "SvgAnimatedInteger")]
    #[doc = "Getter for the `targetX` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEConvolveMatrixElement/targetX)\n\n*This API requires the following crate features to be activated: `SvgAnimatedInteger`, `SvgfeConvolveMatrixElement`*"]
    pub fn target_x(this: &SvgfeConvolveMatrixElement) -> SvgAnimatedInteger;
    # [ wasm_bindgen ( structural , method , getter , js_name = targetY ) ]
    #[cfg(feature = "SvgAnimatedInteger")]
    #[doc = "Getter for the `targetY` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEConvolveMatrixElement/targetY)\n\n*This API requires the following crate features to be activated: `SvgAnimatedInteger`, `SvgfeConvolveMatrixElement`*"]
    pub fn target_y(this: &SvgfeConvolveMatrixElement) -> SvgAnimatedInteger;
    # [ wasm_bindgen ( structural , method , getter , js_name = edgeMode ) ]
    #[cfg(feature = "SvgAnimatedEnumeration")]
    #[doc = "Getter for the `edgeMode` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEConvolveMatrixElement/edgeMode)\n\n*This API requires the following crate features to be activated: `SvgAnimatedEnumeration`, `SvgfeConvolveMatrixElement`*"]
    pub fn edge_mode(this: &SvgfeConvolveMatrixElement) -> SvgAnimatedEnumeration;
    # [ wasm_bindgen ( structural , method , getter , js_name = kernelUnitLengthX ) ]
    #[cfg(feature = "SvgAnimatedNumber")]
    #[doc = "Getter for the `kernelUnitLengthX` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEConvolveMatrixElement/kernelUnitLengthX)\n\n*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgfeConvolveMatrixElement`*"]
    pub fn kernel_unit_length_x(this: &SvgfeConvolveMatrixElement) -> SvgAnimatedNumber;
    # [ wasm_bindgen ( structural , method , getter , js_name = kernelUnitLengthY ) ]
    #[cfg(feature = "SvgAnimatedNumber")]
    #[doc = "Getter for the `kernelUnitLengthY` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEConvolveMatrixElement/kernelUnitLengthY)\n\n*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgfeConvolveMatrixElement`*"]
    pub fn kernel_unit_length_y(this: &SvgfeConvolveMatrixElement) -> SvgAnimatedNumber;
    # [ wasm_bindgen ( structural , method , getter , js_name = preserveAlpha ) ]
    #[cfg(feature = "SvgAnimatedBoolean")]
    #[doc = "Getter for the `preserveAlpha` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEConvolveMatrixElement/preserveAlpha)\n\n*This API requires the following crate features to be activated: `SvgAnimatedBoolean`, `SvgfeConvolveMatrixElement`*"]
    pub fn preserve_alpha(this: &SvgfeConvolveMatrixElement) -> SvgAnimatedBoolean;
    # [ wasm_bindgen ( structural , method , getter , js_name = x ) ]
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `x` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEConvolveMatrixElement/x)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeConvolveMatrixElement`*"]
    pub fn x(this: &SvgfeConvolveMatrixElement) -> SvgAnimatedLength;
    # [ wasm_bindgen ( structural , method , getter , js_name = y ) ]
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `y` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEConvolveMatrixElement/y)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeConvolveMatrixElement`*"]
    pub fn y(this: &SvgfeConvolveMatrixElement) -> SvgAnimatedLength;
    # [ wasm_bindgen ( structural , method , getter , js_name = width ) ]
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `width` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEConvolveMatrixElement/width)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeConvolveMatrixElement`*"]
    pub fn width(this: &SvgfeConvolveMatrixElement) -> SvgAnimatedLength;
    # [ wasm_bindgen ( structural , method , getter , js_name = height ) ]
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `height` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEConvolveMatrixElement/height)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeConvolveMatrixElement`*"]
    pub fn height(this: &SvgfeConvolveMatrixElement) -> SvgAnimatedLength;
    # [ wasm_bindgen ( structural , method , getter , js_name = result ) ]
    #[cfg(feature = "SvgAnimatedString")]
    #[doc = "Getter for the `result` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEConvolveMatrixElement/result)\n\n*This API requires the following crate features to be activated: `SvgAnimatedString`, `SvgfeConvolveMatrixElement`*"]
    pub fn result(this: &SvgfeConvolveMatrixElement) -> SvgAnimatedString;
}
impl SvgfeConvolveMatrixElement {
    pub const SVG_EDGEMODE_UNKNOWN: u16 = 0i64 as u16;
    pub const SVG_EDGEMODE_DUPLICATE: u16 = 1u64 as u16;
    pub const SVG_EDGEMODE_WRAP: u16 = 2u64 as u16;
    pub const SVG_EDGEMODE_NONE: u16 = 3u64 as u16;
}
