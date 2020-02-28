use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = Event , extends = :: js_sys :: Object , js_name = BlobEvent , typescript_name = BlobEvent ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `BlobEvent` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BlobEvent)\n\n*This API requires the following crate features to be activated: `BlobEvent`*"]
    pub type BlobEvent;
    # [ wasm_bindgen ( structural , method , getter , js_class = "BlobEvent" , js_name = data ) ]
    #[cfg(feature = "Blob")]
    #[doc = "Getter for the `data` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BlobEvent/data)\n\n*This API requires the following crate features to be activated: `Blob`, `BlobEvent`*"]
    pub fn data(this: &BlobEvent) -> Option<Blob>;
    #[wasm_bindgen(catch, js_class = "BlobEvent", constructor)]
    #[doc = "The `new BlobEvent(..)` constructor, creating a new instance of `BlobEvent`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BlobEvent/BlobEvent)\n\n*This API requires the following crate features to be activated: `BlobEvent`*"]
    pub fn new(this: &BlobEvent, type_: &str) -> Result<BlobEvent, JsValue>;
    #[cfg(feature = "BlobEventInit")]
    #[wasm_bindgen(catch, js_class = "BlobEvent", constructor)]
    #[doc = "The `new BlobEvent(..)` constructor, creating a new instance of `BlobEvent`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BlobEvent/BlobEvent)\n\n*This API requires the following crate features to be activated: `BlobEvent`, `BlobEventInit`*"]
    pub fn new_with_event_init_dict(
        this: &BlobEvent,
        type_: &str,
        event_init_dict: &BlobEventInit,
    ) -> Result<BlobEvent, JsValue>;
}
