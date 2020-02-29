use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = SvgElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = SVGFEOffsetElement , typescript_type = "SVGFEOffsetElement" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `SvgfeOffsetElement` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEOffsetElement)
    ///
    ///*This API requires the following crate features to be activated: `SvgfeOffsetElement`*
    pub type SvgfeOffsetElement;

    #[cfg(feature = "SvgAnimatedString")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFEOffsetElement" , js_name = in1 ) ]
    ///Getter for the `in1` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEOffsetElement/in1)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedString`, `SvgfeOffsetElement`*
    pub fn in1(this: &SvgfeOffsetElement) -> SvgAnimatedString;

    #[cfg(feature = "SvgAnimatedNumber")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFEOffsetElement" , js_name = dx ) ]
    ///Getter for the `dx` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEOffsetElement/dx)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgfeOffsetElement`*
    pub fn dx(this: &SvgfeOffsetElement) -> SvgAnimatedNumber;

    #[cfg(feature = "SvgAnimatedNumber")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFEOffsetElement" , js_name = dy ) ]
    ///Getter for the `dy` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEOffsetElement/dy)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgfeOffsetElement`*
    pub fn dy(this: &SvgfeOffsetElement) -> SvgAnimatedNumber;

    #[cfg(feature = "SvgAnimatedLength")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFEOffsetElement" , js_name = x ) ]
    ///Getter for the `x` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEOffsetElement/x)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeOffsetElement`*
    pub fn x(this: &SvgfeOffsetElement) -> SvgAnimatedLength;

    #[cfg(feature = "SvgAnimatedLength")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFEOffsetElement" , js_name = y ) ]
    ///Getter for the `y` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEOffsetElement/y)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeOffsetElement`*
    pub fn y(this: &SvgfeOffsetElement) -> SvgAnimatedLength;

    #[cfg(feature = "SvgAnimatedLength")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFEOffsetElement" , js_name = width ) ]
    ///Getter for the `width` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEOffsetElement/width)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeOffsetElement`*
    pub fn width(this: &SvgfeOffsetElement) -> SvgAnimatedLength;

    #[cfg(feature = "SvgAnimatedLength")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFEOffsetElement" , js_name = height ) ]
    ///Getter for the `height` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEOffsetElement/height)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeOffsetElement`*
    pub fn height(this: &SvgfeOffsetElement) -> SvgAnimatedLength;

    #[cfg(feature = "SvgAnimatedString")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFEOffsetElement" , js_name = result ) ]
    ///Getter for the `result` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEOffsetElement/result)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedString`, `SvgfeOffsetElement`*
    pub fn result(this: &SvgfeOffsetElement) -> SvgAnimatedString;

}
