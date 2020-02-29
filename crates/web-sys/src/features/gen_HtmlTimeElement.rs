use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = HtmlElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = HTMLTimeElement , typescript_name = HTMLTimeElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `HtmlTimeElement` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTimeElement)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTimeElement`*
    pub type HtmlTimeElement;

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLTimeElement" , js_name = dateTime ) ]
    ///Getter for the `dateTime` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTimeElement/dateTime)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTimeElement`*
    pub fn date_time(this: &HtmlTimeElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLTimeElement" , js_name = dateTime ) ]
    ///Setter for the `dateTime` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTimeElement/dateTime)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTimeElement`*
    pub fn set_date_time(this: &HtmlTimeElement, value: &str);

}
