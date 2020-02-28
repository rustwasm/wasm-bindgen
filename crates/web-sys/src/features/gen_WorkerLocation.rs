use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = WorkerLocation , typescript_name = WorkerLocation ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `WorkerLocation` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerLocation)\n\n*This API requires the following crate features to be activated: `WorkerLocation`*"]
    pub type WorkerLocation;
    # [ wasm_bindgen ( structural , method , getter , js_class = "WorkerLocation" , js_name = href ) ]
    #[doc = "Getter for the `href` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerLocation/href)\n\n*This API requires the following crate features to be activated: `WorkerLocation`*"]
    pub fn href(this: &WorkerLocation) -> String;
    # [ wasm_bindgen ( structural , method , getter , js_class = "WorkerLocation" , js_name = origin ) ]
    #[doc = "Getter for the `origin` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerLocation/origin)\n\n*This API requires the following crate features to be activated: `WorkerLocation`*"]
    pub fn origin(this: &WorkerLocation) -> String;
    # [ wasm_bindgen ( structural , method , getter , js_class = "WorkerLocation" , js_name = protocol ) ]
    #[doc = "Getter for the `protocol` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerLocation/protocol)\n\n*This API requires the following crate features to be activated: `WorkerLocation`*"]
    pub fn protocol(this: &WorkerLocation) -> String;
    # [ wasm_bindgen ( structural , method , getter , js_class = "WorkerLocation" , js_name = host ) ]
    #[doc = "Getter for the `host` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerLocation/host)\n\n*This API requires the following crate features to be activated: `WorkerLocation`*"]
    pub fn host(this: &WorkerLocation) -> String;
    # [ wasm_bindgen ( structural , method , getter , js_class = "WorkerLocation" , js_name = hostname ) ]
    #[doc = "Getter for the `hostname` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerLocation/hostname)\n\n*This API requires the following crate features to be activated: `WorkerLocation`*"]
    pub fn hostname(this: &WorkerLocation) -> String;
    # [ wasm_bindgen ( structural , method , getter , js_class = "WorkerLocation" , js_name = port ) ]
    #[doc = "Getter for the `port` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerLocation/port)\n\n*This API requires the following crate features to be activated: `WorkerLocation`*"]
    pub fn port(this: &WorkerLocation) -> String;
    # [ wasm_bindgen ( structural , method , getter , js_class = "WorkerLocation" , js_name = pathname ) ]
    #[doc = "Getter for the `pathname` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerLocation/pathname)\n\n*This API requires the following crate features to be activated: `WorkerLocation`*"]
    pub fn pathname(this: &WorkerLocation) -> String;
    # [ wasm_bindgen ( structural , method , getter , js_class = "WorkerLocation" , js_name = search ) ]
    #[doc = "Getter for the `search` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerLocation/search)\n\n*This API requires the following crate features to be activated: `WorkerLocation`*"]
    pub fn search(this: &WorkerLocation) -> String;
    # [ wasm_bindgen ( structural , method , getter , js_class = "WorkerLocation" , js_name = hash ) ]
    #[doc = "Getter for the `hash` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerLocation/hash)\n\n*This API requires the following crate features to be activated: `WorkerLocation`*"]
    pub fn hash(this: &WorkerLocation) -> String;
}
