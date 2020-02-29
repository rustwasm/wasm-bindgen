use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = SVGAnimatedString , typescript_type = "SVGAnimatedString" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `SvgAnimatedString` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedString)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedString`*
    pub type SvgAnimatedString;

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGAnimatedString" , js_name = baseVal ) ]
    ///Getter for the `baseVal` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedString/baseVal)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedString`*
    pub fn base_val(this: &SvgAnimatedString) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGAnimatedString" , js_name = baseVal ) ]
    ///Setter for the `baseVal` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedString/baseVal)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedString`*
    pub fn set_base_val(this: &SvgAnimatedString, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGAnimatedString" , js_name = animVal ) ]
    ///Getter for the `animVal` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedString/animVal)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedString`*
    pub fn anim_val(this: &SvgAnimatedString) -> String;

}
