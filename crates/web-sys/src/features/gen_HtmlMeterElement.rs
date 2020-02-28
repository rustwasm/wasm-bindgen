use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = HtmlElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = HTMLMeterElement , typescript_name = HTMLMeterElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlMeterElement` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMeterElement)\n\n*This API requires the following crate features to be activated: `HtmlMeterElement`*"]
    pub type HtmlMeterElement;
    # [ wasm_bindgen ( structural , method , getter , js_name = value ) ]
    #[doc = "Getter for the `value` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMeterElement/value)\n\n*This API requires the following crate features to be activated: `HtmlMeterElement`*"]
    pub fn value(this: &HtmlMeterElement) -> f64;
    # [ wasm_bindgen ( structural , method , setter , js_name = value ) ]
    #[doc = "Setter for the `value` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMeterElement/value)\n\n*This API requires the following crate features to be activated: `HtmlMeterElement`*"]
    pub fn set_value(this: &HtmlMeterElement, value: f64);
    # [ wasm_bindgen ( structural , method , getter , js_name = min ) ]
    #[doc = "Getter for the `min` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMeterElement/min)\n\n*This API requires the following crate features to be activated: `HtmlMeterElement`*"]
    pub fn min(this: &HtmlMeterElement) -> f64;
    # [ wasm_bindgen ( structural , method , setter , js_name = min ) ]
    #[doc = "Setter for the `min` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMeterElement/min)\n\n*This API requires the following crate features to be activated: `HtmlMeterElement`*"]
    pub fn set_min(this: &HtmlMeterElement, value: f64);
    # [ wasm_bindgen ( structural , method , getter , js_name = max ) ]
    #[doc = "Getter for the `max` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMeterElement/max)\n\n*This API requires the following crate features to be activated: `HtmlMeterElement`*"]
    pub fn max(this: &HtmlMeterElement) -> f64;
    # [ wasm_bindgen ( structural , method , setter , js_name = max ) ]
    #[doc = "Setter for the `max` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMeterElement/max)\n\n*This API requires the following crate features to be activated: `HtmlMeterElement`*"]
    pub fn set_max(this: &HtmlMeterElement, value: f64);
    # [ wasm_bindgen ( structural , method , getter , js_name = low ) ]
    #[doc = "Getter for the `low` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMeterElement/low)\n\n*This API requires the following crate features to be activated: `HtmlMeterElement`*"]
    pub fn low(this: &HtmlMeterElement) -> f64;
    # [ wasm_bindgen ( structural , method , setter , js_name = low ) ]
    #[doc = "Setter for the `low` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMeterElement/low)\n\n*This API requires the following crate features to be activated: `HtmlMeterElement`*"]
    pub fn set_low(this: &HtmlMeterElement, value: f64);
    # [ wasm_bindgen ( structural , method , getter , js_name = high ) ]
    #[doc = "Getter for the `high` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMeterElement/high)\n\n*This API requires the following crate features to be activated: `HtmlMeterElement`*"]
    pub fn high(this: &HtmlMeterElement) -> f64;
    # [ wasm_bindgen ( structural , method , setter , js_name = high ) ]
    #[doc = "Setter for the `high` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMeterElement/high)\n\n*This API requires the following crate features to be activated: `HtmlMeterElement`*"]
    pub fn set_high(this: &HtmlMeterElement, value: f64);
    # [ wasm_bindgen ( structural , method , getter , js_name = optimum ) ]
    #[doc = "Getter for the `optimum` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMeterElement/optimum)\n\n*This API requires the following crate features to be activated: `HtmlMeterElement`*"]
    pub fn optimum(this: &HtmlMeterElement) -> f64;
    # [ wasm_bindgen ( structural , method , setter , js_name = optimum ) ]
    #[doc = "Setter for the `optimum` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMeterElement/optimum)\n\n*This API requires the following crate features to be activated: `HtmlMeterElement`*"]
    pub fn set_optimum(this: &HtmlMeterElement, value: f64);
    # [ wasm_bindgen ( structural , method , getter , js_name = labels ) ]
    #[cfg(feature = "NodeList")]
    #[doc = "Getter for the `labels` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMeterElement/labels)\n\n*This API requires the following crate features to be activated: `HtmlMeterElement`, `NodeList`*"]
    pub fn labels(this: &HtmlMeterElement) -> NodeList;
}
