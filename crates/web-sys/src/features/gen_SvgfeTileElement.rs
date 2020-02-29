use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = SvgElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = SVGFETileElement , typescript_type = "SVGFETileElement" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `SvgfeTileElement` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFETileElement)
    ///
    ///*This API requires the following crate features to be activated: `SvgfeTileElement`*
    pub type SvgfeTileElement;

    #[cfg(feature = "SvgAnimatedString")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFETileElement" , js_name = in1 ) ]
    ///Getter for the `in1` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFETileElement/in1)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedString`, `SvgfeTileElement`*
    pub fn in1(this: &SvgfeTileElement) -> SvgAnimatedString;

    #[cfg(feature = "SvgAnimatedLength")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFETileElement" , js_name = x ) ]
    ///Getter for the `x` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFETileElement/x)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeTileElement`*
    pub fn x(this: &SvgfeTileElement) -> SvgAnimatedLength;

    #[cfg(feature = "SvgAnimatedLength")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFETileElement" , js_name = y ) ]
    ///Getter for the `y` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFETileElement/y)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeTileElement`*
    pub fn y(this: &SvgfeTileElement) -> SvgAnimatedLength;

    #[cfg(feature = "SvgAnimatedLength")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFETileElement" , js_name = width ) ]
    ///Getter for the `width` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFETileElement/width)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeTileElement`*
    pub fn width(this: &SvgfeTileElement) -> SvgAnimatedLength;

    #[cfg(feature = "SvgAnimatedLength")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFETileElement" , js_name = height ) ]
    ///Getter for the `height` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFETileElement/height)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeTileElement`*
    pub fn height(this: &SvgfeTileElement) -> SvgAnimatedLength;

    #[cfg(feature = "SvgAnimatedString")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFETileElement" , js_name = result ) ]
    ///Getter for the `result` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFETileElement/result)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedString`, `SvgfeTileElement`*
    pub fn result(this: &SvgfeTileElement) -> SvgAnimatedString;

}
