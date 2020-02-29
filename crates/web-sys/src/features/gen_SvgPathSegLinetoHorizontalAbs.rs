use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( is_type_of = | _ | false , extends = SvgPathSeg , extends = :: js_sys :: Object , js_name = SVGPathSegLinetoHorizontalAbs , typescript_name = SVGPathSegLinetoHorizontalAbs ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `SvgPathSegLinetoHorizontalAbs` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegLinetoHorizontalAbs)
    ///
    ///*This API requires the following crate features to be activated: `SvgPathSegLinetoHorizontalAbs`*
    pub type SvgPathSegLinetoHorizontalAbs;

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGPathSegLinetoHorizontalAbs" , js_name = x ) ]
    ///Getter for the `x` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegLinetoHorizontalAbs/x)
    ///
    ///*This API requires the following crate features to be activated: `SvgPathSegLinetoHorizontalAbs`*
    pub fn x(this: &SvgPathSegLinetoHorizontalAbs) -> f32;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGPathSegLinetoHorizontalAbs" , js_name = x ) ]
    ///Setter for the `x` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegLinetoHorizontalAbs/x)
    ///
    ///*This API requires the following crate features to be activated: `SvgPathSegLinetoHorizontalAbs`*
    pub fn set_x(this: &SvgPathSegLinetoHorizontalAbs, value: f32);

}
