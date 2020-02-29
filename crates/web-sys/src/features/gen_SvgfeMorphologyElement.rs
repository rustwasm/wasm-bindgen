use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = SvgElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = SVGFEMorphologyElement , typescript_type = "SVGFEMorphologyElement" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `SvgfeMorphologyElement` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEMorphologyElement)
    ///
    ///*This API requires the following crate features to be activated: `SvgfeMorphologyElement`*
    pub type SvgfeMorphologyElement;

    #[cfg(feature = "SvgAnimatedString")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFEMorphologyElement" , js_name = in1 ) ]
    ///Getter for the `in1` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEMorphologyElement/in1)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedString`, `SvgfeMorphologyElement`*
    pub fn in1(this: &SvgfeMorphologyElement) -> SvgAnimatedString;

    #[cfg(feature = "SvgAnimatedEnumeration")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFEMorphologyElement" , js_name = operator ) ]
    ///Getter for the `operator` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEMorphologyElement/operator)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedEnumeration`, `SvgfeMorphologyElement`*
    pub fn operator(this: &SvgfeMorphologyElement) -> SvgAnimatedEnumeration;

    #[cfg(feature = "SvgAnimatedNumber")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFEMorphologyElement" , js_name = radiusX ) ]
    ///Getter for the `radiusX` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEMorphologyElement/radiusX)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgfeMorphologyElement`*
    pub fn radius_x(this: &SvgfeMorphologyElement) -> SvgAnimatedNumber;

    #[cfg(feature = "SvgAnimatedNumber")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFEMorphologyElement" , js_name = radiusY ) ]
    ///Getter for the `radiusY` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEMorphologyElement/radiusY)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgfeMorphologyElement`*
    pub fn radius_y(this: &SvgfeMorphologyElement) -> SvgAnimatedNumber;

    #[cfg(feature = "SvgAnimatedLength")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFEMorphologyElement" , js_name = x ) ]
    ///Getter for the `x` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEMorphologyElement/x)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeMorphologyElement`*
    pub fn x(this: &SvgfeMorphologyElement) -> SvgAnimatedLength;

    #[cfg(feature = "SvgAnimatedLength")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFEMorphologyElement" , js_name = y ) ]
    ///Getter for the `y` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEMorphologyElement/y)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeMorphologyElement`*
    pub fn y(this: &SvgfeMorphologyElement) -> SvgAnimatedLength;

    #[cfg(feature = "SvgAnimatedLength")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFEMorphologyElement" , js_name = width ) ]
    ///Getter for the `width` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEMorphologyElement/width)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeMorphologyElement`*
    pub fn width(this: &SvgfeMorphologyElement) -> SvgAnimatedLength;

    #[cfg(feature = "SvgAnimatedLength")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFEMorphologyElement" , js_name = height ) ]
    ///Getter for the `height` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEMorphologyElement/height)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeMorphologyElement`*
    pub fn height(this: &SvgfeMorphologyElement) -> SvgAnimatedLength;

    #[cfg(feature = "SvgAnimatedString")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFEMorphologyElement" , js_name = result ) ]
    ///Getter for the `result` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEMorphologyElement/result)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedString`, `SvgfeMorphologyElement`*
    pub fn result(this: &SvgfeMorphologyElement) -> SvgAnimatedString;

}

impl SvgfeMorphologyElement {
    ///The `SVGFEMorphologyElement.SVG_MORPHOLOGY_OPERATOR_UNKNOWN` const.
    ///
    ///*This API requires the following crate features to be activated: `SvgfeMorphologyElement`*

    pub const SVG_MORPHOLOGY_OPERATOR_UNKNOWN: u16 = 0i64 as u16;

    ///The `SVGFEMorphologyElement.SVG_MORPHOLOGY_OPERATOR_ERODE` const.
    ///
    ///*This API requires the following crate features to be activated: `SvgfeMorphologyElement`*

    pub const SVG_MORPHOLOGY_OPERATOR_ERODE: u16 = 1u64 as u16;

    ///The `SVGFEMorphologyElement.SVG_MORPHOLOGY_OPERATOR_DILATE` const.
    ///
    ///*This API requires the following crate features to be activated: `SvgfeMorphologyElement`*

    pub const SVG_MORPHOLOGY_OPERATOR_DILATE: u16 = 2u64 as u16;
}
