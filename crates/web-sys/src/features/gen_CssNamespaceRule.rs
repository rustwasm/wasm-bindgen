use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = CssRule , extends = :: js_sys :: Object , js_name = CSSNamespaceRule , typescript_name = CSSNamespaceRule ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `CssNamespaceRule` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSNamespaceRule)\n\n*This API requires the following crate features to be activated: `CssNamespaceRule`*"]
    pub type CssNamespaceRule;
    # [ wasm_bindgen ( structural , method , getter , js_class = "CSSNamespaceRule" , js_name = namespaceURI ) ]
    #[doc = "Getter for the `namespaceURI` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSNamespaceRule/namespaceURI)\n\n*This API requires the following crate features to be activated: `CssNamespaceRule`*"]
    pub fn namespace_uri(this: &CssNamespaceRule) -> String;
    # [ wasm_bindgen ( structural , method , getter , js_class = "CSSNamespaceRule" , js_name = prefix ) ]
    #[doc = "Getter for the `prefix` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSNamespaceRule/prefix)\n\n*This API requires the following crate features to be activated: `CssNamespaceRule`*"]
    pub fn prefix(this: &CssNamespaceRule) -> String;
}
