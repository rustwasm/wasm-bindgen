use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = DOMRectReadOnly , typescript_type = "DOMRectReadOnly" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `DomRectReadOnly` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRectReadOnly)
    ///
    ///*This API requires the following crate features to be activated: `DomRectReadOnly`*
    pub type DomRectReadOnly;

    # [ wasm_bindgen ( structural , method , getter , js_class = "DOMRectReadOnly" , js_name = x ) ]
    ///Getter for the `x` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRectReadOnly/x)
    ///
    ///*This API requires the following crate features to be activated: `DomRectReadOnly`*
    pub fn x(this: &DomRectReadOnly) -> f64;

    # [ wasm_bindgen ( structural , method , getter , js_class = "DOMRectReadOnly" , js_name = y ) ]
    ///Getter for the `y` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRectReadOnly/y)
    ///
    ///*This API requires the following crate features to be activated: `DomRectReadOnly`*
    pub fn y(this: &DomRectReadOnly) -> f64;

    # [ wasm_bindgen ( structural , method , getter , js_class = "DOMRectReadOnly" , js_name = width ) ]
    ///Getter for the `width` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRectReadOnly/width)
    ///
    ///*This API requires the following crate features to be activated: `DomRectReadOnly`*
    pub fn width(this: &DomRectReadOnly) -> f64;

    # [ wasm_bindgen ( structural , method , getter , js_class = "DOMRectReadOnly" , js_name = height ) ]
    ///Getter for the `height` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRectReadOnly/height)
    ///
    ///*This API requires the following crate features to be activated: `DomRectReadOnly`*
    pub fn height(this: &DomRectReadOnly) -> f64;

    # [ wasm_bindgen ( structural , method , getter , js_class = "DOMRectReadOnly" , js_name = top ) ]
    ///Getter for the `top` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRectReadOnly/top)
    ///
    ///*This API requires the following crate features to be activated: `DomRectReadOnly`*
    pub fn top(this: &DomRectReadOnly) -> f64;

    # [ wasm_bindgen ( structural , method , getter , js_class = "DOMRectReadOnly" , js_name = right ) ]
    ///Getter for the `right` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRectReadOnly/right)
    ///
    ///*This API requires the following crate features to be activated: `DomRectReadOnly`*
    pub fn right(this: &DomRectReadOnly) -> f64;

    # [ wasm_bindgen ( structural , method , getter , js_class = "DOMRectReadOnly" , js_name = bottom ) ]
    ///Getter for the `bottom` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRectReadOnly/bottom)
    ///
    ///*This API requires the following crate features to be activated: `DomRectReadOnly`*
    pub fn bottom(this: &DomRectReadOnly) -> f64;

    # [ wasm_bindgen ( structural , method , getter , js_class = "DOMRectReadOnly" , js_name = left ) ]
    ///Getter for the `left` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRectReadOnly/left)
    ///
    ///*This API requires the following crate features to be activated: `DomRectReadOnly`*
    pub fn left(this: &DomRectReadOnly) -> f64;

    #[wasm_bindgen(catch, constructor, js_class = "DOMRectReadOnly")]
    ///The `new DomRectReadOnly(..)` constructor, creating a new instance of `DomRectReadOnly`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRectReadOnly/DOMRectReadOnly)
    ///
    ///*This API requires the following crate features to be activated: `DomRectReadOnly`*
    pub fn new() -> Result<DomRectReadOnly, JsValue>;

    #[wasm_bindgen(catch, constructor, js_class = "DOMRectReadOnly")]
    ///The `new DomRectReadOnly(..)` constructor, creating a new instance of `DomRectReadOnly`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRectReadOnly/DOMRectReadOnly)
    ///
    ///*This API requires the following crate features to be activated: `DomRectReadOnly`*
    pub fn new_with_x(x: f64) -> Result<DomRectReadOnly, JsValue>;

    #[wasm_bindgen(catch, constructor, js_class = "DOMRectReadOnly")]
    ///The `new DomRectReadOnly(..)` constructor, creating a new instance of `DomRectReadOnly`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRectReadOnly/DOMRectReadOnly)
    ///
    ///*This API requires the following crate features to be activated: `DomRectReadOnly`*
    pub fn new_with_x_and_y(x: f64, y: f64) -> Result<DomRectReadOnly, JsValue>;

    #[wasm_bindgen(catch, constructor, js_class = "DOMRectReadOnly")]
    ///The `new DomRectReadOnly(..)` constructor, creating a new instance of `DomRectReadOnly`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRectReadOnly/DOMRectReadOnly)
    ///
    ///*This API requires the following crate features to be activated: `DomRectReadOnly`*
    pub fn new_with_x_and_y_and_width(
        x: f64,
        y: f64,
        width: f64,
    ) -> Result<DomRectReadOnly, JsValue>;

    #[wasm_bindgen(catch, constructor, js_class = "DOMRectReadOnly")]
    ///The `new DomRectReadOnly(..)` constructor, creating a new instance of `DomRectReadOnly`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRectReadOnly/DOMRectReadOnly)
    ///
    ///*This API requires the following crate features to be activated: `DomRectReadOnly`*
    pub fn new_with_x_and_y_and_width_and_height(
        x: f64,
        y: f64,
        width: f64,
        height: f64,
    ) -> Result<DomRectReadOnly, JsValue>;

    # [ wasm_bindgen ( method , structural , js_class = "DOMRectReadOnly" , js_name = toJSON ) ]
    ///The `toJSON()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRectReadOnly/toJSON)
    ///
    ///*This API requires the following crate features to be activated: `DomRectReadOnly`*
    pub fn to_json(this: &DomRectReadOnly) -> ::js_sys::Object;

}
