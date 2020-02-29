use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = HtmlElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = HTMLLinkElement , typescript_name = HTMLLinkElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `HtmlLinkElement` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement)
    ///
    ///*This API requires the following crate features to be activated: `HtmlLinkElement`*
    pub type HtmlLinkElement;

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLLinkElement" , js_name = disabled ) ]
    ///Getter for the `disabled` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/disabled)
    ///
    ///*This API requires the following crate features to be activated: `HtmlLinkElement`*
    pub fn disabled(this: &HtmlLinkElement) -> bool;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLLinkElement" , js_name = disabled ) ]
    ///Setter for the `disabled` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/disabled)
    ///
    ///*This API requires the following crate features to be activated: `HtmlLinkElement`*
    pub fn set_disabled(this: &HtmlLinkElement, value: bool);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLLinkElement" , js_name = href ) ]
    ///Getter for the `href` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/href)
    ///
    ///*This API requires the following crate features to be activated: `HtmlLinkElement`*
    pub fn href(this: &HtmlLinkElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLLinkElement" , js_name = href ) ]
    ///Setter for the `href` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/href)
    ///
    ///*This API requires the following crate features to be activated: `HtmlLinkElement`*
    pub fn set_href(this: &HtmlLinkElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLLinkElement" , js_name = crossOrigin ) ]
    ///Getter for the `crossOrigin` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/crossOrigin)
    ///
    ///*This API requires the following crate features to be activated: `HtmlLinkElement`*
    pub fn cross_origin(this: &HtmlLinkElement) -> Option<String>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLLinkElement" , js_name = crossOrigin ) ]
    ///Setter for the `crossOrigin` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/crossOrigin)
    ///
    ///*This API requires the following crate features to be activated: `HtmlLinkElement`*
    pub fn set_cross_origin(this: &HtmlLinkElement, value: Option<&str>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLLinkElement" , js_name = rel ) ]
    ///Getter for the `rel` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/rel)
    ///
    ///*This API requires the following crate features to be activated: `HtmlLinkElement`*
    pub fn rel(this: &HtmlLinkElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLLinkElement" , js_name = rel ) ]
    ///Setter for the `rel` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/rel)
    ///
    ///*This API requires the following crate features to be activated: `HtmlLinkElement`*
    pub fn set_rel(this: &HtmlLinkElement, value: &str);

    #[cfg(feature = "DomTokenList")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLLinkElement" , js_name = relList ) ]
    ///Getter for the `relList` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/relList)
    ///
    ///*This API requires the following crate features to be activated: `DomTokenList`, `HtmlLinkElement`*
    pub fn rel_list(this: &HtmlLinkElement) -> DomTokenList;

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLLinkElement" , js_name = media ) ]
    ///Getter for the `media` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/media)
    ///
    ///*This API requires the following crate features to be activated: `HtmlLinkElement`*
    pub fn media(this: &HtmlLinkElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLLinkElement" , js_name = media ) ]
    ///Setter for the `media` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/media)
    ///
    ///*This API requires the following crate features to be activated: `HtmlLinkElement`*
    pub fn set_media(this: &HtmlLinkElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLLinkElement" , js_name = hreflang ) ]
    ///Getter for the `hreflang` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/hreflang)
    ///
    ///*This API requires the following crate features to be activated: `HtmlLinkElement`*
    pub fn hreflang(this: &HtmlLinkElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLLinkElement" , js_name = hreflang ) ]
    ///Setter for the `hreflang` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/hreflang)
    ///
    ///*This API requires the following crate features to be activated: `HtmlLinkElement`*
    pub fn set_hreflang(this: &HtmlLinkElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLLinkElement" , js_name = type ) ]
    ///Getter for the `type` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/type)
    ///
    ///*This API requires the following crate features to be activated: `HtmlLinkElement`*
    pub fn type_(this: &HtmlLinkElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLLinkElement" , js_name = type ) ]
    ///Setter for the `type` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/type)
    ///
    ///*This API requires the following crate features to be activated: `HtmlLinkElement`*
    pub fn set_type(this: &HtmlLinkElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLLinkElement" , js_name = referrerPolicy ) ]
    ///Getter for the `referrerPolicy` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/referrerPolicy)
    ///
    ///*This API requires the following crate features to be activated: `HtmlLinkElement`*
    pub fn referrer_policy(this: &HtmlLinkElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLLinkElement" , js_name = referrerPolicy ) ]
    ///Setter for the `referrerPolicy` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/referrerPolicy)
    ///
    ///*This API requires the following crate features to be activated: `HtmlLinkElement`*
    pub fn set_referrer_policy(this: &HtmlLinkElement, value: &str);

    #[cfg(feature = "DomTokenList")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLLinkElement" , js_name = sizes ) ]
    ///Getter for the `sizes` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/sizes)
    ///
    ///*This API requires the following crate features to be activated: `DomTokenList`, `HtmlLinkElement`*
    pub fn sizes(this: &HtmlLinkElement) -> DomTokenList;

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLLinkElement" , js_name = charset ) ]
    ///Getter for the `charset` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/charset)
    ///
    ///*This API requires the following crate features to be activated: `HtmlLinkElement`*
    pub fn charset(this: &HtmlLinkElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLLinkElement" , js_name = charset ) ]
    ///Setter for the `charset` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/charset)
    ///
    ///*This API requires the following crate features to be activated: `HtmlLinkElement`*
    pub fn set_charset(this: &HtmlLinkElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLLinkElement" , js_name = rev ) ]
    ///Getter for the `rev` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/rev)
    ///
    ///*This API requires the following crate features to be activated: `HtmlLinkElement`*
    pub fn rev(this: &HtmlLinkElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLLinkElement" , js_name = rev ) ]
    ///Setter for the `rev` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/rev)
    ///
    ///*This API requires the following crate features to be activated: `HtmlLinkElement`*
    pub fn set_rev(this: &HtmlLinkElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLLinkElement" , js_name = target ) ]
    ///Getter for the `target` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/target)
    ///
    ///*This API requires the following crate features to be activated: `HtmlLinkElement`*
    pub fn target(this: &HtmlLinkElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLLinkElement" , js_name = target ) ]
    ///Setter for the `target` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/target)
    ///
    ///*This API requires the following crate features to be activated: `HtmlLinkElement`*
    pub fn set_target(this: &HtmlLinkElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLLinkElement" , js_name = integrity ) ]
    ///Getter for the `integrity` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/integrity)
    ///
    ///*This API requires the following crate features to be activated: `HtmlLinkElement`*
    pub fn integrity(this: &HtmlLinkElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLLinkElement" , js_name = integrity ) ]
    ///Setter for the `integrity` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/integrity)
    ///
    ///*This API requires the following crate features to be activated: `HtmlLinkElement`*
    pub fn set_integrity(this: &HtmlLinkElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLLinkElement" , js_name = as ) ]
    ///Getter for the `as` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/as)
    ///
    ///*This API requires the following crate features to be activated: `HtmlLinkElement`*
    pub fn as_(this: &HtmlLinkElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLLinkElement" , js_name = as ) ]
    ///Setter for the `as` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/as)
    ///
    ///*This API requires the following crate features to be activated: `HtmlLinkElement`*
    pub fn set_as(this: &HtmlLinkElement, value: &str);

    #[cfg(feature = "StyleSheet")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLLinkElement" , js_name = sheet ) ]
    ///Getter for the `sheet` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/sheet)
    ///
    ///*This API requires the following crate features to be activated: `HtmlLinkElement`, `StyleSheet`*
    pub fn sheet(this: &HtmlLinkElement) -> Option<StyleSheet>;

}
