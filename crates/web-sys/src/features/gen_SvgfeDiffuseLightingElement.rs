use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = SvgElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = SVGFEDiffuseLightingElement , typescript_name = SVGFEDiffuseLightingElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgfeDiffuseLightingElement` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDiffuseLightingElement)\n\n*This API requires the following crate features to be activated: `SvgfeDiffuseLightingElement`*"]
    pub type SvgfeDiffuseLightingElement;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFEDiffuseLightingElement" , js_name = in1 ) ]
    #[cfg(feature = "SvgAnimatedString")]
    #[doc = "Getter for the `in1` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDiffuseLightingElement/in1)\n\n*This API requires the following crate features to be activated: `SvgAnimatedString`, `SvgfeDiffuseLightingElement`*"]
    pub fn in1(this: &SvgfeDiffuseLightingElement) -> SvgAnimatedString;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFEDiffuseLightingElement" , js_name = surfaceScale ) ]
    #[cfg(feature = "SvgAnimatedNumber")]
    #[doc = "Getter for the `surfaceScale` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDiffuseLightingElement/surfaceScale)\n\n*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgfeDiffuseLightingElement`*"]
    pub fn surface_scale(this: &SvgfeDiffuseLightingElement) -> SvgAnimatedNumber;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFEDiffuseLightingElement" , js_name = diffuseConstant ) ]
    #[cfg(feature = "SvgAnimatedNumber")]
    #[doc = "Getter for the `diffuseConstant` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDiffuseLightingElement/diffuseConstant)\n\n*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgfeDiffuseLightingElement`*"]
    pub fn diffuse_constant(this: &SvgfeDiffuseLightingElement) -> SvgAnimatedNumber;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFEDiffuseLightingElement" , js_name = kernelUnitLengthX ) ]
    #[cfg(feature = "SvgAnimatedNumber")]
    #[doc = "Getter for the `kernelUnitLengthX` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDiffuseLightingElement/kernelUnitLengthX)\n\n*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgfeDiffuseLightingElement`*"]
    pub fn kernel_unit_length_x(this: &SvgfeDiffuseLightingElement) -> SvgAnimatedNumber;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFEDiffuseLightingElement" , js_name = kernelUnitLengthY ) ]
    #[cfg(feature = "SvgAnimatedNumber")]
    #[doc = "Getter for the `kernelUnitLengthY` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDiffuseLightingElement/kernelUnitLengthY)\n\n*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgfeDiffuseLightingElement`*"]
    pub fn kernel_unit_length_y(this: &SvgfeDiffuseLightingElement) -> SvgAnimatedNumber;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFEDiffuseLightingElement" , js_name = x ) ]
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `x` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDiffuseLightingElement/x)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeDiffuseLightingElement`*"]
    pub fn x(this: &SvgfeDiffuseLightingElement) -> SvgAnimatedLength;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFEDiffuseLightingElement" , js_name = y ) ]
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `y` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDiffuseLightingElement/y)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeDiffuseLightingElement`*"]
    pub fn y(this: &SvgfeDiffuseLightingElement) -> SvgAnimatedLength;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFEDiffuseLightingElement" , js_name = width ) ]
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `width` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDiffuseLightingElement/width)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeDiffuseLightingElement`*"]
    pub fn width(this: &SvgfeDiffuseLightingElement) -> SvgAnimatedLength;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFEDiffuseLightingElement" , js_name = height ) ]
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `height` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDiffuseLightingElement/height)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeDiffuseLightingElement`*"]
    pub fn height(this: &SvgfeDiffuseLightingElement) -> SvgAnimatedLength;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFEDiffuseLightingElement" , js_name = result ) ]
    #[cfg(feature = "SvgAnimatedString")]
    #[doc = "Getter for the `result` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDiffuseLightingElement/result)\n\n*This API requires the following crate features to be activated: `SvgAnimatedString`, `SvgfeDiffuseLightingElement`*"]
    pub fn result(this: &SvgfeDiffuseLightingElement) -> SvgAnimatedString;
}
