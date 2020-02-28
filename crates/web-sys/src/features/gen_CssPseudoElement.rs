use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = CSSPseudoElement , typescript_name = CSSPseudoElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `CssPseudoElement` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSPseudoElement)\n\n*This API requires the following crate features to be activated: `CssPseudoElement`*"]
    pub type CssPseudoElement;
    # [ wasm_bindgen ( structural , method , getter , js_class = "CSSPseudoElement" , js_name = type ) ]
    #[doc = "Getter for the `type` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSPseudoElement/type)\n\n*This API requires the following crate features to be activated: `CssPseudoElement`*"]
    pub fn type_(this: &CssPseudoElement) -> String;
    # [ wasm_bindgen ( structural , method , getter , js_class = "CSSPseudoElement" , js_name = parentElement ) ]
    #[cfg(feature = "Element")]
    #[doc = "Getter for the `parentElement` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSPseudoElement/parentElement)\n\n*This API requires the following crate features to be activated: `CssPseudoElement`, `Element`*"]
    pub fn parent_element(this: &CssPseudoElement) -> Element;
}
