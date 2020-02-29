use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = HtmlElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = HTMLMapElement , typescript_type = "HTMLMapElement" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `HtmlMapElement` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMapElement)
    ///
    ///*This API requires the following crate features to be activated: `HtmlMapElement`*
    pub type HtmlMapElement;

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLMapElement" , js_name = name ) ]
    ///Getter for the `name` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMapElement/name)
    ///
    ///*This API requires the following crate features to be activated: `HtmlMapElement`*
    pub fn name(this: &HtmlMapElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLMapElement" , js_name = name ) ]
    ///Setter for the `name` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMapElement/name)
    ///
    ///*This API requires the following crate features to be activated: `HtmlMapElement`*
    pub fn set_name(this: &HtmlMapElement, value: &str);

    #[cfg(feature = "HtmlCollection")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLMapElement" , js_name = areas ) ]
    ///Getter for the `areas` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMapElement/areas)
    ///
    ///*This API requires the following crate features to be activated: `HtmlCollection`, `HtmlMapElement`*
    pub fn areas(this: &HtmlMapElement) -> HtmlCollection;

}
