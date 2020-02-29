use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = URL , typescript_type = "URL" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `Url` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URL)
    ///
    ///*This API requires the following crate features to be activated: `Url`*
    pub type Url;

    # [ wasm_bindgen ( structural , method , getter , js_class = "URL" , js_name = href ) ]
    ///Getter for the `href` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URL/href)
    ///
    ///*This API requires the following crate features to be activated: `Url`*
    pub fn href(this: &Url) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "URL" , js_name = href ) ]
    ///Setter for the `href` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URL/href)
    ///
    ///*This API requires the following crate features to be activated: `Url`*
    pub fn set_href(this: &Url, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "URL" , js_name = origin ) ]
    ///Getter for the `origin` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URL/origin)
    ///
    ///*This API requires the following crate features to be activated: `Url`*
    pub fn origin(this: &Url) -> String;

    # [ wasm_bindgen ( structural , method , getter , js_class = "URL" , js_name = protocol ) ]
    ///Getter for the `protocol` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URL/protocol)
    ///
    ///*This API requires the following crate features to be activated: `Url`*
    pub fn protocol(this: &Url) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "URL" , js_name = protocol ) ]
    ///Setter for the `protocol` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URL/protocol)
    ///
    ///*This API requires the following crate features to be activated: `Url`*
    pub fn set_protocol(this: &Url, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "URL" , js_name = username ) ]
    ///Getter for the `username` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URL/username)
    ///
    ///*This API requires the following crate features to be activated: `Url`*
    pub fn username(this: &Url) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "URL" , js_name = username ) ]
    ///Setter for the `username` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URL/username)
    ///
    ///*This API requires the following crate features to be activated: `Url`*
    pub fn set_username(this: &Url, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "URL" , js_name = password ) ]
    ///Getter for the `password` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URL/password)
    ///
    ///*This API requires the following crate features to be activated: `Url`*
    pub fn password(this: &Url) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "URL" , js_name = password ) ]
    ///Setter for the `password` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URL/password)
    ///
    ///*This API requires the following crate features to be activated: `Url`*
    pub fn set_password(this: &Url, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "URL" , js_name = host ) ]
    ///Getter for the `host` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URL/host)
    ///
    ///*This API requires the following crate features to be activated: `Url`*
    pub fn host(this: &Url) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "URL" , js_name = host ) ]
    ///Setter for the `host` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URL/host)
    ///
    ///*This API requires the following crate features to be activated: `Url`*
    pub fn set_host(this: &Url, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "URL" , js_name = hostname ) ]
    ///Getter for the `hostname` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URL/hostname)
    ///
    ///*This API requires the following crate features to be activated: `Url`*
    pub fn hostname(this: &Url) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "URL" , js_name = hostname ) ]
    ///Setter for the `hostname` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URL/hostname)
    ///
    ///*This API requires the following crate features to be activated: `Url`*
    pub fn set_hostname(this: &Url, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "URL" , js_name = port ) ]
    ///Getter for the `port` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URL/port)
    ///
    ///*This API requires the following crate features to be activated: `Url`*
    pub fn port(this: &Url) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "URL" , js_name = port ) ]
    ///Setter for the `port` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URL/port)
    ///
    ///*This API requires the following crate features to be activated: `Url`*
    pub fn set_port(this: &Url, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "URL" , js_name = pathname ) ]
    ///Getter for the `pathname` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URL/pathname)
    ///
    ///*This API requires the following crate features to be activated: `Url`*
    pub fn pathname(this: &Url) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "URL" , js_name = pathname ) ]
    ///Setter for the `pathname` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URL/pathname)
    ///
    ///*This API requires the following crate features to be activated: `Url`*
    pub fn set_pathname(this: &Url, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "URL" , js_name = search ) ]
    ///Getter for the `search` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URL/search)
    ///
    ///*This API requires the following crate features to be activated: `Url`*
    pub fn search(this: &Url) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "URL" , js_name = search ) ]
    ///Setter for the `search` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URL/search)
    ///
    ///*This API requires the following crate features to be activated: `Url`*
    pub fn set_search(this: &Url, value: &str);

    #[cfg(feature = "UrlSearchParams")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "URL" , js_name = searchParams ) ]
    ///Getter for the `searchParams` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URL/searchParams)
    ///
    ///*This API requires the following crate features to be activated: `Url`, `UrlSearchParams`*
    pub fn search_params(this: &Url) -> UrlSearchParams;

    # [ wasm_bindgen ( structural , method , getter , js_class = "URL" , js_name = hash ) ]
    ///Getter for the `hash` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URL/hash)
    ///
    ///*This API requires the following crate features to be activated: `Url`*
    pub fn hash(this: &Url) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "URL" , js_name = hash ) ]
    ///Setter for the `hash` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URL/hash)
    ///
    ///*This API requires the following crate features to be activated: `Url`*
    pub fn set_hash(this: &Url, value: &str);

    #[wasm_bindgen(catch, constructor, js_class = "URL")]
    ///The `new Url(..)` constructor, creating a new instance of `Url`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URL/URL)
    ///
    ///*This API requires the following crate features to be activated: `Url`*
    pub fn new(url: &str) -> Result<Url, JsValue>;

    #[wasm_bindgen(catch, constructor, js_class = "URL")]
    ///The `new Url(..)` constructor, creating a new instance of `Url`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URL/URL)
    ///
    ///*This API requires the following crate features to be activated: `Url`*
    pub fn new_with_base(url: &str, base: &str) -> Result<Url, JsValue>;

    #[cfg(feature = "Blob")]
    # [ wasm_bindgen ( catch , static_method_of = Url , js_class = "URL" , js_name = createObjectURL ) ]
    ///The `createObjectURL()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URL/createObjectURL)
    ///
    ///*This API requires the following crate features to be activated: `Blob`, `Url`*
    pub fn create_object_url_with_blob(blob: &Blob) -> Result<String, JsValue>;

    #[cfg(feature = "MediaSource")]
    # [ wasm_bindgen ( catch , static_method_of = Url , js_class = "URL" , js_name = createObjectURL ) ]
    ///The `createObjectURL()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URL/createObjectURL)
    ///
    ///*This API requires the following crate features to be activated: `MediaSource`, `Url`*
    pub fn create_object_url_with_source(source: &MediaSource) -> Result<String, JsValue>;

    # [ wasm_bindgen ( catch , static_method_of = Url , js_class = "URL" , js_name = revokeObjectURL ) ]
    ///The `revokeObjectURL()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URL/revokeObjectURL)
    ///
    ///*This API requires the following crate features to be activated: `Url`*
    pub fn revoke_object_url(url: &str) -> Result<(), JsValue>;

    # [ wasm_bindgen ( method , structural , js_class = "URL" , js_name = toJSON ) ]
    ///The `toJSON()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URL/toJSON)
    ///
    ///*This API requires the following crate features to be activated: `Url`*
    pub fn to_json(this: &Url) -> String;

}
