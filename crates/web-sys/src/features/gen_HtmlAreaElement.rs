use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = HtmlElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = HTMLAreaElement , typescript_type = "HTMLAreaElement" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `HtmlAreaElement` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement)
    ///
    ///*This API requires the following crate features to be activated: `HtmlAreaElement`*
    pub type HtmlAreaElement;

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLAreaElement" , js_name = alt ) ]
    ///Getter for the `alt` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement/alt)
    ///
    ///*This API requires the following crate features to be activated: `HtmlAreaElement`*
    pub fn alt(this: &HtmlAreaElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLAreaElement" , js_name = alt ) ]
    ///Setter for the `alt` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement/alt)
    ///
    ///*This API requires the following crate features to be activated: `HtmlAreaElement`*
    pub fn set_alt(this: &HtmlAreaElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLAreaElement" , js_name = coords ) ]
    ///Getter for the `coords` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement/coords)
    ///
    ///*This API requires the following crate features to be activated: `HtmlAreaElement`*
    pub fn coords(this: &HtmlAreaElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLAreaElement" , js_name = coords ) ]
    ///Setter for the `coords` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement/coords)
    ///
    ///*This API requires the following crate features to be activated: `HtmlAreaElement`*
    pub fn set_coords(this: &HtmlAreaElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLAreaElement" , js_name = shape ) ]
    ///Getter for the `shape` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement/shape)
    ///
    ///*This API requires the following crate features to be activated: `HtmlAreaElement`*
    pub fn shape(this: &HtmlAreaElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLAreaElement" , js_name = shape ) ]
    ///Setter for the `shape` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement/shape)
    ///
    ///*This API requires the following crate features to be activated: `HtmlAreaElement`*
    pub fn set_shape(this: &HtmlAreaElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLAreaElement" , js_name = target ) ]
    ///Getter for the `target` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement/target)
    ///
    ///*This API requires the following crate features to be activated: `HtmlAreaElement`*
    pub fn target(this: &HtmlAreaElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLAreaElement" , js_name = target ) ]
    ///Setter for the `target` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement/target)
    ///
    ///*This API requires the following crate features to be activated: `HtmlAreaElement`*
    pub fn set_target(this: &HtmlAreaElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLAreaElement" , js_name = download ) ]
    ///Getter for the `download` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement/download)
    ///
    ///*This API requires the following crate features to be activated: `HtmlAreaElement`*
    pub fn download(this: &HtmlAreaElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLAreaElement" , js_name = download ) ]
    ///Setter for the `download` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement/download)
    ///
    ///*This API requires the following crate features to be activated: `HtmlAreaElement`*
    pub fn set_download(this: &HtmlAreaElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLAreaElement" , js_name = ping ) ]
    ///Getter for the `ping` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement/ping)
    ///
    ///*This API requires the following crate features to be activated: `HtmlAreaElement`*
    pub fn ping(this: &HtmlAreaElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLAreaElement" , js_name = ping ) ]
    ///Setter for the `ping` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement/ping)
    ///
    ///*This API requires the following crate features to be activated: `HtmlAreaElement`*
    pub fn set_ping(this: &HtmlAreaElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLAreaElement" , js_name = rel ) ]
    ///Getter for the `rel` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement/rel)
    ///
    ///*This API requires the following crate features to be activated: `HtmlAreaElement`*
    pub fn rel(this: &HtmlAreaElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLAreaElement" , js_name = rel ) ]
    ///Setter for the `rel` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement/rel)
    ///
    ///*This API requires the following crate features to be activated: `HtmlAreaElement`*
    pub fn set_rel(this: &HtmlAreaElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLAreaElement" , js_name = referrerPolicy ) ]
    ///Getter for the `referrerPolicy` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement/referrerPolicy)
    ///
    ///*This API requires the following crate features to be activated: `HtmlAreaElement`*
    pub fn referrer_policy(this: &HtmlAreaElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLAreaElement" , js_name = referrerPolicy ) ]
    ///Setter for the `referrerPolicy` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement/referrerPolicy)
    ///
    ///*This API requires the following crate features to be activated: `HtmlAreaElement`*
    pub fn set_referrer_policy(this: &HtmlAreaElement, value: &str);

    #[cfg(feature = "DomTokenList")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLAreaElement" , js_name = relList ) ]
    ///Getter for the `relList` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement/relList)
    ///
    ///*This API requires the following crate features to be activated: `DomTokenList`, `HtmlAreaElement`*
    pub fn rel_list(this: &HtmlAreaElement) -> DomTokenList;

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLAreaElement" , js_name = noHref ) ]
    ///Getter for the `noHref` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement/noHref)
    ///
    ///*This API requires the following crate features to be activated: `HtmlAreaElement`*
    pub fn no_href(this: &HtmlAreaElement) -> bool;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLAreaElement" , js_name = noHref ) ]
    ///Setter for the `noHref` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement/noHref)
    ///
    ///*This API requires the following crate features to be activated: `HtmlAreaElement`*
    pub fn set_no_href(this: &HtmlAreaElement, value: bool);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLAreaElement" , js_name = href ) ]
    ///Getter for the `href` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement/href)
    ///
    ///*This API requires the following crate features to be activated: `HtmlAreaElement`*
    pub fn href(this: &HtmlAreaElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLAreaElement" , js_name = href ) ]
    ///Setter for the `href` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement/href)
    ///
    ///*This API requires the following crate features to be activated: `HtmlAreaElement`*
    pub fn set_href(this: &HtmlAreaElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLAreaElement" , js_name = origin ) ]
    ///Getter for the `origin` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement/origin)
    ///
    ///*This API requires the following crate features to be activated: `HtmlAreaElement`*
    pub fn origin(this: &HtmlAreaElement) -> String;

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLAreaElement" , js_name = protocol ) ]
    ///Getter for the `protocol` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement/protocol)
    ///
    ///*This API requires the following crate features to be activated: `HtmlAreaElement`*
    pub fn protocol(this: &HtmlAreaElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLAreaElement" , js_name = protocol ) ]
    ///Setter for the `protocol` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement/protocol)
    ///
    ///*This API requires the following crate features to be activated: `HtmlAreaElement`*
    pub fn set_protocol(this: &HtmlAreaElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLAreaElement" , js_name = username ) ]
    ///Getter for the `username` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement/username)
    ///
    ///*This API requires the following crate features to be activated: `HtmlAreaElement`*
    pub fn username(this: &HtmlAreaElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLAreaElement" , js_name = username ) ]
    ///Setter for the `username` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement/username)
    ///
    ///*This API requires the following crate features to be activated: `HtmlAreaElement`*
    pub fn set_username(this: &HtmlAreaElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLAreaElement" , js_name = password ) ]
    ///Getter for the `password` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement/password)
    ///
    ///*This API requires the following crate features to be activated: `HtmlAreaElement`*
    pub fn password(this: &HtmlAreaElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLAreaElement" , js_name = password ) ]
    ///Setter for the `password` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement/password)
    ///
    ///*This API requires the following crate features to be activated: `HtmlAreaElement`*
    pub fn set_password(this: &HtmlAreaElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLAreaElement" , js_name = host ) ]
    ///Getter for the `host` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement/host)
    ///
    ///*This API requires the following crate features to be activated: `HtmlAreaElement`*
    pub fn host(this: &HtmlAreaElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLAreaElement" , js_name = host ) ]
    ///Setter for the `host` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement/host)
    ///
    ///*This API requires the following crate features to be activated: `HtmlAreaElement`*
    pub fn set_host(this: &HtmlAreaElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLAreaElement" , js_name = hostname ) ]
    ///Getter for the `hostname` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement/hostname)
    ///
    ///*This API requires the following crate features to be activated: `HtmlAreaElement`*
    pub fn hostname(this: &HtmlAreaElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLAreaElement" , js_name = hostname ) ]
    ///Setter for the `hostname` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement/hostname)
    ///
    ///*This API requires the following crate features to be activated: `HtmlAreaElement`*
    pub fn set_hostname(this: &HtmlAreaElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLAreaElement" , js_name = port ) ]
    ///Getter for the `port` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement/port)
    ///
    ///*This API requires the following crate features to be activated: `HtmlAreaElement`*
    pub fn port(this: &HtmlAreaElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLAreaElement" , js_name = port ) ]
    ///Setter for the `port` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement/port)
    ///
    ///*This API requires the following crate features to be activated: `HtmlAreaElement`*
    pub fn set_port(this: &HtmlAreaElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLAreaElement" , js_name = pathname ) ]
    ///Getter for the `pathname` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement/pathname)
    ///
    ///*This API requires the following crate features to be activated: `HtmlAreaElement`*
    pub fn pathname(this: &HtmlAreaElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLAreaElement" , js_name = pathname ) ]
    ///Setter for the `pathname` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement/pathname)
    ///
    ///*This API requires the following crate features to be activated: `HtmlAreaElement`*
    pub fn set_pathname(this: &HtmlAreaElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLAreaElement" , js_name = search ) ]
    ///Getter for the `search` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement/search)
    ///
    ///*This API requires the following crate features to be activated: `HtmlAreaElement`*
    pub fn search(this: &HtmlAreaElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLAreaElement" , js_name = search ) ]
    ///Setter for the `search` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement/search)
    ///
    ///*This API requires the following crate features to be activated: `HtmlAreaElement`*
    pub fn set_search(this: &HtmlAreaElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLAreaElement" , js_name = hash ) ]
    ///Getter for the `hash` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement/hash)
    ///
    ///*This API requires the following crate features to be activated: `HtmlAreaElement`*
    pub fn hash(this: &HtmlAreaElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLAreaElement" , js_name = hash ) ]
    ///Setter for the `hash` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement/hash)
    ///
    ///*This API requires the following crate features to be activated: `HtmlAreaElement`*
    pub fn set_hash(this: &HtmlAreaElement, value: &str);

}
