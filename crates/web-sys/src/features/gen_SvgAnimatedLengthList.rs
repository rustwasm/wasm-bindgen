use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = SVGAnimatedLengthList , typescript_name = SVGAnimatedLengthList ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgAnimatedLengthList` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedLengthList)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLengthList`*"]
    pub type SvgAnimatedLengthList;
    # [ wasm_bindgen ( structural , method , getter , js_name = baseVal ) ]
    #[cfg(feature = "SvgLengthList")]
    #[doc = "Getter for the `baseVal` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedLengthList/baseVal)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLengthList`, `SvgLengthList`*"]
    pub fn base_val(this: &SvgAnimatedLengthList) -> SvgLengthList;
    # [ wasm_bindgen ( structural , method , getter , js_name = animVal ) ]
    #[cfg(feature = "SvgLengthList")]
    #[doc = "Getter for the `animVal` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedLengthList/animVal)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLengthList`, `SvgLengthList`*"]
    pub fn anim_val(this: &SvgAnimatedLengthList) -> SvgLengthList;
}
