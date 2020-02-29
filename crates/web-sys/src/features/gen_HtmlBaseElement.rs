use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = HtmlElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = HTMLBaseElement , typescript_name = HTMLBaseElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `HtmlBaseElement` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBaseElement)
    ///
    ///*This API requires the following crate features to be activated: `HtmlBaseElement`*
    pub type HtmlBaseElement;

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLBaseElement" , js_name = href ) ]
    ///Getter for the `href` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBaseElement/href)
    ///
    ///*This API requires the following crate features to be activated: `HtmlBaseElement`*
    pub fn href(this: &HtmlBaseElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLBaseElement" , js_name = href ) ]
    ///Setter for the `href` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBaseElement/href)
    ///
    ///*This API requires the following crate features to be activated: `HtmlBaseElement`*
    pub fn set_href(this: &HtmlBaseElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLBaseElement" , js_name = target ) ]
    ///Getter for the `target` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBaseElement/target)
    ///
    ///*This API requires the following crate features to be activated: `HtmlBaseElement`*
    pub fn target(this: &HtmlBaseElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLBaseElement" , js_name = target ) ]
    ///Setter for the `target` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBaseElement/target)
    ///
    ///*This API requires the following crate features to be activated: `HtmlBaseElement`*
    pub fn set_target(this: &HtmlBaseElement, value: &str);

}
