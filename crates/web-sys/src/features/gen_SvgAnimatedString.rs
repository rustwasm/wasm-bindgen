use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = SVGAnimatedString , typescript_name = SVGAnimatedString ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgAnimatedString` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedString)\n\n*This API requires the following crate features to be activated: `SvgAnimatedString`*"]
    pub type SvgAnimatedString;
    # [ wasm_bindgen ( structural , method , getter , js_name = baseVal ) ]
    #[doc = "Getter for the `baseVal` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedString/baseVal)\n\n*This API requires the following crate features to be activated: `SvgAnimatedString`*"]
    pub fn base_val(this: &SvgAnimatedString) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = baseVal ) ]
    #[doc = "Setter for the `baseVal` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedString/baseVal)\n\n*This API requires the following crate features to be activated: `SvgAnimatedString`*"]
    pub fn set_base_val(this: &SvgAnimatedString, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_name = animVal ) ]
    #[doc = "Getter for the `animVal` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedString/animVal)\n\n*This API requires the following crate features to be activated: `SvgAnimatedString`*"]
    pub fn anim_val(this: &SvgAnimatedString) -> String;
}
