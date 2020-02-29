use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = SVGAnimatedPreserveAspectRatio , typescript_type = "SVGAnimatedPreserveAspectRatio" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `SvgAnimatedPreserveAspectRatio` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedPreserveAspectRatio)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedPreserveAspectRatio`*
    pub type SvgAnimatedPreserveAspectRatio;

    #[cfg(feature = "SvgPreserveAspectRatio")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGAnimatedPreserveAspectRatio" , js_name = baseVal ) ]
    ///Getter for the `baseVal` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedPreserveAspectRatio/baseVal)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedPreserveAspectRatio`, `SvgPreserveAspectRatio`*
    pub fn base_val(this: &SvgAnimatedPreserveAspectRatio) -> SvgPreserveAspectRatio;

    #[cfg(feature = "SvgPreserveAspectRatio")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGAnimatedPreserveAspectRatio" , js_name = animVal ) ]
    ///Getter for the `animVal` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedPreserveAspectRatio/animVal)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedPreserveAspectRatio`, `SvgPreserveAspectRatio`*
    pub fn anim_val(this: &SvgAnimatedPreserveAspectRatio) -> SvgPreserveAspectRatio;

}
