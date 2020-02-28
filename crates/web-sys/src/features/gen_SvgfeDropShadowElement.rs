use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = SvgElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = SVGFEDropShadowElement , typescript_name = SVGFEDropShadowElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgfeDropShadowElement` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDropShadowElement)\n\n*This API requires the following crate features to be activated: `SvgfeDropShadowElement`*"]
    pub type SvgfeDropShadowElement;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFEDropShadowElement" , js_name = in1 ) ]
    #[cfg(feature = "SvgAnimatedString")]
    #[doc = "Getter for the `in1` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDropShadowElement/in1)\n\n*This API requires the following crate features to be activated: `SvgAnimatedString`, `SvgfeDropShadowElement`*"]
    pub fn in1(this: &SvgfeDropShadowElement) -> SvgAnimatedString;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFEDropShadowElement" , js_name = dx ) ]
    #[cfg(feature = "SvgAnimatedNumber")]
    #[doc = "Getter for the `dx` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDropShadowElement/dx)\n\n*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgfeDropShadowElement`*"]
    pub fn dx(this: &SvgfeDropShadowElement) -> SvgAnimatedNumber;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFEDropShadowElement" , js_name = dy ) ]
    #[cfg(feature = "SvgAnimatedNumber")]
    #[doc = "Getter for the `dy` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDropShadowElement/dy)\n\n*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgfeDropShadowElement`*"]
    pub fn dy(this: &SvgfeDropShadowElement) -> SvgAnimatedNumber;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFEDropShadowElement" , js_name = stdDeviationX ) ]
    #[cfg(feature = "SvgAnimatedNumber")]
    #[doc = "Getter for the `stdDeviationX` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDropShadowElement/stdDeviationX)\n\n*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgfeDropShadowElement`*"]
    pub fn std_deviation_x(this: &SvgfeDropShadowElement) -> SvgAnimatedNumber;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFEDropShadowElement" , js_name = stdDeviationY ) ]
    #[cfg(feature = "SvgAnimatedNumber")]
    #[doc = "Getter for the `stdDeviationY` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDropShadowElement/stdDeviationY)\n\n*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgfeDropShadowElement`*"]
    pub fn std_deviation_y(this: &SvgfeDropShadowElement) -> SvgAnimatedNumber;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFEDropShadowElement" , js_name = x ) ]
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `x` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDropShadowElement/x)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeDropShadowElement`*"]
    pub fn x(this: &SvgfeDropShadowElement) -> SvgAnimatedLength;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFEDropShadowElement" , js_name = y ) ]
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `y` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDropShadowElement/y)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeDropShadowElement`*"]
    pub fn y(this: &SvgfeDropShadowElement) -> SvgAnimatedLength;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFEDropShadowElement" , js_name = width ) ]
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `width` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDropShadowElement/width)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeDropShadowElement`*"]
    pub fn width(this: &SvgfeDropShadowElement) -> SvgAnimatedLength;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFEDropShadowElement" , js_name = height ) ]
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `height` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDropShadowElement/height)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeDropShadowElement`*"]
    pub fn height(this: &SvgfeDropShadowElement) -> SvgAnimatedLength;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFEDropShadowElement" , js_name = result ) ]
    #[cfg(feature = "SvgAnimatedString")]
    #[doc = "Getter for the `result` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDropShadowElement/result)\n\n*This API requires the following crate features to be activated: `SvgAnimatedString`, `SvgfeDropShadowElement`*"]
    pub fn result(this: &SvgfeDropShadowElement) -> SvgAnimatedString;
    # [ wasm_bindgen ( method , structural , js_class = "SVGFEDropShadowElement" , js_name = setStdDeviation ) ]
    #[doc = "The `setStdDeviation()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDropShadowElement/setStdDeviation)\n\n*This API requires the following crate features to be activated: `SvgfeDropShadowElement`*"]
    pub fn set_std_deviation(
        this: &SvgfeDropShadowElement,
        std_deviation_x: f32,
        std_deviation_y: f32,
    );
}
