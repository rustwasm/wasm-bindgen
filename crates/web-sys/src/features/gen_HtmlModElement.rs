use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = HtmlElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = HTMLModElement , typescript_name = HTMLModElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `HtmlModElement` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLModElement)
    ///
    ///*This API requires the following crate features to be activated: `HtmlModElement`*
    pub type HtmlModElement;

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLModElement" , js_name = cite ) ]
    ///Getter for the `cite` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLModElement/cite)
    ///
    ///*This API requires the following crate features to be activated: `HtmlModElement`*
    pub fn cite(this: &HtmlModElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLModElement" , js_name = cite ) ]
    ///Setter for the `cite` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLModElement/cite)
    ///
    ///*This API requires the following crate features to be activated: `HtmlModElement`*
    pub fn set_cite(this: &HtmlModElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLModElement" , js_name = dateTime ) ]
    ///Getter for the `dateTime` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLModElement/dateTime)
    ///
    ///*This API requires the following crate features to be activated: `HtmlModElement`*
    pub fn date_time(this: &HtmlModElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLModElement" , js_name = dateTime ) ]
    ///Setter for the `dateTime` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLModElement/dateTime)
    ///
    ///*This API requires the following crate features to be activated: `HtmlModElement`*
    pub fn set_date_time(this: &HtmlModElement, value: &str);

}
