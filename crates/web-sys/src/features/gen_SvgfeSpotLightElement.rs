use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = SvgElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = SVGFESpotLightElement , typescript_name = SVGFESpotLightElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgfeSpotLightElement` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFESpotLightElement)\n\n*This API requires the following crate features to be activated: `SvgfeSpotLightElement`*"]
    pub type SvgfeSpotLightElement;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFESpotLightElement" , js_name = x ) ]
    #[cfg(feature = "SvgAnimatedNumber")]
    #[doc = "Getter for the `x` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFESpotLightElement/x)\n\n*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgfeSpotLightElement`*"]
    pub fn x(this: &SvgfeSpotLightElement) -> SvgAnimatedNumber;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFESpotLightElement" , js_name = y ) ]
    #[cfg(feature = "SvgAnimatedNumber")]
    #[doc = "Getter for the `y` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFESpotLightElement/y)\n\n*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgfeSpotLightElement`*"]
    pub fn y(this: &SvgfeSpotLightElement) -> SvgAnimatedNumber;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFESpotLightElement" , js_name = z ) ]
    #[cfg(feature = "SvgAnimatedNumber")]
    #[doc = "Getter for the `z` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFESpotLightElement/z)\n\n*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgfeSpotLightElement`*"]
    pub fn z(this: &SvgfeSpotLightElement) -> SvgAnimatedNumber;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFESpotLightElement" , js_name = pointsAtX ) ]
    #[cfg(feature = "SvgAnimatedNumber")]
    #[doc = "Getter for the `pointsAtX` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFESpotLightElement/pointsAtX)\n\n*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgfeSpotLightElement`*"]
    pub fn points_at_x(this: &SvgfeSpotLightElement) -> SvgAnimatedNumber;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFESpotLightElement" , js_name = pointsAtY ) ]
    #[cfg(feature = "SvgAnimatedNumber")]
    #[doc = "Getter for the `pointsAtY` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFESpotLightElement/pointsAtY)\n\n*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgfeSpotLightElement`*"]
    pub fn points_at_y(this: &SvgfeSpotLightElement) -> SvgAnimatedNumber;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFESpotLightElement" , js_name = pointsAtZ ) ]
    #[cfg(feature = "SvgAnimatedNumber")]
    #[doc = "Getter for the `pointsAtZ` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFESpotLightElement/pointsAtZ)\n\n*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgfeSpotLightElement`*"]
    pub fn points_at_z(this: &SvgfeSpotLightElement) -> SvgAnimatedNumber;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFESpotLightElement" , js_name = specularExponent ) ]
    #[cfg(feature = "SvgAnimatedNumber")]
    #[doc = "Getter for the `specularExponent` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFESpotLightElement/specularExponent)\n\n*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgfeSpotLightElement`*"]
    pub fn specular_exponent(this: &SvgfeSpotLightElement) -> SvgAnimatedNumber;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFESpotLightElement" , js_name = limitingConeAngle ) ]
    #[cfg(feature = "SvgAnimatedNumber")]
    #[doc = "Getter for the `limitingConeAngle` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFESpotLightElement/limitingConeAngle)\n\n*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgfeSpotLightElement`*"]
    pub fn limiting_cone_angle(this: &SvgfeSpotLightElement) -> SvgAnimatedNumber;
}
