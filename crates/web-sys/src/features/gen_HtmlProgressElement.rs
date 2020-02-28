use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = HtmlElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = HTMLProgressElement , typescript_name = HTMLProgressElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlProgressElement` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLProgressElement)\n\n*This API requires the following crate features to be activated: `HtmlProgressElement`*"]
    pub type HtmlProgressElement;
    # [ wasm_bindgen ( structural , method , getter , js_name = value ) ]
    #[doc = "Getter for the `value` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLProgressElement/value)\n\n*This API requires the following crate features to be activated: `HtmlProgressElement`*"]
    pub fn value(this: &HtmlProgressElement) -> f64;
    # [ wasm_bindgen ( structural , method , setter , js_name = value ) ]
    #[doc = "Setter for the `value` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLProgressElement/value)\n\n*This API requires the following crate features to be activated: `HtmlProgressElement`*"]
    pub fn set_value(this: &HtmlProgressElement, value: f64);
    # [ wasm_bindgen ( structural , method , getter , js_name = max ) ]
    #[doc = "Getter for the `max` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLProgressElement/max)\n\n*This API requires the following crate features to be activated: `HtmlProgressElement`*"]
    pub fn max(this: &HtmlProgressElement) -> f64;
    # [ wasm_bindgen ( structural , method , setter , js_name = max ) ]
    #[doc = "Setter for the `max` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLProgressElement/max)\n\n*This API requires the following crate features to be activated: `HtmlProgressElement`*"]
    pub fn set_max(this: &HtmlProgressElement, value: f64);
    # [ wasm_bindgen ( structural , method , getter , js_name = position ) ]
    #[doc = "Getter for the `position` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLProgressElement/position)\n\n*This API requires the following crate features to be activated: `HtmlProgressElement`*"]
    pub fn position(this: &HtmlProgressElement) -> f64;
    # [ wasm_bindgen ( structural , method , getter , js_name = labels ) ]
    #[cfg(feature = "NodeList")]
    #[doc = "Getter for the `labels` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLProgressElement/labels)\n\n*This API requires the following crate features to be activated: `HtmlProgressElement`, `NodeList`*"]
    pub fn labels(this: &HtmlProgressElement) -> NodeList;
}
