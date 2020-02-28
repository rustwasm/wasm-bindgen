use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = SvgElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = SVGFESpecularLightingElement , typescript_name = SVGFESpecularLightingElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgfeSpecularLightingElement` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFESpecularLightingElement)\n\n*This API requires the following crate features to be activated: `SvgfeSpecularLightingElement`*"]
    pub type SvgfeSpecularLightingElement;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFESpecularLightingElement" , js_name = in1 ) ]
    #[cfg(feature = "SvgAnimatedString")]
    #[doc = "Getter for the `in1` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFESpecularLightingElement/in1)\n\n*This API requires the following crate features to be activated: `SvgAnimatedString`, `SvgfeSpecularLightingElement`*"]
    pub fn in1(this: &SvgfeSpecularLightingElement) -> SvgAnimatedString;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFESpecularLightingElement" , js_name = surfaceScale ) ]
    #[cfg(feature = "SvgAnimatedNumber")]
    #[doc = "Getter for the `surfaceScale` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFESpecularLightingElement/surfaceScale)\n\n*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgfeSpecularLightingElement`*"]
    pub fn surface_scale(this: &SvgfeSpecularLightingElement) -> SvgAnimatedNumber;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFESpecularLightingElement" , js_name = specularConstant ) ]
    #[cfg(feature = "SvgAnimatedNumber")]
    #[doc = "Getter for the `specularConstant` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFESpecularLightingElement/specularConstant)\n\n*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgfeSpecularLightingElement`*"]
    pub fn specular_constant(this: &SvgfeSpecularLightingElement) -> SvgAnimatedNumber;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFESpecularLightingElement" , js_name = specularExponent ) ]
    #[cfg(feature = "SvgAnimatedNumber")]
    #[doc = "Getter for the `specularExponent` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFESpecularLightingElement/specularExponent)\n\n*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgfeSpecularLightingElement`*"]
    pub fn specular_exponent(this: &SvgfeSpecularLightingElement) -> SvgAnimatedNumber;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFESpecularLightingElement" , js_name = kernelUnitLengthX ) ]
    #[cfg(feature = "SvgAnimatedNumber")]
    #[doc = "Getter for the `kernelUnitLengthX` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFESpecularLightingElement/kernelUnitLengthX)\n\n*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgfeSpecularLightingElement`*"]
    pub fn kernel_unit_length_x(this: &SvgfeSpecularLightingElement) -> SvgAnimatedNumber;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFESpecularLightingElement" , js_name = kernelUnitLengthY ) ]
    #[cfg(feature = "SvgAnimatedNumber")]
    #[doc = "Getter for the `kernelUnitLengthY` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFESpecularLightingElement/kernelUnitLengthY)\n\n*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgfeSpecularLightingElement`*"]
    pub fn kernel_unit_length_y(this: &SvgfeSpecularLightingElement) -> SvgAnimatedNumber;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFESpecularLightingElement" , js_name = x ) ]
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `x` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFESpecularLightingElement/x)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeSpecularLightingElement`*"]
    pub fn x(this: &SvgfeSpecularLightingElement) -> SvgAnimatedLength;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFESpecularLightingElement" , js_name = y ) ]
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `y` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFESpecularLightingElement/y)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeSpecularLightingElement`*"]
    pub fn y(this: &SvgfeSpecularLightingElement) -> SvgAnimatedLength;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFESpecularLightingElement" , js_name = width ) ]
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `width` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFESpecularLightingElement/width)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeSpecularLightingElement`*"]
    pub fn width(this: &SvgfeSpecularLightingElement) -> SvgAnimatedLength;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFESpecularLightingElement" , js_name = height ) ]
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `height` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFESpecularLightingElement/height)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeSpecularLightingElement`*"]
    pub fn height(this: &SvgfeSpecularLightingElement) -> SvgAnimatedLength;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFESpecularLightingElement" , js_name = result ) ]
    #[cfg(feature = "SvgAnimatedString")]
    #[doc = "Getter for the `result` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFESpecularLightingElement/result)\n\n*This API requires the following crate features to be activated: `SvgAnimatedString`, `SvgfeSpecularLightingElement`*"]
    pub fn result(this: &SvgfeSpecularLightingElement) -> SvgAnimatedString;
}
