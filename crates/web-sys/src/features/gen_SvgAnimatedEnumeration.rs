use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = SVGAnimatedEnumeration , typescript_name = SVGAnimatedEnumeration ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgAnimatedEnumeration` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedEnumeration)\n\n*This API requires the following crate features to be activated: `SvgAnimatedEnumeration`*"]
    pub type SvgAnimatedEnumeration;
    # [ wasm_bindgen ( structural , method , getter , js_name = baseVal ) ]
    #[doc = "Getter for the `baseVal` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedEnumeration/baseVal)\n\n*This API requires the following crate features to be activated: `SvgAnimatedEnumeration`*"]
    pub fn base_val(this: &SvgAnimatedEnumeration) -> u16;
    # [ wasm_bindgen ( structural , method , setter , js_name = baseVal ) ]
    #[doc = "Setter for the `baseVal` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedEnumeration/baseVal)\n\n*This API requires the following crate features to be activated: `SvgAnimatedEnumeration`*"]
    pub fn set_base_val(this: &SvgAnimatedEnumeration, value: u16);
    # [ wasm_bindgen ( structural , method , getter , js_name = animVal ) ]
    #[doc = "Getter for the `animVal` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedEnumeration/animVal)\n\n*This API requires the following crate features to be activated: `SvgAnimatedEnumeration`*"]
    pub fn anim_val(this: &SvgAnimatedEnumeration) -> u16;
}
