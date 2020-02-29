use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = Location , typescript_name = Location ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `Location` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Location)
    ///
    ///*This API requires the following crate features to be activated: `Location`*
    pub type Location;

    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "Location" , js_name = href ) ]
    ///Getter for the `href` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Location/href)
    ///
    ///*This API requires the following crate features to be activated: `Location`*
    pub fn href(this: &Location) -> Result<String, JsValue>;

    # [ wasm_bindgen ( structural , catch , method , setter , js_class = "Location" , js_name = href ) ]
    ///Setter for the `href` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Location/href)
    ///
    ///*This API requires the following crate features to be activated: `Location`*
    pub fn set_href(this: &Location, value: &str) -> Result<(), JsValue>;

    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "Location" , js_name = origin ) ]
    ///Getter for the `origin` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Location/origin)
    ///
    ///*This API requires the following crate features to be activated: `Location`*
    pub fn origin(this: &Location) -> Result<String, JsValue>;

    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "Location" , js_name = protocol ) ]
    ///Getter for the `protocol` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Location/protocol)
    ///
    ///*This API requires the following crate features to be activated: `Location`*
    pub fn protocol(this: &Location) -> Result<String, JsValue>;

    # [ wasm_bindgen ( structural , catch , method , setter , js_class = "Location" , js_name = protocol ) ]
    ///Setter for the `protocol` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Location/protocol)
    ///
    ///*This API requires the following crate features to be activated: `Location`*
    pub fn set_protocol(this: &Location, value: &str) -> Result<(), JsValue>;

    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "Location" , js_name = host ) ]
    ///Getter for the `host` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Location/host)
    ///
    ///*This API requires the following crate features to be activated: `Location`*
    pub fn host(this: &Location) -> Result<String, JsValue>;

    # [ wasm_bindgen ( structural , catch , method , setter , js_class = "Location" , js_name = host ) ]
    ///Setter for the `host` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Location/host)
    ///
    ///*This API requires the following crate features to be activated: `Location`*
    pub fn set_host(this: &Location, value: &str) -> Result<(), JsValue>;

    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "Location" , js_name = hostname ) ]
    ///Getter for the `hostname` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Location/hostname)
    ///
    ///*This API requires the following crate features to be activated: `Location`*
    pub fn hostname(this: &Location) -> Result<String, JsValue>;

    # [ wasm_bindgen ( structural , catch , method , setter , js_class = "Location" , js_name = hostname ) ]
    ///Setter for the `hostname` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Location/hostname)
    ///
    ///*This API requires the following crate features to be activated: `Location`*
    pub fn set_hostname(this: &Location, value: &str) -> Result<(), JsValue>;

    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "Location" , js_name = port ) ]
    ///Getter for the `port` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Location/port)
    ///
    ///*This API requires the following crate features to be activated: `Location`*
    pub fn port(this: &Location) -> Result<String, JsValue>;

    # [ wasm_bindgen ( structural , catch , method , setter , js_class = "Location" , js_name = port ) ]
    ///Setter for the `port` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Location/port)
    ///
    ///*This API requires the following crate features to be activated: `Location`*
    pub fn set_port(this: &Location, value: &str) -> Result<(), JsValue>;

    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "Location" , js_name = pathname ) ]
    ///Getter for the `pathname` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Location/pathname)
    ///
    ///*This API requires the following crate features to be activated: `Location`*
    pub fn pathname(this: &Location) -> Result<String, JsValue>;

    # [ wasm_bindgen ( structural , catch , method , setter , js_class = "Location" , js_name = pathname ) ]
    ///Setter for the `pathname` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Location/pathname)
    ///
    ///*This API requires the following crate features to be activated: `Location`*
    pub fn set_pathname(this: &Location, value: &str) -> Result<(), JsValue>;

    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "Location" , js_name = search ) ]
    ///Getter for the `search` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Location/search)
    ///
    ///*This API requires the following crate features to be activated: `Location`*
    pub fn search(this: &Location) -> Result<String, JsValue>;

    # [ wasm_bindgen ( structural , catch , method , setter , js_class = "Location" , js_name = search ) ]
    ///Setter for the `search` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Location/search)
    ///
    ///*This API requires the following crate features to be activated: `Location`*
    pub fn set_search(this: &Location, value: &str) -> Result<(), JsValue>;

    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "Location" , js_name = hash ) ]
    ///Getter for the `hash` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Location/hash)
    ///
    ///*This API requires the following crate features to be activated: `Location`*
    pub fn hash(this: &Location) -> Result<String, JsValue>;

    # [ wasm_bindgen ( structural , catch , method , setter , js_class = "Location" , js_name = hash ) ]
    ///Setter for the `hash` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Location/hash)
    ///
    ///*This API requires the following crate features to be activated: `Location`*
    pub fn set_hash(this: &Location, value: &str) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Location" , js_name = assign ) ]
    ///The `assign()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Location/assign)
    ///
    ///*This API requires the following crate features to be activated: `Location`*
    pub fn assign(this: &Location, url: &str) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Location" , js_name = reload ) ]
    ///The `reload()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Location/reload)
    ///
    ///*This API requires the following crate features to be activated: `Location`*
    pub fn reload(this: &Location) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Location" , js_name = reload ) ]
    ///The `reload()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Location/reload)
    ///
    ///*This API requires the following crate features to be activated: `Location`*
    pub fn reload_with_forceget(this: &Location, forceget: bool) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Location" , js_name = replace ) ]
    ///The `replace()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Location/replace)
    ///
    ///*This API requires the following crate features to be activated: `Location`*
    pub fn replace(this: &Location, url: &str) -> Result<(), JsValue>;

}
