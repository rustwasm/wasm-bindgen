use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = SvgGradientElement , extends = SvgElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = SVGLinearGradientElement , typescript_name = SVGLinearGradientElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `SvgLinearGradientElement` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGLinearGradientElement)
    ///
    ///*This API requires the following crate features to be activated: `SvgLinearGradientElement`*
    pub type SvgLinearGradientElement;

    #[cfg(feature = "SvgAnimatedLength")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGLinearGradientElement" , js_name = x1 ) ]
    ///Getter for the `x1` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGLinearGradientElement/x1)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgLinearGradientElement`*
    pub fn x1(this: &SvgLinearGradientElement) -> SvgAnimatedLength;

    #[cfg(feature = "SvgAnimatedLength")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGLinearGradientElement" , js_name = y1 ) ]
    ///Getter for the `y1` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGLinearGradientElement/y1)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgLinearGradientElement`*
    pub fn y1(this: &SvgLinearGradientElement) -> SvgAnimatedLength;

    #[cfg(feature = "SvgAnimatedLength")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGLinearGradientElement" , js_name = x2 ) ]
    ///Getter for the `x2` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGLinearGradientElement/x2)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgLinearGradientElement`*
    pub fn x2(this: &SvgLinearGradientElement) -> SvgAnimatedLength;

    #[cfg(feature = "SvgAnimatedLength")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGLinearGradientElement" , js_name = y2 ) ]
    ///Getter for the `y2` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGLinearGradientElement/y2)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgLinearGradientElement`*
    pub fn y2(this: &SvgLinearGradientElement) -> SvgAnimatedLength;

}
