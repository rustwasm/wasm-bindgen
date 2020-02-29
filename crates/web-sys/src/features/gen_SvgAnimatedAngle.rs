use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = SVGAnimatedAngle , typescript_type = "SVGAnimatedAngle" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `SvgAnimatedAngle` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedAngle)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedAngle`*
    pub type SvgAnimatedAngle;

    #[cfg(feature = "SvgAngle")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGAnimatedAngle" , js_name = baseVal ) ]
    ///Getter for the `baseVal` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedAngle/baseVal)
    ///
    ///*This API requires the following crate features to be activated: `SvgAngle`, `SvgAnimatedAngle`*
    pub fn base_val(this: &SvgAnimatedAngle) -> SvgAngle;

    #[cfg(feature = "SvgAngle")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGAnimatedAngle" , js_name = animVal ) ]
    ///Getter for the `animVal` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedAngle/animVal)
    ///
    ///*This API requires the following crate features to be activated: `SvgAngle`, `SvgAnimatedAngle`*
    pub fn anim_val(this: &SvgAnimatedAngle) -> SvgAngle;

}
