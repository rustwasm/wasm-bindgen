use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = SvgGraphicsElement , extends = SvgElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = SVGForeignObjectElement , typescript_name = SVGForeignObjectElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgForeignObjectElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGForeignObjectElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgForeignObjectElement`*"]
    pub type SvgForeignObjectElement;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGForeignObjectElement" , js_name = x ) ]
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `x` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGForeignObjectElement/x)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgForeignObjectElement`*"]
    pub fn x(this: &SvgForeignObjectElement) -> SvgAnimatedLength;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGForeignObjectElement" , js_name = y ) ]
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `y` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGForeignObjectElement/y)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgForeignObjectElement`*"]
    pub fn y(this: &SvgForeignObjectElement) -> SvgAnimatedLength;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGForeignObjectElement" , js_name = width ) ]
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `width` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGForeignObjectElement/width)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgForeignObjectElement`*"]
    pub fn width(this: &SvgForeignObjectElement) -> SvgAnimatedLength;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGForeignObjectElement" , js_name = height ) ]
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `height` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGForeignObjectElement/height)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgForeignObjectElement`*"]
    pub fn height(this: &SvgForeignObjectElement) -> SvgAnimatedLength;
}
