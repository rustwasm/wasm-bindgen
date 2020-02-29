use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = SVGAnimatedRect , typescript_type = "SVGAnimatedRect" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `SvgAnimatedRect` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedRect)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedRect`*
    pub type SvgAnimatedRect;

    #[cfg(feature = "SvgRect")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGAnimatedRect" , js_name = baseVal ) ]
    ///Getter for the `baseVal` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedRect/baseVal)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedRect`, `SvgRect`*
    pub fn base_val(this: &SvgAnimatedRect) -> Option<SvgRect>;

    #[cfg(feature = "SvgRect")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGAnimatedRect" , js_name = animVal ) ]
    ///Getter for the `animVal` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedRect/animVal)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedRect`, `SvgRect`*
    pub fn anim_val(this: &SvgAnimatedRect) -> Option<SvgRect>;

}
