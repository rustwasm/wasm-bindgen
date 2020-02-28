use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = HtmlElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = HTMLTemplateElement , typescript_name = HTMLTemplateElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlTemplateElement` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTemplateElement)\n\n*This API requires the following crate features to be activated: `HtmlTemplateElement`*"]
    pub type HtmlTemplateElement;
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLTemplateElement" , js_name = content ) ]
    #[cfg(feature = "DocumentFragment")]
    #[doc = "Getter for the `content` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTemplateElement/content)\n\n*This API requires the following crate features to be activated: `DocumentFragment`, `HtmlTemplateElement`*"]
    pub fn content(this: &HtmlTemplateElement) -> DocumentFragment;
}
