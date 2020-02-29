use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = SvgElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = SVGFEGaussianBlurElement , typescript_name = SVGFEGaussianBlurElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `SvgfeGaussianBlurElement` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEGaussianBlurElement)
    ///
    ///*This API requires the following crate features to be activated: `SvgfeGaussianBlurElement`*
    pub type SvgfeGaussianBlurElement;

    #[cfg(feature = "SvgAnimatedString")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFEGaussianBlurElement" , js_name = in1 ) ]
    ///Getter for the `in1` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEGaussianBlurElement/in1)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedString`, `SvgfeGaussianBlurElement`*
    pub fn in1(this: &SvgfeGaussianBlurElement) -> SvgAnimatedString;

    #[cfg(feature = "SvgAnimatedNumber")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFEGaussianBlurElement" , js_name = stdDeviationX ) ]
    ///Getter for the `stdDeviationX` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEGaussianBlurElement/stdDeviationX)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgfeGaussianBlurElement`*
    pub fn std_deviation_x(this: &SvgfeGaussianBlurElement) -> SvgAnimatedNumber;

    #[cfg(feature = "SvgAnimatedNumber")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFEGaussianBlurElement" , js_name = stdDeviationY ) ]
    ///Getter for the `stdDeviationY` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEGaussianBlurElement/stdDeviationY)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgfeGaussianBlurElement`*
    pub fn std_deviation_y(this: &SvgfeGaussianBlurElement) -> SvgAnimatedNumber;

    #[cfg(feature = "SvgAnimatedLength")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFEGaussianBlurElement" , js_name = x ) ]
    ///Getter for the `x` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEGaussianBlurElement/x)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeGaussianBlurElement`*
    pub fn x(this: &SvgfeGaussianBlurElement) -> SvgAnimatedLength;

    #[cfg(feature = "SvgAnimatedLength")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFEGaussianBlurElement" , js_name = y ) ]
    ///Getter for the `y` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEGaussianBlurElement/y)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeGaussianBlurElement`*
    pub fn y(this: &SvgfeGaussianBlurElement) -> SvgAnimatedLength;

    #[cfg(feature = "SvgAnimatedLength")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFEGaussianBlurElement" , js_name = width ) ]
    ///Getter for the `width` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEGaussianBlurElement/width)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeGaussianBlurElement`*
    pub fn width(this: &SvgfeGaussianBlurElement) -> SvgAnimatedLength;

    #[cfg(feature = "SvgAnimatedLength")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFEGaussianBlurElement" , js_name = height ) ]
    ///Getter for the `height` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEGaussianBlurElement/height)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeGaussianBlurElement`*
    pub fn height(this: &SvgfeGaussianBlurElement) -> SvgAnimatedLength;

    #[cfg(feature = "SvgAnimatedString")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFEGaussianBlurElement" , js_name = result ) ]
    ///Getter for the `result` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEGaussianBlurElement/result)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedString`, `SvgfeGaussianBlurElement`*
    pub fn result(this: &SvgfeGaussianBlurElement) -> SvgAnimatedString;

    # [ wasm_bindgen ( method , structural , js_class = "SVGFEGaussianBlurElement" , js_name = setStdDeviation ) ]
    ///The `setStdDeviation()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEGaussianBlurElement/setStdDeviation)
    ///
    ///*This API requires the following crate features to be activated: `SvgfeGaussianBlurElement`*
    pub fn set_std_deviation(
        this: &SvgfeGaussianBlurElement,
        std_deviation_x: f32,
        std_deviation_y: f32,
    );

}
