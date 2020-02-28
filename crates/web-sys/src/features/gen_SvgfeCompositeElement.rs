use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = SvgElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = SVGFECompositeElement , typescript_name = SVGFECompositeElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgfeCompositeElement` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFECompositeElement)\n\n*This API requires the following crate features to be activated: `SvgfeCompositeElement`*"]
    pub type SvgfeCompositeElement;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFECompositeElement" , js_name = in1 ) ]
    #[cfg(feature = "SvgAnimatedString")]
    #[doc = "Getter for the `in1` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFECompositeElement/in1)\n\n*This API requires the following crate features to be activated: `SvgAnimatedString`, `SvgfeCompositeElement`*"]
    pub fn in1(this: &SvgfeCompositeElement) -> SvgAnimatedString;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFECompositeElement" , js_name = in2 ) ]
    #[cfg(feature = "SvgAnimatedString")]
    #[doc = "Getter for the `in2` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFECompositeElement/in2)\n\n*This API requires the following crate features to be activated: `SvgAnimatedString`, `SvgfeCompositeElement`*"]
    pub fn in2(this: &SvgfeCompositeElement) -> SvgAnimatedString;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFECompositeElement" , js_name = operator ) ]
    #[cfg(feature = "SvgAnimatedEnumeration")]
    #[doc = "Getter for the `operator` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFECompositeElement/operator)\n\n*This API requires the following crate features to be activated: `SvgAnimatedEnumeration`, `SvgfeCompositeElement`*"]
    pub fn operator(this: &SvgfeCompositeElement) -> SvgAnimatedEnumeration;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFECompositeElement" , js_name = k1 ) ]
    #[cfg(feature = "SvgAnimatedNumber")]
    #[doc = "Getter for the `k1` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFECompositeElement/k1)\n\n*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgfeCompositeElement`*"]
    pub fn k1(this: &SvgfeCompositeElement) -> SvgAnimatedNumber;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFECompositeElement" , js_name = k2 ) ]
    #[cfg(feature = "SvgAnimatedNumber")]
    #[doc = "Getter for the `k2` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFECompositeElement/k2)\n\n*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgfeCompositeElement`*"]
    pub fn k2(this: &SvgfeCompositeElement) -> SvgAnimatedNumber;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFECompositeElement" , js_name = k3 ) ]
    #[cfg(feature = "SvgAnimatedNumber")]
    #[doc = "Getter for the `k3` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFECompositeElement/k3)\n\n*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgfeCompositeElement`*"]
    pub fn k3(this: &SvgfeCompositeElement) -> SvgAnimatedNumber;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFECompositeElement" , js_name = k4 ) ]
    #[cfg(feature = "SvgAnimatedNumber")]
    #[doc = "Getter for the `k4` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFECompositeElement/k4)\n\n*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgfeCompositeElement`*"]
    pub fn k4(this: &SvgfeCompositeElement) -> SvgAnimatedNumber;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFECompositeElement" , js_name = x ) ]
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `x` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFECompositeElement/x)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeCompositeElement`*"]
    pub fn x(this: &SvgfeCompositeElement) -> SvgAnimatedLength;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFECompositeElement" , js_name = y ) ]
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `y` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFECompositeElement/y)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeCompositeElement`*"]
    pub fn y(this: &SvgfeCompositeElement) -> SvgAnimatedLength;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFECompositeElement" , js_name = width ) ]
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `width` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFECompositeElement/width)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeCompositeElement`*"]
    pub fn width(this: &SvgfeCompositeElement) -> SvgAnimatedLength;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFECompositeElement" , js_name = height ) ]
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `height` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFECompositeElement/height)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeCompositeElement`*"]
    pub fn height(this: &SvgfeCompositeElement) -> SvgAnimatedLength;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFECompositeElement" , js_name = result ) ]
    #[cfg(feature = "SvgAnimatedString")]
    #[doc = "Getter for the `result` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFECompositeElement/result)\n\n*This API requires the following crate features to be activated: `SvgAnimatedString`, `SvgfeCompositeElement`*"]
    pub fn result(this: &SvgfeCompositeElement) -> SvgAnimatedString;
}
impl SvgfeCompositeElement {
    pub const SVG_FECOMPOSITE_OPERATOR_UNKNOWN: u16 = 0i64 as u16;
    pub const SVG_FECOMPOSITE_OPERATOR_OVER: u16 = 1u64 as u16;
    pub const SVG_FECOMPOSITE_OPERATOR_IN: u16 = 2u64 as u16;
    pub const SVG_FECOMPOSITE_OPERATOR_OUT: u16 = 3u64 as u16;
    pub const SVG_FECOMPOSITE_OPERATOR_ATOP: u16 = 4u64 as u16;
    pub const SVG_FECOMPOSITE_OPERATOR_XOR: u16 = 5u64 as u16;
    pub const SVG_FECOMPOSITE_OPERATOR_ARITHMETIC: u16 = 6u64 as u16;
}
