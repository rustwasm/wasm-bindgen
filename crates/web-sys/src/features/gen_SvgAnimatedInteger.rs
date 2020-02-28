use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = SVGAnimatedInteger , typescript_name = SVGAnimatedInteger ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgAnimatedInteger` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedInteger)\n\n*This API requires the following crate features to be activated: `SvgAnimatedInteger`*"]
    pub type SvgAnimatedInteger;
    # [ wasm_bindgen ( structural , method , getter , js_name = baseVal ) ]
    #[doc = "Getter for the `baseVal` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedInteger/baseVal)\n\n*This API requires the following crate features to be activated: `SvgAnimatedInteger`*"]
    pub fn base_val(this: &SvgAnimatedInteger) -> i32;
    # [ wasm_bindgen ( structural , method , setter , js_name = baseVal ) ]
    #[doc = "Setter for the `baseVal` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedInteger/baseVal)\n\n*This API requires the following crate features to be activated: `SvgAnimatedInteger`*"]
    pub fn set_base_val(this: &SvgAnimatedInteger, value: i32);
    # [ wasm_bindgen ( structural , method , getter , js_name = animVal ) ]
    #[doc = "Getter for the `animVal` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedInteger/animVal)\n\n*This API requires the following crate features to be activated: `SvgAnimatedInteger`*"]
    pub fn anim_val(this: &SvgAnimatedInteger) -> i32;
}
