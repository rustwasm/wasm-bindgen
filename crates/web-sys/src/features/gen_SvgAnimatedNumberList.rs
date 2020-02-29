use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = SVGAnimatedNumberList , typescript_name = SVGAnimatedNumberList ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `SvgAnimatedNumberList` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedNumberList)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedNumberList`*
    pub type SvgAnimatedNumberList;

    #[cfg(feature = "SvgNumberList")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGAnimatedNumberList" , js_name = baseVal ) ]
    ///Getter for the `baseVal` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedNumberList/baseVal)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedNumberList`, `SvgNumberList`*
    pub fn base_val(this: &SvgAnimatedNumberList) -> SvgNumberList;

    #[cfg(feature = "SvgNumberList")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGAnimatedNumberList" , js_name = animVal ) ]
    ///Getter for the `animVal` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedNumberList/animVal)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedNumberList`, `SvgNumberList`*
    pub fn anim_val(this: &SvgAnimatedNumberList) -> SvgNumberList;

}
