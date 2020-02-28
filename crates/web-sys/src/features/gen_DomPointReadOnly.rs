use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = DOMPointReadOnly , typescript_name = DOMPointReadOnly ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DomPointReadOnly` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMPointReadOnly)\n\n*This API requires the following crate features to be activated: `DomPointReadOnly`*"]
    pub type DomPointReadOnly;
    # [ wasm_bindgen ( structural , method , getter , js_class = "DOMPointReadOnly" , js_name = x ) ]
    #[doc = "Getter for the `x` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMPointReadOnly/x)\n\n*This API requires the following crate features to be activated: `DomPointReadOnly`*"]
    pub fn x(this: &DomPointReadOnly) -> f64;
    # [ wasm_bindgen ( structural , method , getter , js_class = "DOMPointReadOnly" , js_name = y ) ]
    #[doc = "Getter for the `y` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMPointReadOnly/y)\n\n*This API requires the following crate features to be activated: `DomPointReadOnly`*"]
    pub fn y(this: &DomPointReadOnly) -> f64;
    # [ wasm_bindgen ( structural , method , getter , js_class = "DOMPointReadOnly" , js_name = z ) ]
    #[doc = "Getter for the `z` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMPointReadOnly/z)\n\n*This API requires the following crate features to be activated: `DomPointReadOnly`*"]
    pub fn z(this: &DomPointReadOnly) -> f64;
    # [ wasm_bindgen ( structural , method , getter , js_class = "DOMPointReadOnly" , js_name = w ) ]
    #[doc = "Getter for the `w` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMPointReadOnly/w)\n\n*This API requires the following crate features to be activated: `DomPointReadOnly`*"]
    pub fn w(this: &DomPointReadOnly) -> f64;
    #[wasm_bindgen(catch, js_class = "DOMPointReadOnly", constructor)]
    #[doc = "The `new DomPointReadOnly(..)` constructor, creating a new instance of `DomPointReadOnly`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMPointReadOnly/DOMPointReadOnly)\n\n*This API requires the following crate features to be activated: `DomPointReadOnly`*"]
    pub fn new(this: &DomPointReadOnly) -> Result<DomPointReadOnly, JsValue>;
    #[wasm_bindgen(catch, js_class = "DOMPointReadOnly", constructor)]
    #[doc = "The `new DomPointReadOnly(..)` constructor, creating a new instance of `DomPointReadOnly`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMPointReadOnly/DOMPointReadOnly)\n\n*This API requires the following crate features to be activated: `DomPointReadOnly`*"]
    pub fn new_with_x(this: &DomPointReadOnly, x: f64) -> Result<DomPointReadOnly, JsValue>;
    #[wasm_bindgen(catch, js_class = "DOMPointReadOnly", constructor)]
    #[doc = "The `new DomPointReadOnly(..)` constructor, creating a new instance of `DomPointReadOnly`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMPointReadOnly/DOMPointReadOnly)\n\n*This API requires the following crate features to be activated: `DomPointReadOnly`*"]
    pub fn new_with_x_and_y(
        this: &DomPointReadOnly,
        x: f64,
        y: f64,
    ) -> Result<DomPointReadOnly, JsValue>;
    #[wasm_bindgen(catch, js_class = "DOMPointReadOnly", constructor)]
    #[doc = "The `new DomPointReadOnly(..)` constructor, creating a new instance of `DomPointReadOnly`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMPointReadOnly/DOMPointReadOnly)\n\n*This API requires the following crate features to be activated: `DomPointReadOnly`*"]
    pub fn new_with_x_and_y_and_z(
        this: &DomPointReadOnly,
        x: f64,
        y: f64,
        z: f64,
    ) -> Result<DomPointReadOnly, JsValue>;
    #[wasm_bindgen(catch, js_class = "DOMPointReadOnly", constructor)]
    #[doc = "The `new DomPointReadOnly(..)` constructor, creating a new instance of `DomPointReadOnly`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMPointReadOnly/DOMPointReadOnly)\n\n*This API requires the following crate features to be activated: `DomPointReadOnly`*"]
    pub fn new_with_x_and_y_and_z_and_w(
        this: &DomPointReadOnly,
        x: f64,
        y: f64,
        z: f64,
        w: f64,
    ) -> Result<DomPointReadOnly, JsValue>;
    # [ wasm_bindgen ( static_method_of = DomPointReadOnly , js_class = "DOMPointReadOnly" , js_name = fromPoint ) ]
    #[doc = "The `fromPoint()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMPointReadOnly/fromPoint)\n\n*This API requires the following crate features to be activated: `DomPointReadOnly`*"]
    pub fn from_point() -> DomPointReadOnly;
    #[cfg(feature = "DomPointInit")]
    # [ wasm_bindgen ( static_method_of = DomPointReadOnly , js_class = "DOMPointReadOnly" , js_name = fromPoint ) ]
    #[doc = "The `fromPoint()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMPointReadOnly/fromPoint)\n\n*This API requires the following crate features to be activated: `DomPointInit`, `DomPointReadOnly`*"]
    pub fn from_point_with_other(other: &DomPointInit) -> DomPointReadOnly;
    # [ wasm_bindgen ( method , structural , js_class = "DOMPointReadOnly" , js_name = toJSON ) ]
    #[doc = "The `toJSON()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMPointReadOnly/toJSON)\n\n*This API requires the following crate features to be activated: `DomPointReadOnly`*"]
    pub fn to_json(this: &DomPointReadOnly) -> ::js_sys::Object;
}
