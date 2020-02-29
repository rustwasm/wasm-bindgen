use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( is_type_of = | _ | false , extends = SvgPathSeg , extends = :: js_sys :: Object , js_name = SVGPathSegLinetoVerticalAbs , typescript_type = "SVGPathSegLinetoVerticalAbs" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `SvgPathSegLinetoVerticalAbs` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegLinetoVerticalAbs)
    ///
    ///*This API requires the following crate features to be activated: `SvgPathSegLinetoVerticalAbs`*
    pub type SvgPathSegLinetoVerticalAbs;

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGPathSegLinetoVerticalAbs" , js_name = y ) ]
    ///Getter for the `y` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegLinetoVerticalAbs/y)
    ///
    ///*This API requires the following crate features to be activated: `SvgPathSegLinetoVerticalAbs`*
    pub fn y(this: &SvgPathSegLinetoVerticalAbs) -> f32;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGPathSegLinetoVerticalAbs" , js_name = y ) ]
    ///Setter for the `y` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegLinetoVerticalAbs/y)
    ///
    ///*This API requires the following crate features to be activated: `SvgPathSegLinetoVerticalAbs`*
    pub fn set_y(this: &SvgPathSegLinetoVerticalAbs, value: f32);

}
