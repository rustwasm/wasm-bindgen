use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = WorkerLocation , typescript_type = "WorkerLocation" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `WorkerLocation` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerLocation)
    ///
    ///*This API requires the following crate features to be activated: `WorkerLocation`*
    pub type WorkerLocation;

    # [ wasm_bindgen ( structural , method , getter , js_class = "WorkerLocation" , js_name = href ) ]
    ///Getter for the `href` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerLocation/href)
    ///
    ///*This API requires the following crate features to be activated: `WorkerLocation`*
    pub fn href(this: &WorkerLocation) -> String;

    # [ wasm_bindgen ( structural , method , getter , js_class = "WorkerLocation" , js_name = origin ) ]
    ///Getter for the `origin` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerLocation/origin)
    ///
    ///*This API requires the following crate features to be activated: `WorkerLocation`*
    pub fn origin(this: &WorkerLocation) -> String;

    # [ wasm_bindgen ( structural , method , getter , js_class = "WorkerLocation" , js_name = protocol ) ]
    ///Getter for the `protocol` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerLocation/protocol)
    ///
    ///*This API requires the following crate features to be activated: `WorkerLocation`*
    pub fn protocol(this: &WorkerLocation) -> String;

    # [ wasm_bindgen ( structural , method , getter , js_class = "WorkerLocation" , js_name = host ) ]
    ///Getter for the `host` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerLocation/host)
    ///
    ///*This API requires the following crate features to be activated: `WorkerLocation`*
    pub fn host(this: &WorkerLocation) -> String;

    # [ wasm_bindgen ( structural , method , getter , js_class = "WorkerLocation" , js_name = hostname ) ]
    ///Getter for the `hostname` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerLocation/hostname)
    ///
    ///*This API requires the following crate features to be activated: `WorkerLocation`*
    pub fn hostname(this: &WorkerLocation) -> String;

    # [ wasm_bindgen ( structural , method , getter , js_class = "WorkerLocation" , js_name = port ) ]
    ///Getter for the `port` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerLocation/port)
    ///
    ///*This API requires the following crate features to be activated: `WorkerLocation`*
    pub fn port(this: &WorkerLocation) -> String;

    # [ wasm_bindgen ( structural , method , getter , js_class = "WorkerLocation" , js_name = pathname ) ]
    ///Getter for the `pathname` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerLocation/pathname)
    ///
    ///*This API requires the following crate features to be activated: `WorkerLocation`*
    pub fn pathname(this: &WorkerLocation) -> String;

    # [ wasm_bindgen ( structural , method , getter , js_class = "WorkerLocation" , js_name = search ) ]
    ///Getter for the `search` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerLocation/search)
    ///
    ///*This API requires the following crate features to be activated: `WorkerLocation`*
    pub fn search(this: &WorkerLocation) -> String;

    # [ wasm_bindgen ( structural , method , getter , js_class = "WorkerLocation" , js_name = hash ) ]
    ///Getter for the `hash` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerLocation/hash)
    ///
    ///*This API requires the following crate features to be activated: `WorkerLocation`*
    pub fn hash(this: &WorkerLocation) -> String;

}
