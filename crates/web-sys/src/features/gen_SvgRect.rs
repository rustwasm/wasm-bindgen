use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = SVGRect , typescript_name = SVGRect ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `SvgRect` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGRect)
    ///
    ///*This API requires the following crate features to be activated: `SvgRect`*
    pub type SvgRect;

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGRect" , js_name = x ) ]
    ///Getter for the `x` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGRect/x)
    ///
    ///*This API requires the following crate features to be activated: `SvgRect`*
    pub fn x(this: &SvgRect) -> f32;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGRect" , js_name = x ) ]
    ///Setter for the `x` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGRect/x)
    ///
    ///*This API requires the following crate features to be activated: `SvgRect`*
    pub fn set_x(this: &SvgRect, value: f32);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGRect" , js_name = y ) ]
    ///Getter for the `y` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGRect/y)
    ///
    ///*This API requires the following crate features to be activated: `SvgRect`*
    pub fn y(this: &SvgRect) -> f32;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGRect" , js_name = y ) ]
    ///Setter for the `y` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGRect/y)
    ///
    ///*This API requires the following crate features to be activated: `SvgRect`*
    pub fn set_y(this: &SvgRect, value: f32);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGRect" , js_name = width ) ]
    ///Getter for the `width` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGRect/width)
    ///
    ///*This API requires the following crate features to be activated: `SvgRect`*
    pub fn width(this: &SvgRect) -> f32;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGRect" , js_name = width ) ]
    ///Setter for the `width` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGRect/width)
    ///
    ///*This API requires the following crate features to be activated: `SvgRect`*
    pub fn set_width(this: &SvgRect, value: f32);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGRect" , js_name = height ) ]
    ///Getter for the `height` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGRect/height)
    ///
    ///*This API requires the following crate features to be activated: `SvgRect`*
    pub fn height(this: &SvgRect) -> f32;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGRect" , js_name = height ) ]
    ///Setter for the `height` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGRect/height)
    ///
    ///*This API requires the following crate features to be activated: `SvgRect`*
    pub fn set_height(this: &SvgRect, value: f32);

}
