use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = DomRectReadOnly , extends = :: js_sys :: Object , js_name = DOMRect , typescript_name = DOMRect ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `DomRect` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRect)
    ///
    ///*This API requires the following crate features to be activated: `DomRect`*
    pub type DomRect;

    # [ wasm_bindgen ( structural , method , getter , js_class = "DOMRect" , js_name = x ) ]
    ///Getter for the `x` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRect/x)
    ///
    ///*This API requires the following crate features to be activated: `DomRect`*
    pub fn x(this: &DomRect) -> f64;

    # [ wasm_bindgen ( structural , method , setter , js_class = "DOMRect" , js_name = x ) ]
    ///Setter for the `x` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRect/x)
    ///
    ///*This API requires the following crate features to be activated: `DomRect`*
    pub fn set_x(this: &DomRect, value: f64);

    # [ wasm_bindgen ( structural , method , getter , js_class = "DOMRect" , js_name = y ) ]
    ///Getter for the `y` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRect/y)
    ///
    ///*This API requires the following crate features to be activated: `DomRect`*
    pub fn y(this: &DomRect) -> f64;

    # [ wasm_bindgen ( structural , method , setter , js_class = "DOMRect" , js_name = y ) ]
    ///Setter for the `y` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRect/y)
    ///
    ///*This API requires the following crate features to be activated: `DomRect`*
    pub fn set_y(this: &DomRect, value: f64);

    # [ wasm_bindgen ( structural , method , getter , js_class = "DOMRect" , js_name = width ) ]
    ///Getter for the `width` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRect/width)
    ///
    ///*This API requires the following crate features to be activated: `DomRect`*
    pub fn width(this: &DomRect) -> f64;

    # [ wasm_bindgen ( structural , method , setter , js_class = "DOMRect" , js_name = width ) ]
    ///Setter for the `width` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRect/width)
    ///
    ///*This API requires the following crate features to be activated: `DomRect`*
    pub fn set_width(this: &DomRect, value: f64);

    # [ wasm_bindgen ( structural , method , getter , js_class = "DOMRect" , js_name = height ) ]
    ///Getter for the `height` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRect/height)
    ///
    ///*This API requires the following crate features to be activated: `DomRect`*
    pub fn height(this: &DomRect) -> f64;

    # [ wasm_bindgen ( structural , method , setter , js_class = "DOMRect" , js_name = height ) ]
    ///Setter for the `height` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRect/height)
    ///
    ///*This API requires the following crate features to be activated: `DomRect`*
    pub fn set_height(this: &DomRect, value: f64);

    #[wasm_bindgen(catch, constructor, js_class = "DOMRect")]
    ///The `new DomRect(..)` constructor, creating a new instance of `DomRect`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRect/DOMRect)
    ///
    ///*This API requires the following crate features to be activated: `DomRect`*
    pub fn new() -> Result<DomRect, JsValue>;

    #[wasm_bindgen(catch, constructor, js_class = "DOMRect")]
    ///The `new DomRect(..)` constructor, creating a new instance of `DomRect`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRect/DOMRect)
    ///
    ///*This API requires the following crate features to be activated: `DomRect`*
    pub fn new_with_x(x: f64) -> Result<DomRect, JsValue>;

    #[wasm_bindgen(catch, constructor, js_class = "DOMRect")]
    ///The `new DomRect(..)` constructor, creating a new instance of `DomRect`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRect/DOMRect)
    ///
    ///*This API requires the following crate features to be activated: `DomRect`*
    pub fn new_with_x_and_y(x: f64, y: f64) -> Result<DomRect, JsValue>;

    #[wasm_bindgen(catch, constructor, js_class = "DOMRect")]
    ///The `new DomRect(..)` constructor, creating a new instance of `DomRect`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRect/DOMRect)
    ///
    ///*This API requires the following crate features to be activated: `DomRect`*
    pub fn new_with_x_and_y_and_width(x: f64, y: f64, width: f64) -> Result<DomRect, JsValue>;

    #[wasm_bindgen(catch, constructor, js_class = "DOMRect")]
    ///The `new DomRect(..)` constructor, creating a new instance of `DomRect`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRect/DOMRect)
    ///
    ///*This API requires the following crate features to be activated: `DomRect`*
    pub fn new_with_x_and_y_and_width_and_height(
        x: f64,
        y: f64,
        width: f64,
        height: f64,
    ) -> Result<DomRect, JsValue>;

}
