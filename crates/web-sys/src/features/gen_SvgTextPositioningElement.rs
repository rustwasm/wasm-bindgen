use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = SvgTextContentElement , extends = SvgGraphicsElement , extends = SvgElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = SVGTextPositioningElement , typescript_name = SVGTextPositioningElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgTextPositioningElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTextPositioningElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgTextPositioningElement`*"]
    pub type SvgTextPositioningElement;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGTextPositioningElement" , js_name = x ) ]
    #[cfg(feature = "SvgAnimatedLengthList")]
    #[doc = "Getter for the `x` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTextPositioningElement/x)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedLengthList`, `SvgTextPositioningElement`*"]
    pub fn x(this: &SvgTextPositioningElement) -> SvgAnimatedLengthList;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGTextPositioningElement" , js_name = y ) ]
    #[cfg(feature = "SvgAnimatedLengthList")]
    #[doc = "Getter for the `y` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTextPositioningElement/y)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedLengthList`, `SvgTextPositioningElement`*"]
    pub fn y(this: &SvgTextPositioningElement) -> SvgAnimatedLengthList;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGTextPositioningElement" , js_name = dx ) ]
    #[cfg(feature = "SvgAnimatedLengthList")]
    #[doc = "Getter for the `dx` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTextPositioningElement/dx)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedLengthList`, `SvgTextPositioningElement`*"]
    pub fn dx(this: &SvgTextPositioningElement) -> SvgAnimatedLengthList;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGTextPositioningElement" , js_name = dy ) ]
    #[cfg(feature = "SvgAnimatedLengthList")]
    #[doc = "Getter for the `dy` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTextPositioningElement/dy)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedLengthList`, `SvgTextPositioningElement`*"]
    pub fn dy(this: &SvgTextPositioningElement) -> SvgAnimatedLengthList;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGTextPositioningElement" , js_name = rotate ) ]
    #[cfg(feature = "SvgAnimatedNumberList")]
    #[doc = "Getter for the `rotate` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTextPositioningElement/rotate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedNumberList`, `SvgTextPositioningElement`*"]
    pub fn rotate(this: &SvgTextPositioningElement) -> SvgAnimatedNumberList;
}
