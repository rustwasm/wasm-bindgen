use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = SVGAnimatedBoolean , typescript_name = SVGAnimatedBoolean ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `SvgAnimatedBoolean` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedBoolean)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedBoolean`*
    pub type SvgAnimatedBoolean;

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGAnimatedBoolean" , js_name = baseVal ) ]
    ///Getter for the `baseVal` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedBoolean/baseVal)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedBoolean`*
    pub fn base_val(this: &SvgAnimatedBoolean) -> bool;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGAnimatedBoolean" , js_name = baseVal ) ]
    ///Setter for the `baseVal` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedBoolean/baseVal)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedBoolean`*
    pub fn set_base_val(this: &SvgAnimatedBoolean, value: bool);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGAnimatedBoolean" , js_name = animVal ) ]
    ///Getter for the `animVal` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedBoolean/animVal)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedBoolean`*
    pub fn anim_val(this: &SvgAnimatedBoolean) -> bool;

}
