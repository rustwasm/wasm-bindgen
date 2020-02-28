use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = DomRectReadOnly , extends = :: js_sys :: Object , js_name = DOMRect , typescript_name = DOMRect ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DomRect` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRect)\n\n*This API requires the following crate features to be activated: `DomRect`*"]
    pub type DomRect;
    # [ wasm_bindgen ( structural , method , getter , js_name = x ) ]
    #[doc = "Getter for the `x` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRect/x)\n\n*This API requires the following crate features to be activated: `DomRect`*"]
    pub fn x(this: &DomRect) -> f64;
    # [ wasm_bindgen ( structural , method , setter , js_name = x ) ]
    #[doc = "Setter for the `x` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRect/x)\n\n*This API requires the following crate features to be activated: `DomRect`*"]
    pub fn set_x(this: &DomRect, value: f64);
    # [ wasm_bindgen ( structural , method , getter , js_name = y ) ]
    #[doc = "Getter for the `y` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRect/y)\n\n*This API requires the following crate features to be activated: `DomRect`*"]
    pub fn y(this: &DomRect) -> f64;
    # [ wasm_bindgen ( structural , method , setter , js_name = y ) ]
    #[doc = "Setter for the `y` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRect/y)\n\n*This API requires the following crate features to be activated: `DomRect`*"]
    pub fn set_y(this: &DomRect, value: f64);
    # [ wasm_bindgen ( structural , method , getter , js_name = width ) ]
    #[doc = "Getter for the `width` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRect/width)\n\n*This API requires the following crate features to be activated: `DomRect`*"]
    pub fn width(this: &DomRect) -> f64;
    # [ wasm_bindgen ( structural , method , setter , js_name = width ) ]
    #[doc = "Setter for the `width` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRect/width)\n\n*This API requires the following crate features to be activated: `DomRect`*"]
    pub fn set_width(this: &DomRect, value: f64);
    # [ wasm_bindgen ( structural , method , getter , js_name = height ) ]
    #[doc = "Getter for the `height` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRect/height)\n\n*This API requires the following crate features to be activated: `DomRect`*"]
    pub fn height(this: &DomRect) -> f64;
    # [ wasm_bindgen ( structural , method , setter , js_name = height ) ]
    #[doc = "Setter for the `height` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRect/height)\n\n*This API requires the following crate features to be activated: `DomRect`*"]
    pub fn set_height(this: &DomRect, value: f64);
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new DomRect(..)` constructor, creating a new instance of `DomRect`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRect/DOMRect)\n\n*This API requires the following crate features to be activated: `DomRect`*"]
    pub fn new(this: &DomRect) -> Result<DomRect, JsValue>;
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new DomRect(..)` constructor, creating a new instance of `DomRect`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRect/DOMRect)\n\n*This API requires the following crate features to be activated: `DomRect`*"]
    pub fn new_with_x(this: &DomRect, x: f64) -> Result<DomRect, JsValue>;
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new DomRect(..)` constructor, creating a new instance of `DomRect`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRect/DOMRect)\n\n*This API requires the following crate features to be activated: `DomRect`*"]
    pub fn new_with_x_and_y(this: &DomRect, x: f64, y: f64) -> Result<DomRect, JsValue>;
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new DomRect(..)` constructor, creating a new instance of `DomRect`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRect/DOMRect)\n\n*This API requires the following crate features to be activated: `DomRect`*"]
    pub fn new_with_x_and_y_and_width(
        this: &DomRect,
        x: f64,
        y: f64,
        width: f64,
    ) -> Result<DomRect, JsValue>;
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new DomRect(..)` constructor, creating a new instance of `DomRect`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRect/DOMRect)\n\n*This API requires the following crate features to be activated: `DomRect`*"]
    pub fn new_with_x_and_y_and_width_and_height(
        this: &DomRect,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
    ) -> Result<DomRect, JsValue>;
}
