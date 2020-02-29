use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = HtmlElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = HTMLTemplateElement , typescript_type = "HTMLTemplateElement" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `HtmlTemplateElement` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTemplateElement)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTemplateElement`*
    pub type HtmlTemplateElement;

    #[cfg(feature = "DocumentFragment")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLTemplateElement" , js_name = content ) ]
    ///Getter for the `content` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTemplateElement/content)
    ///
    ///*This API requires the following crate features to be activated: `DocumentFragment`, `HtmlTemplateElement`*
    pub fn content(this: &HtmlTemplateElement) -> DocumentFragment;

}
