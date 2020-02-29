use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = PresentationReceiver , typescript_type = "PresentationReceiver" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `PresentationReceiver` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationReceiver)
    ///
    ///*This API requires the following crate features to be activated: `PresentationReceiver`*
    pub type PresentationReceiver;

    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "PresentationReceiver" , js_name = connectionList ) ]
    ///Getter for the `connectionList` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationReceiver/connectionList)
    ///
    ///*This API requires the following crate features to be activated: `PresentationReceiver`*
    pub fn connection_list(this: &PresentationReceiver) -> Result<::js_sys::Promise, JsValue>;

}
