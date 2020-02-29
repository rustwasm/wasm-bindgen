use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = CssRule , extends = :: js_sys :: Object , js_name = CSSNamespaceRule , typescript_type = "CSSNamespaceRule" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `CssNamespaceRule` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSNamespaceRule)
    ///
    ///*This API requires the following crate features to be activated: `CssNamespaceRule`*
    pub type CssNamespaceRule;

    # [ wasm_bindgen ( structural , method , getter , js_class = "CSSNamespaceRule" , js_name = namespaceURI ) ]
    ///Getter for the `namespaceURI` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSNamespaceRule/namespaceURI)
    ///
    ///*This API requires the following crate features to be activated: `CssNamespaceRule`*
    pub fn namespace_uri(this: &CssNamespaceRule) -> String;

    # [ wasm_bindgen ( structural , method , getter , js_class = "CSSNamespaceRule" , js_name = prefix ) ]
    ///Getter for the `prefix` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSNamespaceRule/prefix)
    ///
    ///*This API requires the following crate features to be activated: `CssNamespaceRule`*
    pub fn prefix(this: &CssNamespaceRule) -> String;

}
