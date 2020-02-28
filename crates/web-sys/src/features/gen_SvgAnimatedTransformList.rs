use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = SVGAnimatedTransformList , typescript_name = SVGAnimatedTransformList ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgAnimatedTransformList` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedTransformList)\n\n*This API requires the following crate features to be activated: `SvgAnimatedTransformList`*"]
    pub type SvgAnimatedTransformList;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGAnimatedTransformList" , js_name = baseVal ) ]
    #[cfg(feature = "SvgTransformList")]
    #[doc = "Getter for the `baseVal` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedTransformList/baseVal)\n\n*This API requires the following crate features to be activated: `SvgAnimatedTransformList`, `SvgTransformList`*"]
    pub fn base_val(this: &SvgAnimatedTransformList) -> SvgTransformList;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGAnimatedTransformList" , js_name = animVal ) ]
    #[cfg(feature = "SvgTransformList")]
    #[doc = "Getter for the `animVal` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedTransformList/animVal)\n\n*This API requires the following crate features to be activated: `SvgAnimatedTransformList`, `SvgTransformList`*"]
    pub fn anim_val(this: &SvgAnimatedTransformList) -> SvgTransformList;
}
