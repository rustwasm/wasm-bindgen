use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = SvgElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = SVGFESpecularLightingElement , typescript_type = "SVGFESpecularLightingElement" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `SvgfeSpecularLightingElement` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFESpecularLightingElement)
    ///
    ///*This API requires the following crate features to be activated: `SvgfeSpecularLightingElement`*
    pub type SvgfeSpecularLightingElement;

    #[cfg(feature = "SvgAnimatedString")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFESpecularLightingElement" , js_name = in1 ) ]
    ///Getter for the `in1` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFESpecularLightingElement/in1)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedString`, `SvgfeSpecularLightingElement`*
    pub fn in1(this: &SvgfeSpecularLightingElement) -> SvgAnimatedString;

    #[cfg(feature = "SvgAnimatedNumber")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFESpecularLightingElement" , js_name = surfaceScale ) ]
    ///Getter for the `surfaceScale` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFESpecularLightingElement/surfaceScale)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgfeSpecularLightingElement`*
    pub fn surface_scale(this: &SvgfeSpecularLightingElement) -> SvgAnimatedNumber;

    #[cfg(feature = "SvgAnimatedNumber")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFESpecularLightingElement" , js_name = specularConstant ) ]
    ///Getter for the `specularConstant` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFESpecularLightingElement/specularConstant)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgfeSpecularLightingElement`*
    pub fn specular_constant(this: &SvgfeSpecularLightingElement) -> SvgAnimatedNumber;

    #[cfg(feature = "SvgAnimatedNumber")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFESpecularLightingElement" , js_name = specularExponent ) ]
    ///Getter for the `specularExponent` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFESpecularLightingElement/specularExponent)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgfeSpecularLightingElement`*
    pub fn specular_exponent(this: &SvgfeSpecularLightingElement) -> SvgAnimatedNumber;

    #[cfg(feature = "SvgAnimatedNumber")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFESpecularLightingElement" , js_name = kernelUnitLengthX ) ]
    ///Getter for the `kernelUnitLengthX` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFESpecularLightingElement/kernelUnitLengthX)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgfeSpecularLightingElement`*
    pub fn kernel_unit_length_x(this: &SvgfeSpecularLightingElement) -> SvgAnimatedNumber;

    #[cfg(feature = "SvgAnimatedNumber")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFESpecularLightingElement" , js_name = kernelUnitLengthY ) ]
    ///Getter for the `kernelUnitLengthY` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFESpecularLightingElement/kernelUnitLengthY)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgfeSpecularLightingElement`*
    pub fn kernel_unit_length_y(this: &SvgfeSpecularLightingElement) -> SvgAnimatedNumber;

    #[cfg(feature = "SvgAnimatedLength")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFESpecularLightingElement" , js_name = x ) ]
    ///Getter for the `x` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFESpecularLightingElement/x)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeSpecularLightingElement`*
    pub fn x(this: &SvgfeSpecularLightingElement) -> SvgAnimatedLength;

    #[cfg(feature = "SvgAnimatedLength")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFESpecularLightingElement" , js_name = y ) ]
    ///Getter for the `y` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFESpecularLightingElement/y)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeSpecularLightingElement`*
    pub fn y(this: &SvgfeSpecularLightingElement) -> SvgAnimatedLength;

    #[cfg(feature = "SvgAnimatedLength")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFESpecularLightingElement" , js_name = width ) ]
    ///Getter for the `width` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFESpecularLightingElement/width)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeSpecularLightingElement`*
    pub fn width(this: &SvgfeSpecularLightingElement) -> SvgAnimatedLength;

    #[cfg(feature = "SvgAnimatedLength")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFESpecularLightingElement" , js_name = height ) ]
    ///Getter for the `height` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFESpecularLightingElement/height)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeSpecularLightingElement`*
    pub fn height(this: &SvgfeSpecularLightingElement) -> SvgAnimatedLength;

    #[cfg(feature = "SvgAnimatedString")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFESpecularLightingElement" , js_name = result ) ]
    ///Getter for the `result` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFESpecularLightingElement/result)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedString`, `SvgfeSpecularLightingElement`*
    pub fn result(this: &SvgfeSpecularLightingElement) -> SvgAnimatedString;

}
