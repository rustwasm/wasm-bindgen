use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = SvgElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = SVGFEPointLightElement , typescript_name = SVGFEPointLightElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgfePointLightElement` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEPointLightElement)\n\n*This API requires the following crate features to be activated: `SvgfePointLightElement`*"]
    pub type SvgfePointLightElement;
    # [ wasm_bindgen ( structural , method , getter , js_name = x ) ]
    #[cfg(feature = "SvgAnimatedNumber")]
    #[doc = "Getter for the `x` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEPointLightElement/x)\n\n*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgfePointLightElement`*"]
    pub fn x(this: &SvgfePointLightElement) -> SvgAnimatedNumber;
    # [ wasm_bindgen ( structural , method , getter , js_name = y ) ]
    #[cfg(feature = "SvgAnimatedNumber")]
    #[doc = "Getter for the `y` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEPointLightElement/y)\n\n*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgfePointLightElement`*"]
    pub fn y(this: &SvgfePointLightElement) -> SvgAnimatedNumber;
    # [ wasm_bindgen ( structural , method , getter , js_name = z ) ]
    #[cfg(feature = "SvgAnimatedNumber")]
    #[doc = "Getter for the `z` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEPointLightElement/z)\n\n*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgfePointLightElement`*"]
    pub fn z(this: &SvgfePointLightElement) -> SvgAnimatedNumber;
}
