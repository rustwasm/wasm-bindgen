use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = DOMRectReadOnly , typescript_name = DOMRectReadOnly ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DomRectReadOnly` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRectReadOnly)\n\n*This API requires the following crate features to be activated: `DomRectReadOnly`*"]
    pub type DomRectReadOnly;
    # [ wasm_bindgen ( structural , method , getter , js_class = "DOMRectReadOnly" , js_name = x ) ]
    #[doc = "Getter for the `x` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRectReadOnly/x)\n\n*This API requires the following crate features to be activated: `DomRectReadOnly`*"]
    pub fn x(this: &DomRectReadOnly) -> f64;
    # [ wasm_bindgen ( structural , method , getter , js_class = "DOMRectReadOnly" , js_name = y ) ]
    #[doc = "Getter for the `y` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRectReadOnly/y)\n\n*This API requires the following crate features to be activated: `DomRectReadOnly`*"]
    pub fn y(this: &DomRectReadOnly) -> f64;
    # [ wasm_bindgen ( structural , method , getter , js_class = "DOMRectReadOnly" , js_name = width ) ]
    #[doc = "Getter for the `width` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRectReadOnly/width)\n\n*This API requires the following crate features to be activated: `DomRectReadOnly`*"]
    pub fn width(this: &DomRectReadOnly) -> f64;
    # [ wasm_bindgen ( structural , method , getter , js_class = "DOMRectReadOnly" , js_name = height ) ]
    #[doc = "Getter for the `height` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRectReadOnly/height)\n\n*This API requires the following crate features to be activated: `DomRectReadOnly`*"]
    pub fn height(this: &DomRectReadOnly) -> f64;
    # [ wasm_bindgen ( structural , method , getter , js_class = "DOMRectReadOnly" , js_name = top ) ]
    #[doc = "Getter for the `top` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRectReadOnly/top)\n\n*This API requires the following crate features to be activated: `DomRectReadOnly`*"]
    pub fn top(this: &DomRectReadOnly) -> f64;
    # [ wasm_bindgen ( structural , method , getter , js_class = "DOMRectReadOnly" , js_name = right ) ]
    #[doc = "Getter for the `right` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRectReadOnly/right)\n\n*This API requires the following crate features to be activated: `DomRectReadOnly`*"]
    pub fn right(this: &DomRectReadOnly) -> f64;
    # [ wasm_bindgen ( structural , method , getter , js_class = "DOMRectReadOnly" , js_name = bottom ) ]
    #[doc = "Getter for the `bottom` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRectReadOnly/bottom)\n\n*This API requires the following crate features to be activated: `DomRectReadOnly`*"]
    pub fn bottom(this: &DomRectReadOnly) -> f64;
    # [ wasm_bindgen ( structural , method , getter , js_class = "DOMRectReadOnly" , js_name = left ) ]
    #[doc = "Getter for the `left` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRectReadOnly/left)\n\n*This API requires the following crate features to be activated: `DomRectReadOnly`*"]
    pub fn left(this: &DomRectReadOnly) -> f64;
    #[wasm_bindgen(catch, js_class = "DOMRectReadOnly", constructor)]
    #[doc = "The `new DomRectReadOnly(..)` constructor, creating a new instance of `DomRectReadOnly`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRectReadOnly/DOMRectReadOnly)\n\n*This API requires the following crate features to be activated: `DomRectReadOnly`*"]
    pub fn new(this: &DomRectReadOnly) -> Result<DomRectReadOnly, JsValue>;
    #[wasm_bindgen(catch, js_class = "DOMRectReadOnly", constructor)]
    #[doc = "The `new DomRectReadOnly(..)` constructor, creating a new instance of `DomRectReadOnly`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRectReadOnly/DOMRectReadOnly)\n\n*This API requires the following crate features to be activated: `DomRectReadOnly`*"]
    pub fn new_with_x(this: &DomRectReadOnly, x: f64) -> Result<DomRectReadOnly, JsValue>;
    #[wasm_bindgen(catch, js_class = "DOMRectReadOnly", constructor)]
    #[doc = "The `new DomRectReadOnly(..)` constructor, creating a new instance of `DomRectReadOnly`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRectReadOnly/DOMRectReadOnly)\n\n*This API requires the following crate features to be activated: `DomRectReadOnly`*"]
    pub fn new_with_x_and_y(
        this: &DomRectReadOnly,
        x: f64,
        y: f64,
    ) -> Result<DomRectReadOnly, JsValue>;
    #[wasm_bindgen(catch, js_class = "DOMRectReadOnly", constructor)]
    #[doc = "The `new DomRectReadOnly(..)` constructor, creating a new instance of `DomRectReadOnly`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRectReadOnly/DOMRectReadOnly)\n\n*This API requires the following crate features to be activated: `DomRectReadOnly`*"]
    pub fn new_with_x_and_y_and_width(
        this: &DomRectReadOnly,
        x: f64,
        y: f64,
        width: f64,
    ) -> Result<DomRectReadOnly, JsValue>;
    #[wasm_bindgen(catch, js_class = "DOMRectReadOnly", constructor)]
    #[doc = "The `new DomRectReadOnly(..)` constructor, creating a new instance of `DomRectReadOnly`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRectReadOnly/DOMRectReadOnly)\n\n*This API requires the following crate features to be activated: `DomRectReadOnly`*"]
    pub fn new_with_x_and_y_and_width_and_height(
        this: &DomRectReadOnly,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
    ) -> Result<DomRectReadOnly, JsValue>;
    # [ wasm_bindgen ( method , structural , js_class = "DOMRectReadOnly" , js_name = toJSON ) ]
    #[doc = "The `toJSON()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRectReadOnly/toJSON)\n\n*This API requires the following crate features to be activated: `DomRectReadOnly`*"]
    pub fn to_json(this: &DomRectReadOnly) -> ::js_sys::Object;
}
