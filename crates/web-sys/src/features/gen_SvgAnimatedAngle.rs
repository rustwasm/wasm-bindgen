use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = SVGAnimatedAngle , typescript_name = SVGAnimatedAngle ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgAnimatedAngle` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedAngle)\n\n*This API requires the following crate features to be activated: `SvgAnimatedAngle`*"]
    pub type SvgAnimatedAngle;
    # [ wasm_bindgen ( structural , method , getter , js_name = baseVal ) ]
    #[cfg(feature = "SvgAngle")]
    #[doc = "Getter for the `baseVal` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedAngle/baseVal)\n\n*This API requires the following crate features to be activated: `SvgAngle`, `SvgAnimatedAngle`*"]
    pub fn base_val(this: &SvgAnimatedAngle) -> SvgAngle;
    # [ wasm_bindgen ( structural , method , getter , js_name = animVal ) ]
    #[cfg(feature = "SvgAngle")]
    #[doc = "Getter for the `animVal` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedAngle/animVal)\n\n*This API requires the following crate features to be activated: `SvgAngle`, `SvgAnimatedAngle`*"]
    pub fn anim_val(this: &SvgAnimatedAngle) -> SvgAngle;
}
