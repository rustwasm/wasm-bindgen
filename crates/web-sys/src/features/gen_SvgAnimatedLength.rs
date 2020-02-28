use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = SVGAnimatedLength , typescript_name = SVGAnimatedLength ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgAnimatedLength` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedLength)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`*"]
    pub type SvgAnimatedLength;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGAnimatedLength" , js_name = baseVal ) ]
    #[cfg(feature = "SvgLength")]
    #[doc = "Getter for the `baseVal` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedLength/baseVal)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgLength`*"]
    pub fn base_val(this: &SvgAnimatedLength) -> SvgLength;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGAnimatedLength" , js_name = animVal ) ]
    #[cfg(feature = "SvgLength")]
    #[doc = "Getter for the `animVal` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedLength/animVal)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgLength`*"]
    pub fn anim_val(this: &SvgAnimatedLength) -> SvgLength;
}
