use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = HtmlElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = HTMLOptGroupElement , typescript_name = HTMLOptGroupElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlOptGroupElement` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptGroupElement)\n\n*This API requires the following crate features to be activated: `HtmlOptGroupElement`*"]
    pub type HtmlOptGroupElement;
    # [ wasm_bindgen ( structural , method , getter , js_name = disabled ) ]
    #[doc = "Getter for the `disabled` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptGroupElement/disabled)\n\n*This API requires the following crate features to be activated: `HtmlOptGroupElement`*"]
    pub fn disabled(this: &HtmlOptGroupElement) -> bool;
    # [ wasm_bindgen ( structural , method , setter , js_name = disabled ) ]
    #[doc = "Setter for the `disabled` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptGroupElement/disabled)\n\n*This API requires the following crate features to be activated: `HtmlOptGroupElement`*"]
    pub fn set_disabled(this: &HtmlOptGroupElement, value: bool);
    # [ wasm_bindgen ( structural , method , getter , js_name = label ) ]
    #[doc = "Getter for the `label` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptGroupElement/label)\n\n*This API requires the following crate features to be activated: `HtmlOptGroupElement`*"]
    pub fn label(this: &HtmlOptGroupElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = label ) ]
    #[doc = "Setter for the `label` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptGroupElement/label)\n\n*This API requires the following crate features to be activated: `HtmlOptGroupElement`*"]
    pub fn set_label(this: &HtmlOptGroupElement, value: &str);
}
