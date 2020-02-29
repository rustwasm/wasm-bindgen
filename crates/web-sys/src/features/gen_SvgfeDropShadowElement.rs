use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = SvgElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = SVGFEDropShadowElement , typescript_name = SVGFEDropShadowElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `SvgfeDropShadowElement` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDropShadowElement)
    ///
    ///*This API requires the following crate features to be activated: `SvgfeDropShadowElement`*
    pub type SvgfeDropShadowElement;

    #[cfg(feature = "SvgAnimatedString")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFEDropShadowElement" , js_name = in1 ) ]
    ///Getter for the `in1` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDropShadowElement/in1)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedString`, `SvgfeDropShadowElement`*
    pub fn in1(this: &SvgfeDropShadowElement) -> SvgAnimatedString;

    #[cfg(feature = "SvgAnimatedNumber")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFEDropShadowElement" , js_name = dx ) ]
    ///Getter for the `dx` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDropShadowElement/dx)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgfeDropShadowElement`*
    pub fn dx(this: &SvgfeDropShadowElement) -> SvgAnimatedNumber;

    #[cfg(feature = "SvgAnimatedNumber")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFEDropShadowElement" , js_name = dy ) ]
    ///Getter for the `dy` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDropShadowElement/dy)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgfeDropShadowElement`*
    pub fn dy(this: &SvgfeDropShadowElement) -> SvgAnimatedNumber;

    #[cfg(feature = "SvgAnimatedNumber")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFEDropShadowElement" , js_name = stdDeviationX ) ]
    ///Getter for the `stdDeviationX` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDropShadowElement/stdDeviationX)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgfeDropShadowElement`*
    pub fn std_deviation_x(this: &SvgfeDropShadowElement) -> SvgAnimatedNumber;

    #[cfg(feature = "SvgAnimatedNumber")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFEDropShadowElement" , js_name = stdDeviationY ) ]
    ///Getter for the `stdDeviationY` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDropShadowElement/stdDeviationY)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgfeDropShadowElement`*
    pub fn std_deviation_y(this: &SvgfeDropShadowElement) -> SvgAnimatedNumber;

    #[cfg(feature = "SvgAnimatedLength")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFEDropShadowElement" , js_name = x ) ]
    ///Getter for the `x` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDropShadowElement/x)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeDropShadowElement`*
    pub fn x(this: &SvgfeDropShadowElement) -> SvgAnimatedLength;

    #[cfg(feature = "SvgAnimatedLength")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFEDropShadowElement" , js_name = y ) ]
    ///Getter for the `y` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDropShadowElement/y)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeDropShadowElement`*
    pub fn y(this: &SvgfeDropShadowElement) -> SvgAnimatedLength;

    #[cfg(feature = "SvgAnimatedLength")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFEDropShadowElement" , js_name = width ) ]
    ///Getter for the `width` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDropShadowElement/width)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeDropShadowElement`*
    pub fn width(this: &SvgfeDropShadowElement) -> SvgAnimatedLength;

    #[cfg(feature = "SvgAnimatedLength")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFEDropShadowElement" , js_name = height ) ]
    ///Getter for the `height` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDropShadowElement/height)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeDropShadowElement`*
    pub fn height(this: &SvgfeDropShadowElement) -> SvgAnimatedLength;

    #[cfg(feature = "SvgAnimatedString")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFEDropShadowElement" , js_name = result ) ]
    ///Getter for the `result` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDropShadowElement/result)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedString`, `SvgfeDropShadowElement`*
    pub fn result(this: &SvgfeDropShadowElement) -> SvgAnimatedString;

    # [ wasm_bindgen ( method , structural , js_class = "SVGFEDropShadowElement" , js_name = setStdDeviation ) ]
    ///The `setStdDeviation()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDropShadowElement/setStdDeviation)
    ///
    ///*This API requires the following crate features to be activated: `SvgfeDropShadowElement`*
    pub fn set_std_deviation(
        this: &SvgfeDropShadowElement,
        std_deviation_x: f32,
        std_deviation_y: f32,
    );

}
