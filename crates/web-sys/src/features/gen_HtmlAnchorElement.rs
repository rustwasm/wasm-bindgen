use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = HtmlElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = HTMLAnchorElement , typescript_name = HTMLAnchorElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlAnchorElement` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement)\n\n*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
    pub type HtmlAnchorElement;
    # [ wasm_bindgen ( structural , method , getter , js_name = target ) ]
    #[doc = "Getter for the `target` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/target)\n\n*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
    pub fn target(this: &HtmlAnchorElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = target ) ]
    #[doc = "Setter for the `target` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/target)\n\n*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
    pub fn set_target(this: &HtmlAnchorElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_name = download ) ]
    #[doc = "Getter for the `download` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/download)\n\n*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
    pub fn download(this: &HtmlAnchorElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = download ) ]
    #[doc = "Setter for the `download` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/download)\n\n*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
    pub fn set_download(this: &HtmlAnchorElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_name = ping ) ]
    #[doc = "Getter for the `ping` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/ping)\n\n*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
    pub fn ping(this: &HtmlAnchorElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = ping ) ]
    #[doc = "Setter for the `ping` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/ping)\n\n*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
    pub fn set_ping(this: &HtmlAnchorElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_name = rel ) ]
    #[doc = "Getter for the `rel` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/rel)\n\n*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
    pub fn rel(this: &HtmlAnchorElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = rel ) ]
    #[doc = "Setter for the `rel` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/rel)\n\n*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
    pub fn set_rel(this: &HtmlAnchorElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_name = referrerPolicy ) ]
    #[doc = "Getter for the `referrerPolicy` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/referrerPolicy)\n\n*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
    pub fn referrer_policy(this: &HtmlAnchorElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = referrerPolicy ) ]
    #[doc = "Setter for the `referrerPolicy` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/referrerPolicy)\n\n*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
    pub fn set_referrer_policy(this: &HtmlAnchorElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_name = relList ) ]
    #[cfg(feature = "DomTokenList")]
    #[doc = "Getter for the `relList` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/relList)\n\n*This API requires the following crate features to be activated: `DomTokenList`, `HtmlAnchorElement`*"]
    pub fn rel_list(this: &HtmlAnchorElement) -> DomTokenList;
    # [ wasm_bindgen ( structural , method , getter , js_name = hreflang ) ]
    #[doc = "Getter for the `hreflang` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/hreflang)\n\n*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
    pub fn hreflang(this: &HtmlAnchorElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = hreflang ) ]
    #[doc = "Setter for the `hreflang` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/hreflang)\n\n*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
    pub fn set_hreflang(this: &HtmlAnchorElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_name = type ) ]
    #[doc = "Getter for the `type` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/type)\n\n*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
    pub fn type_(this: &HtmlAnchorElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = type ) ]
    #[doc = "Setter for the `type` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/type)\n\n*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
    pub fn set_type(this: &HtmlAnchorElement, value: &str);
    # [ wasm_bindgen ( structural , catch , method , getter , js_name = text ) ]
    #[doc = "Getter for the `text` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/text)\n\n*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
    pub fn text(this: &HtmlAnchorElement) -> Result<String, JsValue>;
    # [ wasm_bindgen ( structural , catch , method , setter , js_name = text ) ]
    #[doc = "Setter for the `text` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/text)\n\n*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
    pub fn set_text(this: &HtmlAnchorElement, value: &str) -> Result<(), JsValue>;
    # [ wasm_bindgen ( structural , method , getter , js_name = coords ) ]
    #[doc = "Getter for the `coords` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/coords)\n\n*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
    pub fn coords(this: &HtmlAnchorElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = coords ) ]
    #[doc = "Setter for the `coords` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/coords)\n\n*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
    pub fn set_coords(this: &HtmlAnchorElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_name = charset ) ]
    #[doc = "Getter for the `charset` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/charset)\n\n*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
    pub fn charset(this: &HtmlAnchorElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = charset ) ]
    #[doc = "Setter for the `charset` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/charset)\n\n*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
    pub fn set_charset(this: &HtmlAnchorElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_name = name ) ]
    #[doc = "Getter for the `name` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/name)\n\n*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
    pub fn name(this: &HtmlAnchorElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = name ) ]
    #[doc = "Setter for the `name` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/name)\n\n*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
    pub fn set_name(this: &HtmlAnchorElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_name = rev ) ]
    #[doc = "Getter for the `rev` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/rev)\n\n*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
    pub fn rev(this: &HtmlAnchorElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = rev ) ]
    #[doc = "Setter for the `rev` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/rev)\n\n*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
    pub fn set_rev(this: &HtmlAnchorElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_name = shape ) ]
    #[doc = "Getter for the `shape` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/shape)\n\n*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
    pub fn shape(this: &HtmlAnchorElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = shape ) ]
    #[doc = "Setter for the `shape` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/shape)\n\n*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
    pub fn set_shape(this: &HtmlAnchorElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_name = href ) ]
    #[doc = "Getter for the `href` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/href)\n\n*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
    pub fn href(this: &HtmlAnchorElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = href ) ]
    #[doc = "Setter for the `href` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/href)\n\n*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
    pub fn set_href(this: &HtmlAnchorElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_name = origin ) ]
    #[doc = "Getter for the `origin` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/origin)\n\n*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
    pub fn origin(this: &HtmlAnchorElement) -> String;
    # [ wasm_bindgen ( structural , method , getter , js_name = protocol ) ]
    #[doc = "Getter for the `protocol` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/protocol)\n\n*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
    pub fn protocol(this: &HtmlAnchorElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = protocol ) ]
    #[doc = "Setter for the `protocol` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/protocol)\n\n*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
    pub fn set_protocol(this: &HtmlAnchorElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_name = username ) ]
    #[doc = "Getter for the `username` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/username)\n\n*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
    pub fn username(this: &HtmlAnchorElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = username ) ]
    #[doc = "Setter for the `username` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/username)\n\n*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
    pub fn set_username(this: &HtmlAnchorElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_name = password ) ]
    #[doc = "Getter for the `password` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/password)\n\n*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
    pub fn password(this: &HtmlAnchorElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = password ) ]
    #[doc = "Setter for the `password` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/password)\n\n*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
    pub fn set_password(this: &HtmlAnchorElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_name = host ) ]
    #[doc = "Getter for the `host` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/host)\n\n*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
    pub fn host(this: &HtmlAnchorElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = host ) ]
    #[doc = "Setter for the `host` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/host)\n\n*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
    pub fn set_host(this: &HtmlAnchorElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_name = hostname ) ]
    #[doc = "Getter for the `hostname` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/hostname)\n\n*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
    pub fn hostname(this: &HtmlAnchorElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = hostname ) ]
    #[doc = "Setter for the `hostname` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/hostname)\n\n*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
    pub fn set_hostname(this: &HtmlAnchorElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_name = port ) ]
    #[doc = "Getter for the `port` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/port)\n\n*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
    pub fn port(this: &HtmlAnchorElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = port ) ]
    #[doc = "Setter for the `port` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/port)\n\n*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
    pub fn set_port(this: &HtmlAnchorElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_name = pathname ) ]
    #[doc = "Getter for the `pathname` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/pathname)\n\n*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
    pub fn pathname(this: &HtmlAnchorElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = pathname ) ]
    #[doc = "Setter for the `pathname` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/pathname)\n\n*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
    pub fn set_pathname(this: &HtmlAnchorElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_name = search ) ]
    #[doc = "Getter for the `search` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/search)\n\n*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
    pub fn search(this: &HtmlAnchorElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = search ) ]
    #[doc = "Setter for the `search` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/search)\n\n*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
    pub fn set_search(this: &HtmlAnchorElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_name = hash ) ]
    #[doc = "Getter for the `hash` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/hash)\n\n*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
    pub fn hash(this: &HtmlAnchorElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = hash ) ]
    #[doc = "Setter for the `hash` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/hash)\n\n*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
    pub fn set_hash(this: &HtmlAnchorElement, value: &str);
}
