use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = SVGPoint , typescript_name = SVGPoint ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgPoint` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPoint)\n\n*This API requires the following crate features to be activated: `SvgPoint`*"]
    pub type SvgPoint;
    # [ wasm_bindgen ( structural , method , getter , js_name = x ) ]
    #[doc = "Getter for the `x` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPoint/x)\n\n*This API requires the following crate features to be activated: `SvgPoint`*"]
    pub fn x(this: &SvgPoint) -> f32;
    # [ wasm_bindgen ( structural , method , setter , js_name = x ) ]
    #[doc = "Setter for the `x` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPoint/x)\n\n*This API requires the following crate features to be activated: `SvgPoint`*"]
    pub fn set_x(this: &SvgPoint, value: f32);
    # [ wasm_bindgen ( structural , method , getter , js_name = y ) ]
    #[doc = "Getter for the `y` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPoint/y)\n\n*This API requires the following crate features to be activated: `SvgPoint`*"]
    pub fn y(this: &SvgPoint) -> f32;
    # [ wasm_bindgen ( structural , method , setter , js_name = y ) ]
    #[doc = "Setter for the `y` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPoint/y)\n\n*This API requires the following crate features to be activated: `SvgPoint`*"]
    pub fn set_y(this: &SvgPoint, value: f32);
    #[cfg(feature = "SvgMatrix")]
    # [ wasm_bindgen ( method , structural , js_name = matrixTransform ) ]
    #[doc = "The `matrixTransform()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPoint/matrixTransform)\n\n*This API requires the following crate features to be activated: `SvgMatrix`, `SvgPoint`*"]
    pub fn matrix_transform(this: &SvgPoint, matrix: &SvgMatrix) -> SvgPoint;
}
