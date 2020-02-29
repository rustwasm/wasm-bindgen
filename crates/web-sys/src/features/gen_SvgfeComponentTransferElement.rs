use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = SvgElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = SVGFEComponentTransferElement , typescript_type = "SVGFEComponentTransferElement" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `SvgfeComponentTransferElement` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEComponentTransferElement)
    ///
    ///*This API requires the following crate features to be activated: `SvgfeComponentTransferElement`*
    pub type SvgfeComponentTransferElement;

    #[cfg(feature = "SvgAnimatedString")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFEComponentTransferElement" , js_name = in1 ) ]
    ///Getter for the `in1` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEComponentTransferElement/in1)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedString`, `SvgfeComponentTransferElement`*
    pub fn in1(this: &SvgfeComponentTransferElement) -> SvgAnimatedString;

    #[cfg(feature = "SvgAnimatedLength")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFEComponentTransferElement" , js_name = x ) ]
    ///Getter for the `x` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEComponentTransferElement/x)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeComponentTransferElement`*
    pub fn x(this: &SvgfeComponentTransferElement) -> SvgAnimatedLength;

    #[cfg(feature = "SvgAnimatedLength")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFEComponentTransferElement" , js_name = y ) ]
    ///Getter for the `y` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEComponentTransferElement/y)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeComponentTransferElement`*
    pub fn y(this: &SvgfeComponentTransferElement) -> SvgAnimatedLength;

    #[cfg(feature = "SvgAnimatedLength")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFEComponentTransferElement" , js_name = width ) ]
    ///Getter for the `width` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEComponentTransferElement/width)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeComponentTransferElement`*
    pub fn width(this: &SvgfeComponentTransferElement) -> SvgAnimatedLength;

    #[cfg(feature = "SvgAnimatedLength")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFEComponentTransferElement" , js_name = height ) ]
    ///Getter for the `height` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEComponentTransferElement/height)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeComponentTransferElement`*
    pub fn height(this: &SvgfeComponentTransferElement) -> SvgAnimatedLength;

    #[cfg(feature = "SvgAnimatedString")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFEComponentTransferElement" , js_name = result ) ]
    ///Getter for the `result` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEComponentTransferElement/result)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedString`, `SvgfeComponentTransferElement`*
    pub fn result(this: &SvgfeComponentTransferElement) -> SvgAnimatedString;

}
