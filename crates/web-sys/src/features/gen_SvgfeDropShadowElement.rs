use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = SvgElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = SVGFEDropShadowElement , typescript_name = SVGFEDropShadowElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgfeDropShadowElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDropShadowElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgfeDropShadowElement`*"]
    pub type SvgfeDropShadowElement;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFEDropShadowElement" , js_name = in1 ) ]
    #[cfg(feature = "SvgAnimatedString")]
    #[doc = "Getter for the `in1` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDropShadowElement/in1)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedString`, `SvgfeDropShadowElement`*"]
    pub fn in1(this: &SvgfeDropShadowElement) -> SvgAnimatedString;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFEDropShadowElement" , js_name = dx ) ]
    #[cfg(feature = "SvgAnimatedNumber")]
    #[doc = "Getter for the `dx` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDropShadowElement/dx)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgfeDropShadowElement`*"]
    pub fn dx(this: &SvgfeDropShadowElement) -> SvgAnimatedNumber;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFEDropShadowElement" , js_name = dy ) ]
    #[cfg(feature = "SvgAnimatedNumber")]
    #[doc = "Getter for the `dy` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDropShadowElement/dy)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgfeDropShadowElement`*"]
    pub fn dy(this: &SvgfeDropShadowElement) -> SvgAnimatedNumber;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFEDropShadowElement" , js_name = stdDeviationX ) ]
    #[cfg(feature = "SvgAnimatedNumber")]
    #[doc = "Getter for the `stdDeviationX` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDropShadowElement/stdDeviationX)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgfeDropShadowElement`*"]
    pub fn std_deviation_x(this: &SvgfeDropShadowElement) -> SvgAnimatedNumber;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFEDropShadowElement" , js_name = stdDeviationY ) ]
    #[cfg(feature = "SvgAnimatedNumber")]
    #[doc = "Getter for the `stdDeviationY` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDropShadowElement/stdDeviationY)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgfeDropShadowElement`*"]
    pub fn std_deviation_y(this: &SvgfeDropShadowElement) -> SvgAnimatedNumber;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFEDropShadowElement" , js_name = x ) ]
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `x` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDropShadowElement/x)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeDropShadowElement`*"]
    pub fn x(this: &SvgfeDropShadowElement) -> SvgAnimatedLength;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFEDropShadowElement" , js_name = y ) ]
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `y` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDropShadowElement/y)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeDropShadowElement`*"]
    pub fn y(this: &SvgfeDropShadowElement) -> SvgAnimatedLength;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFEDropShadowElement" , js_name = width ) ]
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `width` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDropShadowElement/width)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeDropShadowElement`*"]
    pub fn width(this: &SvgfeDropShadowElement) -> SvgAnimatedLength;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFEDropShadowElement" , js_name = height ) ]
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `height` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDropShadowElement/height)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeDropShadowElement`*"]
    pub fn height(this: &SvgfeDropShadowElement) -> SvgAnimatedLength;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFEDropShadowElement" , js_name = result ) ]
    #[cfg(feature = "SvgAnimatedString")]
    #[doc = "Getter for the `result` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDropShadowElement/result)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedString`, `SvgfeDropShadowElement`*"]
    pub fn result(this: &SvgfeDropShadowElement) -> SvgAnimatedString;
    # [ wasm_bindgen ( method , structural , js_class = "SVGFEDropShadowElement" , js_name = setStdDeviation ) ]
    #[doc = "The `setStdDeviation()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDropShadowElement/setStdDeviation)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgfeDropShadowElement`*"]
    pub fn set_std_deviation(
        this: &SvgfeDropShadowElement,
        std_deviation_x: f32,
        std_deviation_y: f32,
    );
}
