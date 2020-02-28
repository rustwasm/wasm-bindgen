use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = HtmlElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = HTMLLinkElement , typescript_name = HTMLLinkElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlLinkElement` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement)\n\n*This API requires the following crate features to be activated: `HtmlLinkElement`*"]
    pub type HtmlLinkElement;
    # [ wasm_bindgen ( structural , method , getter , js_name = disabled ) ]
    #[doc = "Getter for the `disabled` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/disabled)\n\n*This API requires the following crate features to be activated: `HtmlLinkElement`*"]
    pub fn disabled(this: &HtmlLinkElement) -> bool;
    # [ wasm_bindgen ( structural , method , setter , js_name = disabled ) ]
    #[doc = "Setter for the `disabled` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/disabled)\n\n*This API requires the following crate features to be activated: `HtmlLinkElement`*"]
    pub fn set_disabled(this: &HtmlLinkElement, value: bool);
    # [ wasm_bindgen ( structural , method , getter , js_name = href ) ]
    #[doc = "Getter for the `href` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/href)\n\n*This API requires the following crate features to be activated: `HtmlLinkElement`*"]
    pub fn href(this: &HtmlLinkElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = href ) ]
    #[doc = "Setter for the `href` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/href)\n\n*This API requires the following crate features to be activated: `HtmlLinkElement`*"]
    pub fn set_href(this: &HtmlLinkElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_name = crossOrigin ) ]
    #[doc = "Getter for the `crossOrigin` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/crossOrigin)\n\n*This API requires the following crate features to be activated: `HtmlLinkElement`*"]
    pub fn cross_origin(this: &HtmlLinkElement) -> Option<String>;
    # [ wasm_bindgen ( structural , method , setter , js_name = crossOrigin ) ]
    #[doc = "Setter for the `crossOrigin` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/crossOrigin)\n\n*This API requires the following crate features to be activated: `HtmlLinkElement`*"]
    pub fn set_cross_origin(this: &HtmlLinkElement, value: Option<&str>);
    # [ wasm_bindgen ( structural , method , getter , js_name = rel ) ]
    #[doc = "Getter for the `rel` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/rel)\n\n*This API requires the following crate features to be activated: `HtmlLinkElement`*"]
    pub fn rel(this: &HtmlLinkElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = rel ) ]
    #[doc = "Setter for the `rel` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/rel)\n\n*This API requires the following crate features to be activated: `HtmlLinkElement`*"]
    pub fn set_rel(this: &HtmlLinkElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_name = relList ) ]
    #[cfg(feature = "DomTokenList")]
    #[doc = "Getter for the `relList` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/relList)\n\n*This API requires the following crate features to be activated: `DomTokenList`, `HtmlLinkElement`*"]
    pub fn rel_list(this: &HtmlLinkElement) -> DomTokenList;
    # [ wasm_bindgen ( structural , method , getter , js_name = media ) ]
    #[doc = "Getter for the `media` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/media)\n\n*This API requires the following crate features to be activated: `HtmlLinkElement`*"]
    pub fn media(this: &HtmlLinkElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = media ) ]
    #[doc = "Setter for the `media` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/media)\n\n*This API requires the following crate features to be activated: `HtmlLinkElement`*"]
    pub fn set_media(this: &HtmlLinkElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_name = hreflang ) ]
    #[doc = "Getter for the `hreflang` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/hreflang)\n\n*This API requires the following crate features to be activated: `HtmlLinkElement`*"]
    pub fn hreflang(this: &HtmlLinkElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = hreflang ) ]
    #[doc = "Setter for the `hreflang` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/hreflang)\n\n*This API requires the following crate features to be activated: `HtmlLinkElement`*"]
    pub fn set_hreflang(this: &HtmlLinkElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_name = type ) ]
    #[doc = "Getter for the `type` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/type)\n\n*This API requires the following crate features to be activated: `HtmlLinkElement`*"]
    pub fn type_(this: &HtmlLinkElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = type ) ]
    #[doc = "Setter for the `type` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/type)\n\n*This API requires the following crate features to be activated: `HtmlLinkElement`*"]
    pub fn set_type(this: &HtmlLinkElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_name = referrerPolicy ) ]
    #[doc = "Getter for the `referrerPolicy` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/referrerPolicy)\n\n*This API requires the following crate features to be activated: `HtmlLinkElement`*"]
    pub fn referrer_policy(this: &HtmlLinkElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = referrerPolicy ) ]
    #[doc = "Setter for the `referrerPolicy` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/referrerPolicy)\n\n*This API requires the following crate features to be activated: `HtmlLinkElement`*"]
    pub fn set_referrer_policy(this: &HtmlLinkElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_name = sizes ) ]
    #[cfg(feature = "DomTokenList")]
    #[doc = "Getter for the `sizes` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/sizes)\n\n*This API requires the following crate features to be activated: `DomTokenList`, `HtmlLinkElement`*"]
    pub fn sizes(this: &HtmlLinkElement) -> DomTokenList;
    # [ wasm_bindgen ( structural , method , getter , js_name = charset ) ]
    #[doc = "Getter for the `charset` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/charset)\n\n*This API requires the following crate features to be activated: `HtmlLinkElement`*"]
    pub fn charset(this: &HtmlLinkElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = charset ) ]
    #[doc = "Setter for the `charset` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/charset)\n\n*This API requires the following crate features to be activated: `HtmlLinkElement`*"]
    pub fn set_charset(this: &HtmlLinkElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_name = rev ) ]
    #[doc = "Getter for the `rev` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/rev)\n\n*This API requires the following crate features to be activated: `HtmlLinkElement`*"]
    pub fn rev(this: &HtmlLinkElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = rev ) ]
    #[doc = "Setter for the `rev` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/rev)\n\n*This API requires the following crate features to be activated: `HtmlLinkElement`*"]
    pub fn set_rev(this: &HtmlLinkElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_name = target ) ]
    #[doc = "Getter for the `target` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/target)\n\n*This API requires the following crate features to be activated: `HtmlLinkElement`*"]
    pub fn target(this: &HtmlLinkElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = target ) ]
    #[doc = "Setter for the `target` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/target)\n\n*This API requires the following crate features to be activated: `HtmlLinkElement`*"]
    pub fn set_target(this: &HtmlLinkElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_name = integrity ) ]
    #[doc = "Getter for the `integrity` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/integrity)\n\n*This API requires the following crate features to be activated: `HtmlLinkElement`*"]
    pub fn integrity(this: &HtmlLinkElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = integrity ) ]
    #[doc = "Setter for the `integrity` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/integrity)\n\n*This API requires the following crate features to be activated: `HtmlLinkElement`*"]
    pub fn set_integrity(this: &HtmlLinkElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_name = as ) ]
    #[doc = "Getter for the `as` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/as)\n\n*This API requires the following crate features to be activated: `HtmlLinkElement`*"]
    pub fn as_(this: &HtmlLinkElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = as ) ]
    #[doc = "Setter for the `as` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/as)\n\n*This API requires the following crate features to be activated: `HtmlLinkElement`*"]
    pub fn set_as(this: &HtmlLinkElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_name = sheet ) ]
    #[cfg(feature = "StyleSheet")]
    #[doc = "Getter for the `sheet` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/sheet)\n\n*This API requires the following crate features to be activated: `HtmlLinkElement`, `StyleSheet`*"]
    pub fn sheet(this: &HtmlLinkElement) -> Option<StyleSheet>;
}
