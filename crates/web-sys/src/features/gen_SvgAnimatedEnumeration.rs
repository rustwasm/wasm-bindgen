use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = SVGAnimatedEnumeration , typescript_type = "SVGAnimatedEnumeration" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `SvgAnimatedEnumeration` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedEnumeration)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedEnumeration`*
    pub type SvgAnimatedEnumeration;

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGAnimatedEnumeration" , js_name = baseVal ) ]
    ///Getter for the `baseVal` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedEnumeration/baseVal)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedEnumeration`*
    pub fn base_val(this: &SvgAnimatedEnumeration) -> u16;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGAnimatedEnumeration" , js_name = baseVal ) ]
    ///Setter for the `baseVal` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedEnumeration/baseVal)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedEnumeration`*
    pub fn set_base_val(this: &SvgAnimatedEnumeration, value: u16);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGAnimatedEnumeration" , js_name = animVal ) ]
    ///Getter for the `animVal` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedEnumeration/animVal)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedEnumeration`*
    pub fn anim_val(this: &SvgAnimatedEnumeration) -> u16;

}
