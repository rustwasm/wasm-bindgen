use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( is_type_of = | _ | false , extends = SvgPathSeg , extends = :: js_sys :: Object , js_name = SVGPathSegLinetoAbs , typescript_name = SVGPathSegLinetoAbs ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `SvgPathSegLinetoAbs` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegLinetoAbs)
    ///
    ///*This API requires the following crate features to be activated: `SvgPathSegLinetoAbs`*
    pub type SvgPathSegLinetoAbs;

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGPathSegLinetoAbs" , js_name = x ) ]
    ///Getter for the `x` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegLinetoAbs/x)
    ///
    ///*This API requires the following crate features to be activated: `SvgPathSegLinetoAbs`*
    pub fn x(this: &SvgPathSegLinetoAbs) -> f32;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGPathSegLinetoAbs" , js_name = x ) ]
    ///Setter for the `x` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegLinetoAbs/x)
    ///
    ///*This API requires the following crate features to be activated: `SvgPathSegLinetoAbs`*
    pub fn set_x(this: &SvgPathSegLinetoAbs, value: f32);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGPathSegLinetoAbs" , js_name = y ) ]
    ///Getter for the `y` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegLinetoAbs/y)
    ///
    ///*This API requires the following crate features to be activated: `SvgPathSegLinetoAbs`*
    pub fn y(this: &SvgPathSegLinetoAbs) -> f32;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGPathSegLinetoAbs" , js_name = y ) ]
    ///Setter for the `y` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegLinetoAbs/y)
    ///
    ///*This API requires the following crate features to be activated: `SvgPathSegLinetoAbs`*
    pub fn set_y(this: &SvgPathSegLinetoAbs, value: f32);

}
