use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = SVGAnimatedTransformList , typescript_name = SVGAnimatedTransformList ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `SvgAnimatedTransformList` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedTransformList)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedTransformList`*
    pub type SvgAnimatedTransformList;

    #[cfg(feature = "SvgTransformList")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGAnimatedTransformList" , js_name = baseVal ) ]
    ///Getter for the `baseVal` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedTransformList/baseVal)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedTransformList`, `SvgTransformList`*
    pub fn base_val(this: &SvgAnimatedTransformList) -> SvgTransformList;

    #[cfg(feature = "SvgTransformList")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGAnimatedTransformList" , js_name = animVal ) ]
    ///Getter for the `animVal` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedTransformList/animVal)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedTransformList`, `SvgTransformList`*
    pub fn anim_val(this: &SvgAnimatedTransformList) -> SvgTransformList;

}
