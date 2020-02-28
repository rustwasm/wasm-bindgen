use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = HtmlElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = HTMLLabelElement , typescript_name = HTMLLabelElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlLabelElement` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLabelElement)\n\n*This API requires the following crate features to be activated: `HtmlLabelElement`*"]
    pub type HtmlLabelElement;
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLLabelElement" , js_name = form ) ]
    #[cfg(feature = "HtmlFormElement")]
    #[doc = "Getter for the `form` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLabelElement/form)\n\n*This API requires the following crate features to be activated: `HtmlFormElement`, `HtmlLabelElement`*"]
    pub fn form(this: &HtmlLabelElement) -> Option<HtmlFormElement>;
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLLabelElement" , js_name = htmlFor ) ]
    #[doc = "Getter for the `htmlFor` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLabelElement/htmlFor)\n\n*This API requires the following crate features to be activated: `HtmlLabelElement`*"]
    pub fn html_for(this: &HtmlLabelElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLLabelElement" , js_name = htmlFor ) ]
    #[doc = "Setter for the `htmlFor` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLabelElement/htmlFor)\n\n*This API requires the following crate features to be activated: `HtmlLabelElement`*"]
    pub fn set_html_for(this: &HtmlLabelElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLLabelElement" , js_name = control ) ]
    #[doc = "Getter for the `control` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLabelElement/control)\n\n*This API requires the following crate features to be activated: `HtmlLabelElement`*"]
    pub fn control(this: &HtmlLabelElement) -> Option<HtmlElement>;
}
