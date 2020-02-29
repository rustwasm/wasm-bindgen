use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = SvgGeometryElement , extends = SvgGraphicsElement , extends = SvgElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = SVGLineElement , typescript_name = SVGLineElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `SvgLineElement` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGLineElement)
    ///
    ///*This API requires the following crate features to be activated: `SvgLineElement`*
    pub type SvgLineElement;

    #[cfg(feature = "SvgAnimatedLength")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGLineElement" , js_name = x1 ) ]
    ///Getter for the `x1` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGLineElement/x1)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgLineElement`*
    pub fn x1(this: &SvgLineElement) -> SvgAnimatedLength;

    #[cfg(feature = "SvgAnimatedLength")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGLineElement" , js_name = y1 ) ]
    ///Getter for the `y1` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGLineElement/y1)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgLineElement`*
    pub fn y1(this: &SvgLineElement) -> SvgAnimatedLength;

    #[cfg(feature = "SvgAnimatedLength")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGLineElement" , js_name = x2 ) ]
    ///Getter for the `x2` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGLineElement/x2)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgLineElement`*
    pub fn x2(this: &SvgLineElement) -> SvgAnimatedLength;

    #[cfg(feature = "SvgAnimatedLength")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGLineElement" , js_name = y2 ) ]
    ///Getter for the `y2` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGLineElement/y2)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgLineElement`*
    pub fn y2(this: &SvgLineElement) -> SvgAnimatedLength;

}
