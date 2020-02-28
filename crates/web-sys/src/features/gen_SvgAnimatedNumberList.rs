use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = SVGAnimatedNumberList , typescript_name = SVGAnimatedNumberList ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgAnimatedNumberList` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedNumberList)\n\n*This API requires the following crate features to be activated: `SvgAnimatedNumberList`*"]
    pub type SvgAnimatedNumberList;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGAnimatedNumberList" , js_name = baseVal ) ]
    #[cfg(feature = "SvgNumberList")]
    #[doc = "Getter for the `baseVal` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedNumberList/baseVal)\n\n*This API requires the following crate features to be activated: `SvgAnimatedNumberList`, `SvgNumberList`*"]
    pub fn base_val(this: &SvgAnimatedNumberList) -> SvgNumberList;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGAnimatedNumberList" , js_name = animVal ) ]
    #[cfg(feature = "SvgNumberList")]
    #[doc = "Getter for the `animVal` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedNumberList/animVal)\n\n*This API requires the following crate features to be activated: `SvgAnimatedNumberList`, `SvgNumberList`*"]
    pub fn anim_val(this: &SvgAnimatedNumberList) -> SvgNumberList;
}
