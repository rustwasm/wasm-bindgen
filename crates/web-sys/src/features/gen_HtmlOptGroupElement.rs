use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = HtmlElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = HTMLOptGroupElement , typescript_type = "HTMLOptGroupElement" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `HtmlOptGroupElement` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptGroupElement)
    ///
    ///*This API requires the following crate features to be activated: `HtmlOptGroupElement`*
    pub type HtmlOptGroupElement;

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLOptGroupElement" , js_name = disabled ) ]
    ///Getter for the `disabled` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptGroupElement/disabled)
    ///
    ///*This API requires the following crate features to be activated: `HtmlOptGroupElement`*
    pub fn disabled(this: &HtmlOptGroupElement) -> bool;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLOptGroupElement" , js_name = disabled ) ]
    ///Setter for the `disabled` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptGroupElement/disabled)
    ///
    ///*This API requires the following crate features to be activated: `HtmlOptGroupElement`*
    pub fn set_disabled(this: &HtmlOptGroupElement, value: bool);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLOptGroupElement" , js_name = label ) ]
    ///Getter for the `label` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptGroupElement/label)
    ///
    ///*This API requires the following crate features to be activated: `HtmlOptGroupElement`*
    pub fn label(this: &HtmlOptGroupElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLOptGroupElement" , js_name = label ) ]
    ///Setter for the `label` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptGroupElement/label)
    ///
    ///*This API requires the following crate features to be activated: `HtmlOptGroupElement`*
    pub fn set_label(this: &HtmlOptGroupElement, value: &str);

}
