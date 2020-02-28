use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = HtmlElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = HTMLLegendElement , typescript_name = HTMLLegendElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlLegendElement` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLegendElement)\n\n*This API requires the following crate features to be activated: `HtmlLegendElement`*"]
    pub type HtmlLegendElement;
    # [ wasm_bindgen ( structural , method , getter , js_name = form ) ]
    #[cfg(feature = "HtmlFormElement")]
    #[doc = "Getter for the `form` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLegendElement/form)\n\n*This API requires the following crate features to be activated: `HtmlFormElement`, `HtmlLegendElement`*"]
    pub fn form(this: &HtmlLegendElement) -> Option<HtmlFormElement>;
    # [ wasm_bindgen ( structural , method , getter , js_name = align ) ]
    #[doc = "Getter for the `align` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLegendElement/align)\n\n*This API requires the following crate features to be activated: `HtmlLegendElement`*"]
    pub fn align(this: &HtmlLegendElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = align ) ]
    #[doc = "Setter for the `align` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLegendElement/align)\n\n*This API requires the following crate features to be activated: `HtmlLegendElement`*"]
    pub fn set_align(this: &HtmlLegendElement, value: &str);
}
