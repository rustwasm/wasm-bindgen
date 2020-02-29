use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = SvgTextContentElement , extends = SvgGraphicsElement , extends = SvgElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = SVGTextPositioningElement , typescript_type = "SVGTextPositioningElement" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `SvgTextPositioningElement` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTextPositioningElement)
    ///
    ///*This API requires the following crate features to be activated: `SvgTextPositioningElement`*
    pub type SvgTextPositioningElement;

    #[cfg(feature = "SvgAnimatedLengthList")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGTextPositioningElement" , js_name = x ) ]
    ///Getter for the `x` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTextPositioningElement/x)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedLengthList`, `SvgTextPositioningElement`*
    pub fn x(this: &SvgTextPositioningElement) -> SvgAnimatedLengthList;

    #[cfg(feature = "SvgAnimatedLengthList")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGTextPositioningElement" , js_name = y ) ]
    ///Getter for the `y` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTextPositioningElement/y)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedLengthList`, `SvgTextPositioningElement`*
    pub fn y(this: &SvgTextPositioningElement) -> SvgAnimatedLengthList;

    #[cfg(feature = "SvgAnimatedLengthList")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGTextPositioningElement" , js_name = dx ) ]
    ///Getter for the `dx` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTextPositioningElement/dx)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedLengthList`, `SvgTextPositioningElement`*
    pub fn dx(this: &SvgTextPositioningElement) -> SvgAnimatedLengthList;

    #[cfg(feature = "SvgAnimatedLengthList")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGTextPositioningElement" , js_name = dy ) ]
    ///Getter for the `dy` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTextPositioningElement/dy)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedLengthList`, `SvgTextPositioningElement`*
    pub fn dy(this: &SvgTextPositioningElement) -> SvgAnimatedLengthList;

    #[cfg(feature = "SvgAnimatedNumberList")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGTextPositioningElement" , js_name = rotate ) ]
    ///Getter for the `rotate` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTextPositioningElement/rotate)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedNumberList`, `SvgTextPositioningElement`*
    pub fn rotate(this: &SvgTextPositioningElement) -> SvgAnimatedNumberList;

}
