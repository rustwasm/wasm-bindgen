use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = SVGAnimatedLengthList , typescript_type = "SVGAnimatedLengthList" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `SvgAnimatedLengthList` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedLengthList)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedLengthList`*
    pub type SvgAnimatedLengthList;

    #[cfg(feature = "SvgLengthList")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGAnimatedLengthList" , js_name = baseVal ) ]
    ///Getter for the `baseVal` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedLengthList/baseVal)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedLengthList`, `SvgLengthList`*
    pub fn base_val(this: &SvgAnimatedLengthList) -> SvgLengthList;

    #[cfg(feature = "SvgLengthList")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGAnimatedLengthList" , js_name = animVal ) ]
    ///Getter for the `animVal` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedLengthList/animVal)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedLengthList`, `SvgLengthList`*
    pub fn anim_val(this: &SvgAnimatedLengthList) -> SvgLengthList;

}
