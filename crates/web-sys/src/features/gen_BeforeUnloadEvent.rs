use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = Event , extends = :: js_sys :: Object , js_name = BeforeUnloadEvent , typescript_name = BeforeUnloadEvent ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `BeforeUnloadEvent` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BeforeUnloadEvent)\n\n*This API requires the following crate features to be activated: `BeforeUnloadEvent`*"]
    pub type BeforeUnloadEvent;
    # [ wasm_bindgen ( structural , method , getter , js_name = returnValue ) ]
    #[doc = "Getter for the `returnValue` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BeforeUnloadEvent/returnValue)\n\n*This API requires the following crate features to be activated: `BeforeUnloadEvent`*"]
    pub fn return_value(this: &BeforeUnloadEvent) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = returnValue ) ]
    #[doc = "Setter for the `returnValue` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BeforeUnloadEvent/returnValue)\n\n*This API requires the following crate features to be activated: `BeforeUnloadEvent`*"]
    pub fn set_return_value(this: &BeforeUnloadEvent, value: String);
}
