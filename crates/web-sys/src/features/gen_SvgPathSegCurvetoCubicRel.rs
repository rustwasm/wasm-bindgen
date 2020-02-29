use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( is_type_of = | _ | false , extends = SvgPathSeg , extends = :: js_sys :: Object , js_name = SVGPathSegCurvetoCubicRel , typescript_name = SVGPathSegCurvetoCubicRel ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `SvgPathSegCurvetoCubicRel` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegCurvetoCubicRel)
    ///
    ///*This API requires the following crate features to be activated: `SvgPathSegCurvetoCubicRel`*
    pub type SvgPathSegCurvetoCubicRel;

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGPathSegCurvetoCubicRel" , js_name = x ) ]
    ///Getter for the `x` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegCurvetoCubicRel/x)
    ///
    ///*This API requires the following crate features to be activated: `SvgPathSegCurvetoCubicRel`*
    pub fn x(this: &SvgPathSegCurvetoCubicRel) -> f32;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGPathSegCurvetoCubicRel" , js_name = x ) ]
    ///Setter for the `x` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegCurvetoCubicRel/x)
    ///
    ///*This API requires the following crate features to be activated: `SvgPathSegCurvetoCubicRel`*
    pub fn set_x(this: &SvgPathSegCurvetoCubicRel, value: f32);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGPathSegCurvetoCubicRel" , js_name = y ) ]
    ///Getter for the `y` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegCurvetoCubicRel/y)
    ///
    ///*This API requires the following crate features to be activated: `SvgPathSegCurvetoCubicRel`*
    pub fn y(this: &SvgPathSegCurvetoCubicRel) -> f32;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGPathSegCurvetoCubicRel" , js_name = y ) ]
    ///Setter for the `y` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegCurvetoCubicRel/y)
    ///
    ///*This API requires the following crate features to be activated: `SvgPathSegCurvetoCubicRel`*
    pub fn set_y(this: &SvgPathSegCurvetoCubicRel, value: f32);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGPathSegCurvetoCubicRel" , js_name = x1 ) ]
    ///Getter for the `x1` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegCurvetoCubicRel/x1)
    ///
    ///*This API requires the following crate features to be activated: `SvgPathSegCurvetoCubicRel`*
    pub fn x1(this: &SvgPathSegCurvetoCubicRel) -> f32;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGPathSegCurvetoCubicRel" , js_name = x1 ) ]
    ///Setter for the `x1` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegCurvetoCubicRel/x1)
    ///
    ///*This API requires the following crate features to be activated: `SvgPathSegCurvetoCubicRel`*
    pub fn set_x1(this: &SvgPathSegCurvetoCubicRel, value: f32);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGPathSegCurvetoCubicRel" , js_name = y1 ) ]
    ///Getter for the `y1` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegCurvetoCubicRel/y1)
    ///
    ///*This API requires the following crate features to be activated: `SvgPathSegCurvetoCubicRel`*
    pub fn y1(this: &SvgPathSegCurvetoCubicRel) -> f32;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGPathSegCurvetoCubicRel" , js_name = y1 ) ]
    ///Setter for the `y1` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegCurvetoCubicRel/y1)
    ///
    ///*This API requires the following crate features to be activated: `SvgPathSegCurvetoCubicRel`*
    pub fn set_y1(this: &SvgPathSegCurvetoCubicRel, value: f32);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGPathSegCurvetoCubicRel" , js_name = x2 ) ]
    ///Getter for the `x2` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegCurvetoCubicRel/x2)
    ///
    ///*This API requires the following crate features to be activated: `SvgPathSegCurvetoCubicRel`*
    pub fn x2(this: &SvgPathSegCurvetoCubicRel) -> f32;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGPathSegCurvetoCubicRel" , js_name = x2 ) ]
    ///Setter for the `x2` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegCurvetoCubicRel/x2)
    ///
    ///*This API requires the following crate features to be activated: `SvgPathSegCurvetoCubicRel`*
    pub fn set_x2(this: &SvgPathSegCurvetoCubicRel, value: f32);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGPathSegCurvetoCubicRel" , js_name = y2 ) ]
    ///Getter for the `y2` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegCurvetoCubicRel/y2)
    ///
    ///*This API requires the following crate features to be activated: `SvgPathSegCurvetoCubicRel`*
    pub fn y2(this: &SvgPathSegCurvetoCubicRel) -> f32;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGPathSegCurvetoCubicRel" , js_name = y2 ) ]
    ///Setter for the `y2` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegCurvetoCubicRel/y2)
    ///
    ///*This API requires the following crate features to be activated: `SvgPathSegCurvetoCubicRel`*
    pub fn set_y2(this: &SvgPathSegCurvetoCubicRel, value: f32);

}
