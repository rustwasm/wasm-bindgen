use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = PresentationReceiver , typescript_name = PresentationReceiver ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PresentationReceiver` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationReceiver)\n\n*This API requires the following crate features to be activated: `PresentationReceiver`*"]
    pub type PresentationReceiver;
    # [ wasm_bindgen ( structural , catch , method , getter , js_name = connectionList ) ]
    #[doc = "Getter for the `connectionList` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationReceiver/connectionList)\n\n*This API requires the following crate features to be activated: `PresentationReceiver`*"]
    pub fn connection_list(this: &PresentationReceiver) -> Result<::js_sys::Promise, JsValue>;
}
