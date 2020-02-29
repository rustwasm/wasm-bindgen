use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = DomPointReadOnly , extends = :: js_sys :: Object , js_name = DOMPoint , typescript_type = "DOMPoint" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `DomPoint` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMPoint)
    ///
    ///*This API requires the following crate features to be activated: `DomPoint`*
    pub type DomPoint;

    # [ wasm_bindgen ( structural , method , getter , js_class = "DOMPoint" , js_name = x ) ]
    ///Getter for the `x` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMPoint/x)
    ///
    ///*This API requires the following crate features to be activated: `DomPoint`*
    pub fn x(this: &DomPoint) -> f64;

    # [ wasm_bindgen ( structural , method , setter , js_class = "DOMPoint" , js_name = x ) ]
    ///Setter for the `x` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMPoint/x)
    ///
    ///*This API requires the following crate features to be activated: `DomPoint`*
    pub fn set_x(this: &DomPoint, value: f64);

    # [ wasm_bindgen ( structural , method , getter , js_class = "DOMPoint" , js_name = y ) ]
    ///Getter for the `y` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMPoint/y)
    ///
    ///*This API requires the following crate features to be activated: `DomPoint`*
    pub fn y(this: &DomPoint) -> f64;

    # [ wasm_bindgen ( structural , method , setter , js_class = "DOMPoint" , js_name = y ) ]
    ///Setter for the `y` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMPoint/y)
    ///
    ///*This API requires the following crate features to be activated: `DomPoint`*
    pub fn set_y(this: &DomPoint, value: f64);

    # [ wasm_bindgen ( structural , method , getter , js_class = "DOMPoint" , js_name = z ) ]
    ///Getter for the `z` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMPoint/z)
    ///
    ///*This API requires the following crate features to be activated: `DomPoint`*
    pub fn z(this: &DomPoint) -> f64;

    # [ wasm_bindgen ( structural , method , setter , js_class = "DOMPoint" , js_name = z ) ]
    ///Setter for the `z` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMPoint/z)
    ///
    ///*This API requires the following crate features to be activated: `DomPoint`*
    pub fn set_z(this: &DomPoint, value: f64);

    # [ wasm_bindgen ( structural , method , getter , js_class = "DOMPoint" , js_name = w ) ]
    ///Getter for the `w` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMPoint/w)
    ///
    ///*This API requires the following crate features to be activated: `DomPoint`*
    pub fn w(this: &DomPoint) -> f64;

    # [ wasm_bindgen ( structural , method , setter , js_class = "DOMPoint" , js_name = w ) ]
    ///Setter for the `w` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMPoint/w)
    ///
    ///*This API requires the following crate features to be activated: `DomPoint`*
    pub fn set_w(this: &DomPoint, value: f64);

    #[wasm_bindgen(catch, constructor, js_class = "DOMPoint")]
    ///The `new DomPoint(..)` constructor, creating a new instance of `DomPoint`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMPoint/DOMPoint)
    ///
    ///*This API requires the following crate features to be activated: `DomPoint`*
    pub fn new() -> Result<DomPoint, JsValue>;

    #[wasm_bindgen(catch, constructor, js_class = "DOMPoint")]
    ///The `new DomPoint(..)` constructor, creating a new instance of `DomPoint`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMPoint/DOMPoint)
    ///
    ///*This API requires the following crate features to be activated: `DomPoint`*
    pub fn new_with_x(x: f64) -> Result<DomPoint, JsValue>;

    #[wasm_bindgen(catch, constructor, js_class = "DOMPoint")]
    ///The `new DomPoint(..)` constructor, creating a new instance of `DomPoint`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMPoint/DOMPoint)
    ///
    ///*This API requires the following crate features to be activated: `DomPoint`*
    pub fn new_with_x_and_y(x: f64, y: f64) -> Result<DomPoint, JsValue>;

    #[wasm_bindgen(catch, constructor, js_class = "DOMPoint")]
    ///The `new DomPoint(..)` constructor, creating a new instance of `DomPoint`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMPoint/DOMPoint)
    ///
    ///*This API requires the following crate features to be activated: `DomPoint`*
    pub fn new_with_x_and_y_and_z(x: f64, y: f64, z: f64) -> Result<DomPoint, JsValue>;

    #[wasm_bindgen(catch, constructor, js_class = "DOMPoint")]
    ///The `new DomPoint(..)` constructor, creating a new instance of `DomPoint`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMPoint/DOMPoint)
    ///
    ///*This API requires the following crate features to be activated: `DomPoint`*
    pub fn new_with_x_and_y_and_z_and_w(
        x: f64,
        y: f64,
        z: f64,
        w: f64,
    ) -> Result<DomPoint, JsValue>;

    # [ wasm_bindgen ( static_method_of = DomPoint , js_class = "DOMPoint" , js_name = fromPoint ) ]
    ///The `fromPoint()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMPoint/fromPoint)
    ///
    ///*This API requires the following crate features to be activated: `DomPoint`*
    pub fn from_point() -> DomPoint;

    #[cfg(feature = "DomPointInit")]
    # [ wasm_bindgen ( static_method_of = DomPoint , js_class = "DOMPoint" , js_name = fromPoint ) ]
    ///The `fromPoint()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMPoint/fromPoint)
    ///
    ///*This API requires the following crate features to be activated: `DomPoint`, `DomPointInit`*
    pub fn from_point_with_other(other: &DomPointInit) -> DomPoint;

}
