use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = URL , typescript_name = URL ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `Url` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URL)\n\n*This API requires the following crate features to be activated: `Url`*"]
    pub type Url;
    # [ wasm_bindgen ( structural , method , getter , js_name = href ) ]
    #[doc = "Getter for the `href` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URL/href)\n\n*This API requires the following crate features to be activated: `Url`*"]
    pub fn href(this: &Url) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = href ) ]
    #[doc = "Setter for the `href` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URL/href)\n\n*This API requires the following crate features to be activated: `Url`*"]
    pub fn set_href(this: &Url, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_name = origin ) ]
    #[doc = "Getter for the `origin` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URL/origin)\n\n*This API requires the following crate features to be activated: `Url`*"]
    pub fn origin(this: &Url) -> String;
    # [ wasm_bindgen ( structural , method , getter , js_name = protocol ) ]
    #[doc = "Getter for the `protocol` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URL/protocol)\n\n*This API requires the following crate features to be activated: `Url`*"]
    pub fn protocol(this: &Url) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = protocol ) ]
    #[doc = "Setter for the `protocol` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URL/protocol)\n\n*This API requires the following crate features to be activated: `Url`*"]
    pub fn set_protocol(this: &Url, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_name = username ) ]
    #[doc = "Getter for the `username` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URL/username)\n\n*This API requires the following crate features to be activated: `Url`*"]
    pub fn username(this: &Url) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = username ) ]
    #[doc = "Setter for the `username` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URL/username)\n\n*This API requires the following crate features to be activated: `Url`*"]
    pub fn set_username(this: &Url, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_name = password ) ]
    #[doc = "Getter for the `password` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URL/password)\n\n*This API requires the following crate features to be activated: `Url`*"]
    pub fn password(this: &Url) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = password ) ]
    #[doc = "Setter for the `password` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URL/password)\n\n*This API requires the following crate features to be activated: `Url`*"]
    pub fn set_password(this: &Url, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_name = host ) ]
    #[doc = "Getter for the `host` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URL/host)\n\n*This API requires the following crate features to be activated: `Url`*"]
    pub fn host(this: &Url) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = host ) ]
    #[doc = "Setter for the `host` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URL/host)\n\n*This API requires the following crate features to be activated: `Url`*"]
    pub fn set_host(this: &Url, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_name = hostname ) ]
    #[doc = "Getter for the `hostname` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URL/hostname)\n\n*This API requires the following crate features to be activated: `Url`*"]
    pub fn hostname(this: &Url) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = hostname ) ]
    #[doc = "Setter for the `hostname` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URL/hostname)\n\n*This API requires the following crate features to be activated: `Url`*"]
    pub fn set_hostname(this: &Url, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_name = port ) ]
    #[doc = "Getter for the `port` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URL/port)\n\n*This API requires the following crate features to be activated: `Url`*"]
    pub fn port(this: &Url) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = port ) ]
    #[doc = "Setter for the `port` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URL/port)\n\n*This API requires the following crate features to be activated: `Url`*"]
    pub fn set_port(this: &Url, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_name = pathname ) ]
    #[doc = "Getter for the `pathname` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URL/pathname)\n\n*This API requires the following crate features to be activated: `Url`*"]
    pub fn pathname(this: &Url) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = pathname ) ]
    #[doc = "Setter for the `pathname` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URL/pathname)\n\n*This API requires the following crate features to be activated: `Url`*"]
    pub fn set_pathname(this: &Url, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_name = search ) ]
    #[doc = "Getter for the `search` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URL/search)\n\n*This API requires the following crate features to be activated: `Url`*"]
    pub fn search(this: &Url) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = search ) ]
    #[doc = "Setter for the `search` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URL/search)\n\n*This API requires the following crate features to be activated: `Url`*"]
    pub fn set_search(this: &Url, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_name = searchParams ) ]
    #[cfg(feature = "UrlSearchParams")]
    #[doc = "Getter for the `searchParams` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URL/searchParams)\n\n*This API requires the following crate features to be activated: `Url`, `UrlSearchParams`*"]
    pub fn search_params(this: &Url) -> UrlSearchParams;
    # [ wasm_bindgen ( structural , method , getter , js_name = hash ) ]
    #[doc = "Getter for the `hash` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URL/hash)\n\n*This API requires the following crate features to be activated: `Url`*"]
    pub fn hash(this: &Url) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = hash ) ]
    #[doc = "Setter for the `hash` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URL/hash)\n\n*This API requires the following crate features to be activated: `Url`*"]
    pub fn set_hash(this: &Url, value: &str);
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new Url(..)` constructor, creating a new instance of `Url`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URL/URL)\n\n*This API requires the following crate features to be activated: `Url`*"]
    pub fn new(this: &Url, url: &str) -> Result<Url, JsValue>;
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new Url(..)` constructor, creating a new instance of `Url`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URL/URL)\n\n*This API requires the following crate features to be activated: `Url`*"]
    pub fn new_with_base(this: &Url, url: &str, base: &str) -> Result<Url, JsValue>;
    #[cfg(feature = "Blob")]
    # [ wasm_bindgen ( catch , static_method_of = URL , js_name = createObjectURL ) ]
    #[doc = "The `createObjectURL()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URL/createObjectURL)\n\n*This API requires the following crate features to be activated: `Blob`, `Url`*"]
    pub fn create_object_url_with_blob(blob: &Blob) -> Result<String, JsValue>;
    #[cfg(feature = "MediaSource")]
    # [ wasm_bindgen ( catch , static_method_of = URL , js_name = createObjectURL ) ]
    #[doc = "The `createObjectURL()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URL/createObjectURL)\n\n*This API requires the following crate features to be activated: `MediaSource`, `Url`*"]
    pub fn create_object_url_with_source(source: &MediaSource) -> Result<String, JsValue>;
    # [ wasm_bindgen ( catch , static_method_of = URL , js_name = revokeObjectURL ) ]
    #[doc = "The `revokeObjectURL()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URL/revokeObjectURL)\n\n*This API requires the following crate features to be activated: `Url`*"]
    pub fn revoke_object_url(url: &str) -> Result<(), JsValue>;
    # [ wasm_bindgen ( method , structural , js_name = toJSON ) ]
    #[doc = "The `toJSON()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URL/toJSON)\n\n*This API requires the following crate features to be activated: `Url`*"]
    pub fn to_json(this: &Url) -> String;
}
