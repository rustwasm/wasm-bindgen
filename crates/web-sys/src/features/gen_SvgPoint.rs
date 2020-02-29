use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = SVGPoint , typescript_type = "SVGPoint" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `SvgPoint` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPoint)
    ///
    ///*This API requires the following crate features to be activated: `SvgPoint`*
    pub type SvgPoint;

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGPoint" , js_name = x ) ]
    ///Getter for the `x` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPoint/x)
    ///
    ///*This API requires the following crate features to be activated: `SvgPoint`*
    pub fn x(this: &SvgPoint) -> f32;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGPoint" , js_name = x ) ]
    ///Setter for the `x` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPoint/x)
    ///
    ///*This API requires the following crate features to be activated: `SvgPoint`*
    pub fn set_x(this: &SvgPoint, value: f32);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGPoint" , js_name = y ) ]
    ///Getter for the `y` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPoint/y)
    ///
    ///*This API requires the following crate features to be activated: `SvgPoint`*
    pub fn y(this: &SvgPoint) -> f32;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGPoint" , js_name = y ) ]
    ///Setter for the `y` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPoint/y)
    ///
    ///*This API requires the following crate features to be activated: `SvgPoint`*
    pub fn set_y(this: &SvgPoint, value: f32);

    #[cfg(feature = "SvgMatrix")]
    # [ wasm_bindgen ( method , structural , js_class = "SVGPoint" , js_name = matrixTransform ) ]
    ///The `matrixTransform()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPoint/matrixTransform)
    ///
    ///*This API requires the following crate features to be activated: `SvgMatrix`, `SvgPoint`*
    pub fn matrix_transform(this: &SvgPoint, matrix: &SvgMatrix) -> SvgPoint;

}
