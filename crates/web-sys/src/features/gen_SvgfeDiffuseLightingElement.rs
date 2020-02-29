use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = SvgElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = SVGFEDiffuseLightingElement , typescript_type = "SVGFEDiffuseLightingElement" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `SvgfeDiffuseLightingElement` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDiffuseLightingElement)
    ///
    ///*This API requires the following crate features to be activated: `SvgfeDiffuseLightingElement`*
    pub type SvgfeDiffuseLightingElement;

    #[cfg(feature = "SvgAnimatedString")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFEDiffuseLightingElement" , js_name = in1 ) ]
    ///Getter for the `in1` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDiffuseLightingElement/in1)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedString`, `SvgfeDiffuseLightingElement`*
    pub fn in1(this: &SvgfeDiffuseLightingElement) -> SvgAnimatedString;

    #[cfg(feature = "SvgAnimatedNumber")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFEDiffuseLightingElement" , js_name = surfaceScale ) ]
    ///Getter for the `surfaceScale` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDiffuseLightingElement/surfaceScale)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgfeDiffuseLightingElement`*
    pub fn surface_scale(this: &SvgfeDiffuseLightingElement) -> SvgAnimatedNumber;

    #[cfg(feature = "SvgAnimatedNumber")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFEDiffuseLightingElement" , js_name = diffuseConstant ) ]
    ///Getter for the `diffuseConstant` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDiffuseLightingElement/diffuseConstant)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgfeDiffuseLightingElement`*
    pub fn diffuse_constant(this: &SvgfeDiffuseLightingElement) -> SvgAnimatedNumber;

    #[cfg(feature = "SvgAnimatedNumber")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFEDiffuseLightingElement" , js_name = kernelUnitLengthX ) ]
    ///Getter for the `kernelUnitLengthX` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDiffuseLightingElement/kernelUnitLengthX)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgfeDiffuseLightingElement`*
    pub fn kernel_unit_length_x(this: &SvgfeDiffuseLightingElement) -> SvgAnimatedNumber;

    #[cfg(feature = "SvgAnimatedNumber")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFEDiffuseLightingElement" , js_name = kernelUnitLengthY ) ]
    ///Getter for the `kernelUnitLengthY` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDiffuseLightingElement/kernelUnitLengthY)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgfeDiffuseLightingElement`*
    pub fn kernel_unit_length_y(this: &SvgfeDiffuseLightingElement) -> SvgAnimatedNumber;

    #[cfg(feature = "SvgAnimatedLength")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFEDiffuseLightingElement" , js_name = x ) ]
    ///Getter for the `x` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDiffuseLightingElement/x)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeDiffuseLightingElement`*
    pub fn x(this: &SvgfeDiffuseLightingElement) -> SvgAnimatedLength;

    #[cfg(feature = "SvgAnimatedLength")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFEDiffuseLightingElement" , js_name = y ) ]
    ///Getter for the `y` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDiffuseLightingElement/y)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeDiffuseLightingElement`*
    pub fn y(this: &SvgfeDiffuseLightingElement) -> SvgAnimatedLength;

    #[cfg(feature = "SvgAnimatedLength")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFEDiffuseLightingElement" , js_name = width ) ]
    ///Getter for the `width` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDiffuseLightingElement/width)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeDiffuseLightingElement`*
    pub fn width(this: &SvgfeDiffuseLightingElement) -> SvgAnimatedLength;

    #[cfg(feature = "SvgAnimatedLength")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFEDiffuseLightingElement" , js_name = height ) ]
    ///Getter for the `height` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDiffuseLightingElement/height)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeDiffuseLightingElement`*
    pub fn height(this: &SvgfeDiffuseLightingElement) -> SvgAnimatedLength;

    #[cfg(feature = "SvgAnimatedString")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFEDiffuseLightingElement" , js_name = result ) ]
    ///Getter for the `result` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDiffuseLightingElement/result)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedString`, `SvgfeDiffuseLightingElement`*
    pub fn result(this: &SvgfeDiffuseLightingElement) -> SvgAnimatedString;

}
