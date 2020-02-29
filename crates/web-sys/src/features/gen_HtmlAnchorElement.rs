use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = HtmlElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = HTMLAnchorElement , typescript_name = HTMLAnchorElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `HtmlAnchorElement` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement)
    ///
    ///*This API requires the following crate features to be activated: `HtmlAnchorElement`*
    pub type HtmlAnchorElement;

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLAnchorElement" , js_name = target ) ]
    ///Getter for the `target` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/target)
    ///
    ///*This API requires the following crate features to be activated: `HtmlAnchorElement`*
    pub fn target(this: &HtmlAnchorElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLAnchorElement" , js_name = target ) ]
    ///Setter for the `target` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/target)
    ///
    ///*This API requires the following crate features to be activated: `HtmlAnchorElement`*
    pub fn set_target(this: &HtmlAnchorElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLAnchorElement" , js_name = download ) ]
    ///Getter for the `download` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/download)
    ///
    ///*This API requires the following crate features to be activated: `HtmlAnchorElement`*
    pub fn download(this: &HtmlAnchorElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLAnchorElement" , js_name = download ) ]
    ///Setter for the `download` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/download)
    ///
    ///*This API requires the following crate features to be activated: `HtmlAnchorElement`*
    pub fn set_download(this: &HtmlAnchorElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLAnchorElement" , js_name = ping ) ]
    ///Getter for the `ping` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/ping)
    ///
    ///*This API requires the following crate features to be activated: `HtmlAnchorElement`*
    pub fn ping(this: &HtmlAnchorElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLAnchorElement" , js_name = ping ) ]
    ///Setter for the `ping` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/ping)
    ///
    ///*This API requires the following crate features to be activated: `HtmlAnchorElement`*
    pub fn set_ping(this: &HtmlAnchorElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLAnchorElement" , js_name = rel ) ]
    ///Getter for the `rel` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/rel)
    ///
    ///*This API requires the following crate features to be activated: `HtmlAnchorElement`*
    pub fn rel(this: &HtmlAnchorElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLAnchorElement" , js_name = rel ) ]
    ///Setter for the `rel` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/rel)
    ///
    ///*This API requires the following crate features to be activated: `HtmlAnchorElement`*
    pub fn set_rel(this: &HtmlAnchorElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLAnchorElement" , js_name = referrerPolicy ) ]
    ///Getter for the `referrerPolicy` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/referrerPolicy)
    ///
    ///*This API requires the following crate features to be activated: `HtmlAnchorElement`*
    pub fn referrer_policy(this: &HtmlAnchorElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLAnchorElement" , js_name = referrerPolicy ) ]
    ///Setter for the `referrerPolicy` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/referrerPolicy)
    ///
    ///*This API requires the following crate features to be activated: `HtmlAnchorElement`*
    pub fn set_referrer_policy(this: &HtmlAnchorElement, value: &str);

    #[cfg(feature = "DomTokenList")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLAnchorElement" , js_name = relList ) ]
    ///Getter for the `relList` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/relList)
    ///
    ///*This API requires the following crate features to be activated: `DomTokenList`, `HtmlAnchorElement`*
    pub fn rel_list(this: &HtmlAnchorElement) -> DomTokenList;

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLAnchorElement" , js_name = hreflang ) ]
    ///Getter for the `hreflang` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/hreflang)
    ///
    ///*This API requires the following crate features to be activated: `HtmlAnchorElement`*
    pub fn hreflang(this: &HtmlAnchorElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLAnchorElement" , js_name = hreflang ) ]
    ///Setter for the `hreflang` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/hreflang)
    ///
    ///*This API requires the following crate features to be activated: `HtmlAnchorElement`*
    pub fn set_hreflang(this: &HtmlAnchorElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLAnchorElement" , js_name = type ) ]
    ///Getter for the `type` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/type)
    ///
    ///*This API requires the following crate features to be activated: `HtmlAnchorElement`*
    pub fn type_(this: &HtmlAnchorElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLAnchorElement" , js_name = type ) ]
    ///Setter for the `type` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/type)
    ///
    ///*This API requires the following crate features to be activated: `HtmlAnchorElement`*
    pub fn set_type(this: &HtmlAnchorElement, value: &str);

    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "HTMLAnchorElement" , js_name = text ) ]
    ///Getter for the `text` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/text)
    ///
    ///*This API requires the following crate features to be activated: `HtmlAnchorElement`*
    pub fn text(this: &HtmlAnchorElement) -> Result<String, JsValue>;

    # [ wasm_bindgen ( structural , catch , method , setter , js_class = "HTMLAnchorElement" , js_name = text ) ]
    ///Setter for the `text` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/text)
    ///
    ///*This API requires the following crate features to be activated: `HtmlAnchorElement`*
    pub fn set_text(this: &HtmlAnchorElement, value: &str) -> Result<(), JsValue>;

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLAnchorElement" , js_name = coords ) ]
    ///Getter for the `coords` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/coords)
    ///
    ///*This API requires the following crate features to be activated: `HtmlAnchorElement`*
    pub fn coords(this: &HtmlAnchorElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLAnchorElement" , js_name = coords ) ]
    ///Setter for the `coords` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/coords)
    ///
    ///*This API requires the following crate features to be activated: `HtmlAnchorElement`*
    pub fn set_coords(this: &HtmlAnchorElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLAnchorElement" , js_name = charset ) ]
    ///Getter for the `charset` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/charset)
    ///
    ///*This API requires the following crate features to be activated: `HtmlAnchorElement`*
    pub fn charset(this: &HtmlAnchorElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLAnchorElement" , js_name = charset ) ]
    ///Setter for the `charset` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/charset)
    ///
    ///*This API requires the following crate features to be activated: `HtmlAnchorElement`*
    pub fn set_charset(this: &HtmlAnchorElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLAnchorElement" , js_name = name ) ]
    ///Getter for the `name` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/name)
    ///
    ///*This API requires the following crate features to be activated: `HtmlAnchorElement`*
    pub fn name(this: &HtmlAnchorElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLAnchorElement" , js_name = name ) ]
    ///Setter for the `name` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/name)
    ///
    ///*This API requires the following crate features to be activated: `HtmlAnchorElement`*
    pub fn set_name(this: &HtmlAnchorElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLAnchorElement" , js_name = rev ) ]
    ///Getter for the `rev` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/rev)
    ///
    ///*This API requires the following crate features to be activated: `HtmlAnchorElement`*
    pub fn rev(this: &HtmlAnchorElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLAnchorElement" , js_name = rev ) ]
    ///Setter for the `rev` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/rev)
    ///
    ///*This API requires the following crate features to be activated: `HtmlAnchorElement`*
    pub fn set_rev(this: &HtmlAnchorElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLAnchorElement" , js_name = shape ) ]
    ///Getter for the `shape` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/shape)
    ///
    ///*This API requires the following crate features to be activated: `HtmlAnchorElement`*
    pub fn shape(this: &HtmlAnchorElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLAnchorElement" , js_name = shape ) ]
    ///Setter for the `shape` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/shape)
    ///
    ///*This API requires the following crate features to be activated: `HtmlAnchorElement`*
    pub fn set_shape(this: &HtmlAnchorElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLAnchorElement" , js_name = href ) ]
    ///Getter for the `href` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/href)
    ///
    ///*This API requires the following crate features to be activated: `HtmlAnchorElement`*
    pub fn href(this: &HtmlAnchorElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLAnchorElement" , js_name = href ) ]
    ///Setter for the `href` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/href)
    ///
    ///*This API requires the following crate features to be activated: `HtmlAnchorElement`*
    pub fn set_href(this: &HtmlAnchorElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLAnchorElement" , js_name = origin ) ]
    ///Getter for the `origin` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/origin)
    ///
    ///*This API requires the following crate features to be activated: `HtmlAnchorElement`*
    pub fn origin(this: &HtmlAnchorElement) -> String;

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLAnchorElement" , js_name = protocol ) ]
    ///Getter for the `protocol` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/protocol)
    ///
    ///*This API requires the following crate features to be activated: `HtmlAnchorElement`*
    pub fn protocol(this: &HtmlAnchorElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLAnchorElement" , js_name = protocol ) ]
    ///Setter for the `protocol` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/protocol)
    ///
    ///*This API requires the following crate features to be activated: `HtmlAnchorElement`*
    pub fn set_protocol(this: &HtmlAnchorElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLAnchorElement" , js_name = username ) ]
    ///Getter for the `username` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/username)
    ///
    ///*This API requires the following crate features to be activated: `HtmlAnchorElement`*
    pub fn username(this: &HtmlAnchorElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLAnchorElement" , js_name = username ) ]
    ///Setter for the `username` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/username)
    ///
    ///*This API requires the following crate features to be activated: `HtmlAnchorElement`*
    pub fn set_username(this: &HtmlAnchorElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLAnchorElement" , js_name = password ) ]
    ///Getter for the `password` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/password)
    ///
    ///*This API requires the following crate features to be activated: `HtmlAnchorElement`*
    pub fn password(this: &HtmlAnchorElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLAnchorElement" , js_name = password ) ]
    ///Setter for the `password` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/password)
    ///
    ///*This API requires the following crate features to be activated: `HtmlAnchorElement`*
    pub fn set_password(this: &HtmlAnchorElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLAnchorElement" , js_name = host ) ]
    ///Getter for the `host` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/host)
    ///
    ///*This API requires the following crate features to be activated: `HtmlAnchorElement`*
    pub fn host(this: &HtmlAnchorElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLAnchorElement" , js_name = host ) ]
    ///Setter for the `host` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/host)
    ///
    ///*This API requires the following crate features to be activated: `HtmlAnchorElement`*
    pub fn set_host(this: &HtmlAnchorElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLAnchorElement" , js_name = hostname ) ]
    ///Getter for the `hostname` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/hostname)
    ///
    ///*This API requires the following crate features to be activated: `HtmlAnchorElement`*
    pub fn hostname(this: &HtmlAnchorElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLAnchorElement" , js_name = hostname ) ]
    ///Setter for the `hostname` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/hostname)
    ///
    ///*This API requires the following crate features to be activated: `HtmlAnchorElement`*
    pub fn set_hostname(this: &HtmlAnchorElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLAnchorElement" , js_name = port ) ]
    ///Getter for the `port` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/port)
    ///
    ///*This API requires the following crate features to be activated: `HtmlAnchorElement`*
    pub fn port(this: &HtmlAnchorElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLAnchorElement" , js_name = port ) ]
    ///Setter for the `port` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/port)
    ///
    ///*This API requires the following crate features to be activated: `HtmlAnchorElement`*
    pub fn set_port(this: &HtmlAnchorElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLAnchorElement" , js_name = pathname ) ]
    ///Getter for the `pathname` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/pathname)
    ///
    ///*This API requires the following crate features to be activated: `HtmlAnchorElement`*
    pub fn pathname(this: &HtmlAnchorElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLAnchorElement" , js_name = pathname ) ]
    ///Setter for the `pathname` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/pathname)
    ///
    ///*This API requires the following crate features to be activated: `HtmlAnchorElement`*
    pub fn set_pathname(this: &HtmlAnchorElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLAnchorElement" , js_name = search ) ]
    ///Getter for the `search` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/search)
    ///
    ///*This API requires the following crate features to be activated: `HtmlAnchorElement`*
    pub fn search(this: &HtmlAnchorElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLAnchorElement" , js_name = search ) ]
    ///Setter for the `search` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/search)
    ///
    ///*This API requires the following crate features to be activated: `HtmlAnchorElement`*
    pub fn set_search(this: &HtmlAnchorElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLAnchorElement" , js_name = hash ) ]
    ///Getter for the `hash` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/hash)
    ///
    ///*This API requires the following crate features to be activated: `HtmlAnchorElement`*
    pub fn hash(this: &HtmlAnchorElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLAnchorElement" , js_name = hash ) ]
    ///Setter for the `hash` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/hash)
    ///
    ///*This API requires the following crate features to be activated: `HtmlAnchorElement`*
    pub fn set_hash(this: &HtmlAnchorElement, value: &str);

}
