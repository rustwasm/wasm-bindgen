use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = SVGAnimatedPreserveAspectRatio , typescript_name = SVGAnimatedPreserveAspectRatio ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgAnimatedPreserveAspectRatio` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedPreserveAspectRatio)\n\n*This API requires the following crate features to be activated: `SvgAnimatedPreserveAspectRatio`*"]
    pub type SvgAnimatedPreserveAspectRatio;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGAnimatedPreserveAspectRatio" , js_name = baseVal ) ]
    #[cfg(feature = "SvgPreserveAspectRatio")]
    #[doc = "Getter for the `baseVal` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedPreserveAspectRatio/baseVal)\n\n*This API requires the following crate features to be activated: `SvgAnimatedPreserveAspectRatio`, `SvgPreserveAspectRatio`*"]
    pub fn base_val(this: &SvgAnimatedPreserveAspectRatio) -> SvgPreserveAspectRatio;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGAnimatedPreserveAspectRatio" , js_name = animVal ) ]
    #[cfg(feature = "SvgPreserveAspectRatio")]
    #[doc = "Getter for the `animVal` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedPreserveAspectRatio/animVal)\n\n*This API requires the following crate features to be activated: `SvgAnimatedPreserveAspectRatio`, `SvgPreserveAspectRatio`*"]
    pub fn anim_val(this: &SvgAnimatedPreserveAspectRatio) -> SvgPreserveAspectRatio;
}
