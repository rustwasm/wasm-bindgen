use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = DOMPointReadOnly , typescript_name = DOMPointReadOnly ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `DomPointReadOnly` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMPointReadOnly)
    ///
    ///*This API requires the following crate features to be activated: `DomPointReadOnly`*
    pub type DomPointReadOnly;

    # [ wasm_bindgen ( structural , method , getter , js_class = "DOMPointReadOnly" , js_name = x ) ]
    ///Getter for the `x` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMPointReadOnly/x)
    ///
    ///*This API requires the following crate features to be activated: `DomPointReadOnly`*
    pub fn x(this: &DomPointReadOnly) -> f64;

    # [ wasm_bindgen ( structural , method , getter , js_class = "DOMPointReadOnly" , js_name = y ) ]
    ///Getter for the `y` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMPointReadOnly/y)
    ///
    ///*This API requires the following crate features to be activated: `DomPointReadOnly`*
    pub fn y(this: &DomPointReadOnly) -> f64;

    # [ wasm_bindgen ( structural , method , getter , js_class = "DOMPointReadOnly" , js_name = z ) ]
    ///Getter for the `z` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMPointReadOnly/z)
    ///
    ///*This API requires the following crate features to be activated: `DomPointReadOnly`*
    pub fn z(this: &DomPointReadOnly) -> f64;

    # [ wasm_bindgen ( structural , method , getter , js_class = "DOMPointReadOnly" , js_name = w ) ]
    ///Getter for the `w` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMPointReadOnly/w)
    ///
    ///*This API requires the following crate features to be activated: `DomPointReadOnly`*
    pub fn w(this: &DomPointReadOnly) -> f64;

    #[wasm_bindgen(catch, constructor, js_class = "DOMPointReadOnly")]
    ///The `new DomPointReadOnly(..)` constructor, creating a new instance of `DomPointReadOnly`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMPointReadOnly/DOMPointReadOnly)
    ///
    ///*This API requires the following crate features to be activated: `DomPointReadOnly`*
    pub fn new() -> Result<DomPointReadOnly, JsValue>;

    #[wasm_bindgen(catch, constructor, js_class = "DOMPointReadOnly")]
    ///The `new DomPointReadOnly(..)` constructor, creating a new instance of `DomPointReadOnly`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMPointReadOnly/DOMPointReadOnly)
    ///
    ///*This API requires the following crate features to be activated: `DomPointReadOnly`*
    pub fn new_with_x(x: f64) -> Result<DomPointReadOnly, JsValue>;

    #[wasm_bindgen(catch, constructor, js_class = "DOMPointReadOnly")]
    ///The `new DomPointReadOnly(..)` constructor, creating a new instance of `DomPointReadOnly`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMPointReadOnly/DOMPointReadOnly)
    ///
    ///*This API requires the following crate features to be activated: `DomPointReadOnly`*
    pub fn new_with_x_and_y(x: f64, y: f64) -> Result<DomPointReadOnly, JsValue>;

    #[wasm_bindgen(catch, constructor, js_class = "DOMPointReadOnly")]
    ///The `new DomPointReadOnly(..)` constructor, creating a new instance of `DomPointReadOnly`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMPointReadOnly/DOMPointReadOnly)
    ///
    ///*This API requires the following crate features to be activated: `DomPointReadOnly`*
    pub fn new_with_x_and_y_and_z(x: f64, y: f64, z: f64) -> Result<DomPointReadOnly, JsValue>;

    #[wasm_bindgen(catch, constructor, js_class = "DOMPointReadOnly")]
    ///The `new DomPointReadOnly(..)` constructor, creating a new instance of `DomPointReadOnly`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMPointReadOnly/DOMPointReadOnly)
    ///
    ///*This API requires the following crate features to be activated: `DomPointReadOnly`*
    pub fn new_with_x_and_y_and_z_and_w(
        x: f64,
        y: f64,
        z: f64,
        w: f64,
    ) -> Result<DomPointReadOnly, JsValue>;

    # [ wasm_bindgen ( static_method_of = DomPointReadOnly , js_class = "DOMPointReadOnly" , js_name = fromPoint ) ]
    ///The `fromPoint()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMPointReadOnly/fromPoint)
    ///
    ///*This API requires the following crate features to be activated: `DomPointReadOnly`*
    pub fn from_point() -> DomPointReadOnly;

    #[cfg(feature = "DomPointInit")]
    # [ wasm_bindgen ( static_method_of = DomPointReadOnly , js_class = "DOMPointReadOnly" , js_name = fromPoint ) ]
    ///The `fromPoint()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMPointReadOnly/fromPoint)
    ///
    ///*This API requires the following crate features to be activated: `DomPointInit`, `DomPointReadOnly`*
    pub fn from_point_with_other(other: &DomPointInit) -> DomPointReadOnly;

    # [ wasm_bindgen ( method , structural , js_class = "DOMPointReadOnly" , js_name = toJSON ) ]
    ///The `toJSON()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMPointReadOnly/toJSON)
    ///
    ///*This API requires the following crate features to be activated: `DomPointReadOnly`*
    pub fn to_json(this: &DomPointReadOnly) -> ::js_sys::Object;

}
