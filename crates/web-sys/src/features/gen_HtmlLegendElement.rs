use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = HtmlElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = HTMLLegendElement , typescript_name = HTMLLegendElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `HtmlLegendElement` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLegendElement)
    ///
    ///*This API requires the following crate features to be activated: `HtmlLegendElement`*
    pub type HtmlLegendElement;

    #[cfg(feature = "HtmlFormElement")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLLegendElement" , js_name = form ) ]
    ///Getter for the `form` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLegendElement/form)
    ///
    ///*This API requires the following crate features to be activated: `HtmlFormElement`, `HtmlLegendElement`*
    pub fn form(this: &HtmlLegendElement) -> Option<HtmlFormElement>;

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLLegendElement" , js_name = align ) ]
    ///Getter for the `align` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLegendElement/align)
    ///
    ///*This API requires the following crate features to be activated: `HtmlLegendElement`*
    pub fn align(this: &HtmlLegendElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLLegendElement" , js_name = align ) ]
    ///Setter for the `align` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLegendElement/align)
    ///
    ///*This API requires the following crate features to be activated: `HtmlLegendElement`*
    pub fn set_align(this: &HtmlLegendElement, value: &str);

}
