use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = HtmlElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = HTMLMetaElement , typescript_name = HTMLMetaElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `HtmlMetaElement` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMetaElement)
    ///
    ///*This API requires the following crate features to be activated: `HtmlMetaElement`*
    pub type HtmlMetaElement;

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLMetaElement" , js_name = name ) ]
    ///Getter for the `name` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMetaElement/name)
    ///
    ///*This API requires the following crate features to be activated: `HtmlMetaElement`*
    pub fn name(this: &HtmlMetaElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLMetaElement" , js_name = name ) ]
    ///Setter for the `name` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMetaElement/name)
    ///
    ///*This API requires the following crate features to be activated: `HtmlMetaElement`*
    pub fn set_name(this: &HtmlMetaElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLMetaElement" , js_name = httpEquiv ) ]
    ///Getter for the `httpEquiv` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMetaElement/httpEquiv)
    ///
    ///*This API requires the following crate features to be activated: `HtmlMetaElement`*
    pub fn http_equiv(this: &HtmlMetaElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLMetaElement" , js_name = httpEquiv ) ]
    ///Setter for the `httpEquiv` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMetaElement/httpEquiv)
    ///
    ///*This API requires the following crate features to be activated: `HtmlMetaElement`*
    pub fn set_http_equiv(this: &HtmlMetaElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLMetaElement" , js_name = content ) ]
    ///Getter for the `content` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMetaElement/content)
    ///
    ///*This API requires the following crate features to be activated: `HtmlMetaElement`*
    pub fn content(this: &HtmlMetaElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLMetaElement" , js_name = content ) ]
    ///Setter for the `content` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMetaElement/content)
    ///
    ///*This API requires the following crate features to be activated: `HtmlMetaElement`*
    pub fn set_content(this: &HtmlMetaElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLMetaElement" , js_name = scheme ) ]
    ///Getter for the `scheme` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMetaElement/scheme)
    ///
    ///*This API requires the following crate features to be activated: `HtmlMetaElement`*
    pub fn scheme(this: &HtmlMetaElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLMetaElement" , js_name = scheme ) ]
    ///Setter for the `scheme` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMetaElement/scheme)
    ///
    ///*This API requires the following crate features to be activated: `HtmlMetaElement`*
    pub fn set_scheme(this: &HtmlMetaElement, value: &str);

}
