use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( is_type_of = | _ | false , extends = SvgPathSeg , extends = :: js_sys :: Object , js_name = SVGPathSegLinetoVerticalRel , typescript_name = SVGPathSegLinetoVerticalRel ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `SvgPathSegLinetoVerticalRel` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegLinetoVerticalRel)
    ///
    ///*This API requires the following crate features to be activated: `SvgPathSegLinetoVerticalRel`*
    pub type SvgPathSegLinetoVerticalRel;

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGPathSegLinetoVerticalRel" , js_name = y ) ]
    ///Getter for the `y` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegLinetoVerticalRel/y)
    ///
    ///*This API requires the following crate features to be activated: `SvgPathSegLinetoVerticalRel`*
    pub fn y(this: &SvgPathSegLinetoVerticalRel) -> f32;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGPathSegLinetoVerticalRel" , js_name = y ) ]
    ///Setter for the `y` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegLinetoVerticalRel/y)
    ///
    ///*This API requires the following crate features to be activated: `SvgPathSegLinetoVerticalRel`*
    pub fn set_y(this: &SvgPathSegLinetoVerticalRel, value: f32);

}
