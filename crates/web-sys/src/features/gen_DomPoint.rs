use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = DomPointReadOnly , extends = :: js_sys :: Object , js_name = DOMPoint , typescript_name = DOMPoint ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DomPoint` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMPoint)\n\n*This API requires the following crate features to be activated: `DomPoint`*"]
    pub type DomPoint;
    # [ wasm_bindgen ( structural , method , getter , js_name = x ) ]
    #[doc = "Getter for the `x` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMPoint/x)\n\n*This API requires the following crate features to be activated: `DomPoint`*"]
    pub fn x(this: &DomPoint) -> f64;
    # [ wasm_bindgen ( structural , method , setter , js_name = x ) ]
    #[doc = "Setter for the `x` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMPoint/x)\n\n*This API requires the following crate features to be activated: `DomPoint`*"]
    pub fn set_x(this: &DomPoint, value: f64);
    # [ wasm_bindgen ( structural , method , getter , js_name = y ) ]
    #[doc = "Getter for the `y` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMPoint/y)\n\n*This API requires the following crate features to be activated: `DomPoint`*"]
    pub fn y(this: &DomPoint) -> f64;
    # [ wasm_bindgen ( structural , method , setter , js_name = y ) ]
    #[doc = "Setter for the `y` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMPoint/y)\n\n*This API requires the following crate features to be activated: `DomPoint`*"]
    pub fn set_y(this: &DomPoint, value: f64);
    # [ wasm_bindgen ( structural , method , getter , js_name = z ) ]
    #[doc = "Getter for the `z` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMPoint/z)\n\n*This API requires the following crate features to be activated: `DomPoint`*"]
    pub fn z(this: &DomPoint) -> f64;
    # [ wasm_bindgen ( structural , method , setter , js_name = z ) ]
    #[doc = "Setter for the `z` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMPoint/z)\n\n*This API requires the following crate features to be activated: `DomPoint`*"]
    pub fn set_z(this: &DomPoint, value: f64);
    # [ wasm_bindgen ( structural , method , getter , js_name = w ) ]
    #[doc = "Getter for the `w` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMPoint/w)\n\n*This API requires the following crate features to be activated: `DomPoint`*"]
    pub fn w(this: &DomPoint) -> f64;
    # [ wasm_bindgen ( structural , method , setter , js_name = w ) ]
    #[doc = "Setter for the `w` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMPoint/w)\n\n*This API requires the following crate features to be activated: `DomPoint`*"]
    pub fn set_w(this: &DomPoint, value: f64);
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new DomPoint(..)` constructor, creating a new instance of `DomPoint`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMPoint/DOMPoint)\n\n*This API requires the following crate features to be activated: `DomPoint`*"]
    pub fn new(this: &DomPoint) -> Result<DomPoint, JsValue>;
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new DomPoint(..)` constructor, creating a new instance of `DomPoint`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMPoint/DOMPoint)\n\n*This API requires the following crate features to be activated: `DomPoint`*"]
    pub fn new_with_x(this: &DomPoint, x: f64) -> Result<DomPoint, JsValue>;
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new DomPoint(..)` constructor, creating a new instance of `DomPoint`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMPoint/DOMPoint)\n\n*This API requires the following crate features to be activated: `DomPoint`*"]
    pub fn new_with_x_and_y(this: &DomPoint, x: f64, y: f64) -> Result<DomPoint, JsValue>;
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new DomPoint(..)` constructor, creating a new instance of `DomPoint`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMPoint/DOMPoint)\n\n*This API requires the following crate features to be activated: `DomPoint`*"]
    pub fn new_with_x_and_y_and_z(
        this: &DomPoint,
        x: f64,
        y: f64,
        z: f64,
    ) -> Result<DomPoint, JsValue>;
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new DomPoint(..)` constructor, creating a new instance of `DomPoint`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMPoint/DOMPoint)\n\n*This API requires the following crate features to be activated: `DomPoint`*"]
    pub fn new_with_x_and_y_and_z_and_w(
        this: &DomPoint,
        x: f64,
        y: f64,
        z: f64,
        w: f64,
    ) -> Result<DomPoint, JsValue>;
    # [ wasm_bindgen ( static_method_of = DOMPoint , js_name = fromPoint ) ]
    #[doc = "The `fromPoint()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMPoint/fromPoint)\n\n*This API requires the following crate features to be activated: `DomPoint`*"]
    pub fn from_point() -> DomPoint;
    #[cfg(feature = "DomPointInit")]
    # [ wasm_bindgen ( static_method_of = DOMPoint , js_name = fromPoint ) ]
    #[doc = "The `fromPoint()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMPoint/fromPoint)\n\n*This API requires the following crate features to be activated: `DomPoint`, `DomPointInit`*"]
    pub fn from_point_with_other(other: &DomPointInit) -> DomPoint;
}
