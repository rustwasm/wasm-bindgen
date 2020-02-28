use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = Event , extends = :: js_sys :: Object , js_name = ProgressEvent , typescript_name = ProgressEvent ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ProgressEvent` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ProgressEvent)\n\n*This API requires the following crate features to be activated: `ProgressEvent`*"]
    pub type ProgressEvent;
    # [ wasm_bindgen ( structural , method , getter , js_name = lengthComputable ) ]
    #[doc = "Getter for the `lengthComputable` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ProgressEvent/lengthComputable)\n\n*This API requires the following crate features to be activated: `ProgressEvent`*"]
    pub fn length_computable(this: &ProgressEvent) -> bool;
    # [ wasm_bindgen ( structural , method , getter , js_name = loaded ) ]
    #[doc = "Getter for the `loaded` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ProgressEvent/loaded)\n\n*This API requires the following crate features to be activated: `ProgressEvent`*"]
    pub fn loaded(this: &ProgressEvent) -> f64;
    # [ wasm_bindgen ( structural , method , getter , js_name = total ) ]
    #[doc = "Getter for the `total` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ProgressEvent/total)\n\n*This API requires the following crate features to be activated: `ProgressEvent`*"]
    pub fn total(this: &ProgressEvent) -> f64;
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new ProgressEvent(..)` constructor, creating a new instance of `ProgressEvent`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ProgressEvent/ProgressEvent)\n\n*This API requires the following crate features to be activated: `ProgressEvent`*"]
    pub fn new(this: &ProgressEvent, type_: &str) -> Result<ProgressEvent, JsValue>;
    #[cfg(feature = "ProgressEventInit")]
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new ProgressEvent(..)` constructor, creating a new instance of `ProgressEvent`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ProgressEvent/ProgressEvent)\n\n*This API requires the following crate features to be activated: `ProgressEvent`, `ProgressEventInit`*"]
    pub fn new_with_event_init_dict(
        this: &ProgressEvent,
        type_: &str,
        event_init_dict: &ProgressEventInit,
    ) -> Result<ProgressEvent, JsValue>;
}
