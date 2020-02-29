use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( is_type_of = | _ | false , extends = SvgPathSeg , extends = :: js_sys :: Object , js_name = SVGPathSegMovetoRel , typescript_name = SVGPathSegMovetoRel ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `SvgPathSegMovetoRel` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegMovetoRel)
    ///
    ///*This API requires the following crate features to be activated: `SvgPathSegMovetoRel`*
    pub type SvgPathSegMovetoRel;

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGPathSegMovetoRel" , js_name = x ) ]
    ///Getter for the `x` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegMovetoRel/x)
    ///
    ///*This API requires the following crate features to be activated: `SvgPathSegMovetoRel`*
    pub fn x(this: &SvgPathSegMovetoRel) -> f32;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGPathSegMovetoRel" , js_name = x ) ]
    ///Setter for the `x` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegMovetoRel/x)
    ///
    ///*This API requires the following crate features to be activated: `SvgPathSegMovetoRel`*
    pub fn set_x(this: &SvgPathSegMovetoRel, value: f32);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGPathSegMovetoRel" , js_name = y ) ]
    ///Getter for the `y` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegMovetoRel/y)
    ///
    ///*This API requires the following crate features to be activated: `SvgPathSegMovetoRel`*
    pub fn y(this: &SvgPathSegMovetoRel) -> f32;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGPathSegMovetoRel" , js_name = y ) ]
    ///Setter for the `y` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegMovetoRel/y)
    ///
    ///*This API requires the following crate features to be activated: `SvgPathSegMovetoRel`*
    pub fn set_y(this: &SvgPathSegMovetoRel, value: f32);

}
