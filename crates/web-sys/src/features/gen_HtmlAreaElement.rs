use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = HtmlElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = HTMLAreaElement , typescript_name = HTMLAreaElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlAreaElement` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement)\n\n*This API requires the following crate features to be activated: `HtmlAreaElement`*"]
    pub type HtmlAreaElement;
    # [ wasm_bindgen ( structural , method , getter , js_name = alt ) ]
    #[doc = "Getter for the `alt` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement/alt)\n\n*This API requires the following crate features to be activated: `HtmlAreaElement`*"]
    pub fn alt(this: &HtmlAreaElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = alt ) ]
    #[doc = "Setter for the `alt` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement/alt)\n\n*This API requires the following crate features to be activated: `HtmlAreaElement`*"]
    pub fn set_alt(this: &HtmlAreaElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_name = coords ) ]
    #[doc = "Getter for the `coords` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement/coords)\n\n*This API requires the following crate features to be activated: `HtmlAreaElement`*"]
    pub fn coords(this: &HtmlAreaElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = coords ) ]
    #[doc = "Setter for the `coords` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement/coords)\n\n*This API requires the following crate features to be activated: `HtmlAreaElement`*"]
    pub fn set_coords(this: &HtmlAreaElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_name = shape ) ]
    #[doc = "Getter for the `shape` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement/shape)\n\n*This API requires the following crate features to be activated: `HtmlAreaElement`*"]
    pub fn shape(this: &HtmlAreaElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = shape ) ]
    #[doc = "Setter for the `shape` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement/shape)\n\n*This API requires the following crate features to be activated: `HtmlAreaElement`*"]
    pub fn set_shape(this: &HtmlAreaElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_name = target ) ]
    #[doc = "Getter for the `target` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement/target)\n\n*This API requires the following crate features to be activated: `HtmlAreaElement`*"]
    pub fn target(this: &HtmlAreaElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = target ) ]
    #[doc = "Setter for the `target` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement/target)\n\n*This API requires the following crate features to be activated: `HtmlAreaElement`*"]
    pub fn set_target(this: &HtmlAreaElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_name = download ) ]
    #[doc = "Getter for the `download` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement/download)\n\n*This API requires the following crate features to be activated: `HtmlAreaElement`*"]
    pub fn download(this: &HtmlAreaElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = download ) ]
    #[doc = "Setter for the `download` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement/download)\n\n*This API requires the following crate features to be activated: `HtmlAreaElement`*"]
    pub fn set_download(this: &HtmlAreaElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_name = ping ) ]
    #[doc = "Getter for the `ping` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement/ping)\n\n*This API requires the following crate features to be activated: `HtmlAreaElement`*"]
    pub fn ping(this: &HtmlAreaElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = ping ) ]
    #[doc = "Setter for the `ping` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement/ping)\n\n*This API requires the following crate features to be activated: `HtmlAreaElement`*"]
    pub fn set_ping(this: &HtmlAreaElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_name = rel ) ]
    #[doc = "Getter for the `rel` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement/rel)\n\n*This API requires the following crate features to be activated: `HtmlAreaElement`*"]
    pub fn rel(this: &HtmlAreaElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = rel ) ]
    #[doc = "Setter for the `rel` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement/rel)\n\n*This API requires the following crate features to be activated: `HtmlAreaElement`*"]
    pub fn set_rel(this: &HtmlAreaElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_name = referrerPolicy ) ]
    #[doc = "Getter for the `referrerPolicy` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement/referrerPolicy)\n\n*This API requires the following crate features to be activated: `HtmlAreaElement`*"]
    pub fn referrer_policy(this: &HtmlAreaElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = referrerPolicy ) ]
    #[doc = "Setter for the `referrerPolicy` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement/referrerPolicy)\n\n*This API requires the following crate features to be activated: `HtmlAreaElement`*"]
    pub fn set_referrer_policy(this: &HtmlAreaElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_name = relList ) ]
    #[cfg(feature = "DomTokenList")]
    #[doc = "Getter for the `relList` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement/relList)\n\n*This API requires the following crate features to be activated: `DomTokenList`, `HtmlAreaElement`*"]
    pub fn rel_list(this: &HtmlAreaElement) -> DomTokenList;
    # [ wasm_bindgen ( structural , method , getter , js_name = noHref ) ]
    #[doc = "Getter for the `noHref` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement/noHref)\n\n*This API requires the following crate features to be activated: `HtmlAreaElement`*"]
    pub fn no_href(this: &HtmlAreaElement) -> bool;
    # [ wasm_bindgen ( structural , method , setter , js_name = noHref ) ]
    #[doc = "Setter for the `noHref` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement/noHref)\n\n*This API requires the following crate features to be activated: `HtmlAreaElement`*"]
    pub fn set_no_href(this: &HtmlAreaElement, value: bool);
    # [ wasm_bindgen ( structural , method , getter , js_name = href ) ]
    #[doc = "Getter for the `href` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement/href)\n\n*This API requires the following crate features to be activated: `HtmlAreaElement`*"]
    pub fn href(this: &HtmlAreaElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = href ) ]
    #[doc = "Setter for the `href` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement/href)\n\n*This API requires the following crate features to be activated: `HtmlAreaElement`*"]
    pub fn set_href(this: &HtmlAreaElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_name = origin ) ]
    #[doc = "Getter for the `origin` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement/origin)\n\n*This API requires the following crate features to be activated: `HtmlAreaElement`*"]
    pub fn origin(this: &HtmlAreaElement) -> String;
    # [ wasm_bindgen ( structural , method , getter , js_name = protocol ) ]
    #[doc = "Getter for the `protocol` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement/protocol)\n\n*This API requires the following crate features to be activated: `HtmlAreaElement`*"]
    pub fn protocol(this: &HtmlAreaElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = protocol ) ]
    #[doc = "Setter for the `protocol` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement/protocol)\n\n*This API requires the following crate features to be activated: `HtmlAreaElement`*"]
    pub fn set_protocol(this: &HtmlAreaElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_name = username ) ]
    #[doc = "Getter for the `username` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement/username)\n\n*This API requires the following crate features to be activated: `HtmlAreaElement`*"]
    pub fn username(this: &HtmlAreaElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = username ) ]
    #[doc = "Setter for the `username` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement/username)\n\n*This API requires the following crate features to be activated: `HtmlAreaElement`*"]
    pub fn set_username(this: &HtmlAreaElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_name = password ) ]
    #[doc = "Getter for the `password` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement/password)\n\n*This API requires the following crate features to be activated: `HtmlAreaElement`*"]
    pub fn password(this: &HtmlAreaElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = password ) ]
    #[doc = "Setter for the `password` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement/password)\n\n*This API requires the following crate features to be activated: `HtmlAreaElement`*"]
    pub fn set_password(this: &HtmlAreaElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_name = host ) ]
    #[doc = "Getter for the `host` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement/host)\n\n*This API requires the following crate features to be activated: `HtmlAreaElement`*"]
    pub fn host(this: &HtmlAreaElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = host ) ]
    #[doc = "Setter for the `host` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement/host)\n\n*This API requires the following crate features to be activated: `HtmlAreaElement`*"]
    pub fn set_host(this: &HtmlAreaElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_name = hostname ) ]
    #[doc = "Getter for the `hostname` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement/hostname)\n\n*This API requires the following crate features to be activated: `HtmlAreaElement`*"]
    pub fn hostname(this: &HtmlAreaElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = hostname ) ]
    #[doc = "Setter for the `hostname` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement/hostname)\n\n*This API requires the following crate features to be activated: `HtmlAreaElement`*"]
    pub fn set_hostname(this: &HtmlAreaElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_name = port ) ]
    #[doc = "Getter for the `port` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement/port)\n\n*This API requires the following crate features to be activated: `HtmlAreaElement`*"]
    pub fn port(this: &HtmlAreaElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = port ) ]
    #[doc = "Setter for the `port` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement/port)\n\n*This API requires the following crate features to be activated: `HtmlAreaElement`*"]
    pub fn set_port(this: &HtmlAreaElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_name = pathname ) ]
    #[doc = "Getter for the `pathname` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement/pathname)\n\n*This API requires the following crate features to be activated: `HtmlAreaElement`*"]
    pub fn pathname(this: &HtmlAreaElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = pathname ) ]
    #[doc = "Setter for the `pathname` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement/pathname)\n\n*This API requires the following crate features to be activated: `HtmlAreaElement`*"]
    pub fn set_pathname(this: &HtmlAreaElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_name = search ) ]
    #[doc = "Getter for the `search` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement/search)\n\n*This API requires the following crate features to be activated: `HtmlAreaElement`*"]
    pub fn search(this: &HtmlAreaElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = search ) ]
    #[doc = "Setter for the `search` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement/search)\n\n*This API requires the following crate features to be activated: `HtmlAreaElement`*"]
    pub fn set_search(this: &HtmlAreaElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_name = hash ) ]
    #[doc = "Getter for the `hash` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement/hash)\n\n*This API requires the following crate features to be activated: `HtmlAreaElement`*"]
    pub fn hash(this: &HtmlAreaElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = hash ) ]
    #[doc = "Setter for the `hash` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement/hash)\n\n*This API requires the following crate features to be activated: `HtmlAreaElement`*"]
    pub fn set_hash(this: &HtmlAreaElement, value: &str);
}
