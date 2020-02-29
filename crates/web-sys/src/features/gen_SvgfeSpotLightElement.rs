use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = SvgElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = SVGFESpotLightElement , typescript_name = SVGFESpotLightElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `SvgfeSpotLightElement` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFESpotLightElement)
    ///
    ///*This API requires the following crate features to be activated: `SvgfeSpotLightElement`*
    pub type SvgfeSpotLightElement;

    #[cfg(feature = "SvgAnimatedNumber")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFESpotLightElement" , js_name = x ) ]
    ///Getter for the `x` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFESpotLightElement/x)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgfeSpotLightElement`*
    pub fn x(this: &SvgfeSpotLightElement) -> SvgAnimatedNumber;

    #[cfg(feature = "SvgAnimatedNumber")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFESpotLightElement" , js_name = y ) ]
    ///Getter for the `y` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFESpotLightElement/y)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgfeSpotLightElement`*
    pub fn y(this: &SvgfeSpotLightElement) -> SvgAnimatedNumber;

    #[cfg(feature = "SvgAnimatedNumber")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFESpotLightElement" , js_name = z ) ]
    ///Getter for the `z` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFESpotLightElement/z)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgfeSpotLightElement`*
    pub fn z(this: &SvgfeSpotLightElement) -> SvgAnimatedNumber;

    #[cfg(feature = "SvgAnimatedNumber")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFESpotLightElement" , js_name = pointsAtX ) ]
    ///Getter for the `pointsAtX` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFESpotLightElement/pointsAtX)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgfeSpotLightElement`*
    pub fn points_at_x(this: &SvgfeSpotLightElement) -> SvgAnimatedNumber;

    #[cfg(feature = "SvgAnimatedNumber")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFESpotLightElement" , js_name = pointsAtY ) ]
    ///Getter for the `pointsAtY` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFESpotLightElement/pointsAtY)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgfeSpotLightElement`*
    pub fn points_at_y(this: &SvgfeSpotLightElement) -> SvgAnimatedNumber;

    #[cfg(feature = "SvgAnimatedNumber")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFESpotLightElement" , js_name = pointsAtZ ) ]
    ///Getter for the `pointsAtZ` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFESpotLightElement/pointsAtZ)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgfeSpotLightElement`*
    pub fn points_at_z(this: &SvgfeSpotLightElement) -> SvgAnimatedNumber;

    #[cfg(feature = "SvgAnimatedNumber")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFESpotLightElement" , js_name = specularExponent ) ]
    ///Getter for the `specularExponent` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFESpotLightElement/specularExponent)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgfeSpotLightElement`*
    pub fn specular_exponent(this: &SvgfeSpotLightElement) -> SvgAnimatedNumber;

    #[cfg(feature = "SvgAnimatedNumber")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFESpotLightElement" , js_name = limitingConeAngle ) ]
    ///Getter for the `limitingConeAngle` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFESpotLightElement/limitingConeAngle)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgfeSpotLightElement`*
    pub fn limiting_cone_angle(this: &SvgfeSpotLightElement) -> SvgAnimatedNumber;

}
