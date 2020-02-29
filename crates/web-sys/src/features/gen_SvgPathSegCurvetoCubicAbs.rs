use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( is_type_of = | _ | false , extends = SvgPathSeg , extends = :: js_sys :: Object , js_name = SVGPathSegCurvetoCubicAbs , typescript_name = SVGPathSegCurvetoCubicAbs ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `SvgPathSegCurvetoCubicAbs` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegCurvetoCubicAbs)
    ///
    ///*This API requires the following crate features to be activated: `SvgPathSegCurvetoCubicAbs`*
    pub type SvgPathSegCurvetoCubicAbs;

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGPathSegCurvetoCubicAbs" , js_name = x ) ]
    ///Getter for the `x` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegCurvetoCubicAbs/x)
    ///
    ///*This API requires the following crate features to be activated: `SvgPathSegCurvetoCubicAbs`*
    pub fn x(this: &SvgPathSegCurvetoCubicAbs) -> f32;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGPathSegCurvetoCubicAbs" , js_name = x ) ]
    ///Setter for the `x` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegCurvetoCubicAbs/x)
    ///
    ///*This API requires the following crate features to be activated: `SvgPathSegCurvetoCubicAbs`*
    pub fn set_x(this: &SvgPathSegCurvetoCubicAbs, value: f32);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGPathSegCurvetoCubicAbs" , js_name = y ) ]
    ///Getter for the `y` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegCurvetoCubicAbs/y)
    ///
    ///*This API requires the following crate features to be activated: `SvgPathSegCurvetoCubicAbs`*
    pub fn y(this: &SvgPathSegCurvetoCubicAbs) -> f32;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGPathSegCurvetoCubicAbs" , js_name = y ) ]
    ///Setter for the `y` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegCurvetoCubicAbs/y)
    ///
    ///*This API requires the following crate features to be activated: `SvgPathSegCurvetoCubicAbs`*
    pub fn set_y(this: &SvgPathSegCurvetoCubicAbs, value: f32);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGPathSegCurvetoCubicAbs" , js_name = x1 ) ]
    ///Getter for the `x1` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegCurvetoCubicAbs/x1)
    ///
    ///*This API requires the following crate features to be activated: `SvgPathSegCurvetoCubicAbs`*
    pub fn x1(this: &SvgPathSegCurvetoCubicAbs) -> f32;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGPathSegCurvetoCubicAbs" , js_name = x1 ) ]
    ///Setter for the `x1` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegCurvetoCubicAbs/x1)
    ///
    ///*This API requires the following crate features to be activated: `SvgPathSegCurvetoCubicAbs`*
    pub fn set_x1(this: &SvgPathSegCurvetoCubicAbs, value: f32);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGPathSegCurvetoCubicAbs" , js_name = y1 ) ]
    ///Getter for the `y1` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegCurvetoCubicAbs/y1)
    ///
    ///*This API requires the following crate features to be activated: `SvgPathSegCurvetoCubicAbs`*
    pub fn y1(this: &SvgPathSegCurvetoCubicAbs) -> f32;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGPathSegCurvetoCubicAbs" , js_name = y1 ) ]
    ///Setter for the `y1` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegCurvetoCubicAbs/y1)
    ///
    ///*This API requires the following crate features to be activated: `SvgPathSegCurvetoCubicAbs`*
    pub fn set_y1(this: &SvgPathSegCurvetoCubicAbs, value: f32);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGPathSegCurvetoCubicAbs" , js_name = x2 ) ]
    ///Getter for the `x2` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegCurvetoCubicAbs/x2)
    ///
    ///*This API requires the following crate features to be activated: `SvgPathSegCurvetoCubicAbs`*
    pub fn x2(this: &SvgPathSegCurvetoCubicAbs) -> f32;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGPathSegCurvetoCubicAbs" , js_name = x2 ) ]
    ///Setter for the `x2` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegCurvetoCubicAbs/x2)
    ///
    ///*This API requires the following crate features to be activated: `SvgPathSegCurvetoCubicAbs`*
    pub fn set_x2(this: &SvgPathSegCurvetoCubicAbs, value: f32);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGPathSegCurvetoCubicAbs" , js_name = y2 ) ]
    ///Getter for the `y2` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegCurvetoCubicAbs/y2)
    ///
    ///*This API requires the following crate features to be activated: `SvgPathSegCurvetoCubicAbs`*
    pub fn y2(this: &SvgPathSegCurvetoCubicAbs) -> f32;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGPathSegCurvetoCubicAbs" , js_name = y2 ) ]
    ///Setter for the `y2` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegCurvetoCubicAbs/y2)
    ///
    ///*This API requires the following crate features to be activated: `SvgPathSegCurvetoCubicAbs`*
    pub fn set_y2(this: &SvgPathSegCurvetoCubicAbs, value: f32);

}
