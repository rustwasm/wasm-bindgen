use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = HtmlElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = HTMLStyleElement , typescript_name = HTMLStyleElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `HtmlStyleElement` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLStyleElement)
    ///
    ///*This API requires the following crate features to be activated: `HtmlStyleElement`*
    pub type HtmlStyleElement;

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLStyleElement" , js_name = disabled ) ]
    ///Getter for the `disabled` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLStyleElement/disabled)
    ///
    ///*This API requires the following crate features to be activated: `HtmlStyleElement`*
    pub fn disabled(this: &HtmlStyleElement) -> bool;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLStyleElement" , js_name = disabled ) ]
    ///Setter for the `disabled` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLStyleElement/disabled)
    ///
    ///*This API requires the following crate features to be activated: `HtmlStyleElement`*
    pub fn set_disabled(this: &HtmlStyleElement, value: bool);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLStyleElement" , js_name = media ) ]
    ///Getter for the `media` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLStyleElement/media)
    ///
    ///*This API requires the following crate features to be activated: `HtmlStyleElement`*
    pub fn media(this: &HtmlStyleElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLStyleElement" , js_name = media ) ]
    ///Setter for the `media` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLStyleElement/media)
    ///
    ///*This API requires the following crate features to be activated: `HtmlStyleElement`*
    pub fn set_media(this: &HtmlStyleElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLStyleElement" , js_name = type ) ]
    ///Getter for the `type` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLStyleElement/type)
    ///
    ///*This API requires the following crate features to be activated: `HtmlStyleElement`*
    pub fn type_(this: &HtmlStyleElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLStyleElement" , js_name = type ) ]
    ///Setter for the `type` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLStyleElement/type)
    ///
    ///*This API requires the following crate features to be activated: `HtmlStyleElement`*
    pub fn set_type(this: &HtmlStyleElement, value: &str);

    #[cfg(feature = "StyleSheet")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLStyleElement" , js_name = sheet ) ]
    ///Getter for the `sheet` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLStyleElement/sheet)
    ///
    ///*This API requires the following crate features to be activated: `HtmlStyleElement`, `StyleSheet`*
    pub fn sheet(this: &HtmlStyleElement) -> Option<StyleSheet>;

}
