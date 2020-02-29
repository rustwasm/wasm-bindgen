use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = Event , extends = :: js_sys :: Object , js_name = ProgressEvent , typescript_type = "ProgressEvent" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `ProgressEvent` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ProgressEvent)
    ///
    ///*This API requires the following crate features to be activated: `ProgressEvent`*
    pub type ProgressEvent;

    # [ wasm_bindgen ( structural , method , getter , js_class = "ProgressEvent" , js_name = lengthComputable ) ]
    ///Getter for the `lengthComputable` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ProgressEvent/lengthComputable)
    ///
    ///*This API requires the following crate features to be activated: `ProgressEvent`*
    pub fn length_computable(this: &ProgressEvent) -> bool;

    # [ wasm_bindgen ( structural , method , getter , js_class = "ProgressEvent" , js_name = loaded ) ]
    ///Getter for the `loaded` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ProgressEvent/loaded)
    ///
    ///*This API requires the following crate features to be activated: `ProgressEvent`*
    pub fn loaded(this: &ProgressEvent) -> f64;

    # [ wasm_bindgen ( structural , method , getter , js_class = "ProgressEvent" , js_name = total ) ]
    ///Getter for the `total` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ProgressEvent/total)
    ///
    ///*This API requires the following crate features to be activated: `ProgressEvent`*
    pub fn total(this: &ProgressEvent) -> f64;

    #[wasm_bindgen(catch, constructor, js_class = "ProgressEvent")]
    ///The `new ProgressEvent(..)` constructor, creating a new instance of `ProgressEvent`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ProgressEvent/ProgressEvent)
    ///
    ///*This API requires the following crate features to be activated: `ProgressEvent`*
    pub fn new(type_: &str) -> Result<ProgressEvent, JsValue>;

    #[cfg(feature = "ProgressEventInit")]
    #[wasm_bindgen(catch, constructor, js_class = "ProgressEvent")]
    ///The `new ProgressEvent(..)` constructor, creating a new instance of `ProgressEvent`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ProgressEvent/ProgressEvent)
    ///
    ///*This API requires the following crate features to be activated: `ProgressEvent`, `ProgressEventInit`*
    pub fn new_with_event_init_dict(
        type_: &str,
        event_init_dict: &ProgressEventInit,
    ) -> Result<ProgressEvent, JsValue>;

}
