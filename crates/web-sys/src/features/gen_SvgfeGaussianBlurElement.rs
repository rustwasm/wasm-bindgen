use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = SvgElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = SVGFEGaussianBlurElement , typescript_name = SVGFEGaussianBlurElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgfeGaussianBlurElement` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEGaussianBlurElement)\n\n*This API requires the following crate features to be activated: `SvgfeGaussianBlurElement`*"]
    pub type SvgfeGaussianBlurElement;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFEGaussianBlurElement" , js_name = in1 ) ]
    #[cfg(feature = "SvgAnimatedString")]
    #[doc = "Getter for the `in1` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEGaussianBlurElement/in1)\n\n*This API requires the following crate features to be activated: `SvgAnimatedString`, `SvgfeGaussianBlurElement`*"]
    pub fn in1(this: &SvgfeGaussianBlurElement) -> SvgAnimatedString;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFEGaussianBlurElement" , js_name = stdDeviationX ) ]
    #[cfg(feature = "SvgAnimatedNumber")]
    #[doc = "Getter for the `stdDeviationX` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEGaussianBlurElement/stdDeviationX)\n\n*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgfeGaussianBlurElement`*"]
    pub fn std_deviation_x(this: &SvgfeGaussianBlurElement) -> SvgAnimatedNumber;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFEGaussianBlurElement" , js_name = stdDeviationY ) ]
    #[cfg(feature = "SvgAnimatedNumber")]
    #[doc = "Getter for the `stdDeviationY` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEGaussianBlurElement/stdDeviationY)\n\n*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgfeGaussianBlurElement`*"]
    pub fn std_deviation_y(this: &SvgfeGaussianBlurElement) -> SvgAnimatedNumber;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFEGaussianBlurElement" , js_name = x ) ]
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `x` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEGaussianBlurElement/x)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeGaussianBlurElement`*"]
    pub fn x(this: &SvgfeGaussianBlurElement) -> SvgAnimatedLength;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFEGaussianBlurElement" , js_name = y ) ]
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `y` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEGaussianBlurElement/y)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeGaussianBlurElement`*"]
    pub fn y(this: &SvgfeGaussianBlurElement) -> SvgAnimatedLength;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFEGaussianBlurElement" , js_name = width ) ]
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `width` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEGaussianBlurElement/width)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeGaussianBlurElement`*"]
    pub fn width(this: &SvgfeGaussianBlurElement) -> SvgAnimatedLength;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFEGaussianBlurElement" , js_name = height ) ]
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `height` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEGaussianBlurElement/height)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeGaussianBlurElement`*"]
    pub fn height(this: &SvgfeGaussianBlurElement) -> SvgAnimatedLength;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFEGaussianBlurElement" , js_name = result ) ]
    #[cfg(feature = "SvgAnimatedString")]
    #[doc = "Getter for the `result` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEGaussianBlurElement/result)\n\n*This API requires the following crate features to be activated: `SvgAnimatedString`, `SvgfeGaussianBlurElement`*"]
    pub fn result(this: &SvgfeGaussianBlurElement) -> SvgAnimatedString;
    # [ wasm_bindgen ( method , structural , js_class = "SVGFEGaussianBlurElement" , js_name = setStdDeviation ) ]
    #[doc = "The `setStdDeviation()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEGaussianBlurElement/setStdDeviation)\n\n*This API requires the following crate features to be activated: `SvgfeGaussianBlurElement`*"]
    pub fn set_std_deviation(
        this: &SvgfeGaussianBlurElement,
        std_deviation_x: f32,
        std_deviation_y: f32,
    );
}
