use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = EventTarget , extends = :: js_sys :: Object , js_name = PresentationAvailability , typescript_name = PresentationAvailability ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PresentationAvailability` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationAvailability)\n\n*This API requires the following crate features to be activated: `PresentationAvailability`*"]
    pub type PresentationAvailability;
    # [ wasm_bindgen ( structural , method , getter , js_name = value ) ]
    #[doc = "Getter for the `value` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationAvailability/value)\n\n*This API requires the following crate features to be activated: `PresentationAvailability`*"]
    pub fn value(this: &PresentationAvailability) -> bool;
    # [ wasm_bindgen ( structural , method , getter , js_name = onchange ) ]
    #[doc = "Getter for the `onchange` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationAvailability/onchange)\n\n*This API requires the following crate features to be activated: `PresentationAvailability`*"]
    pub fn onchange(this: &PresentationAvailability) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onchange ) ]
    #[doc = "Setter for the `onchange` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationAvailability/onchange)\n\n*This API requires the following crate features to be activated: `PresentationAvailability`*"]
    pub fn set_onchange(this: &PresentationAvailability, value: Option<&::js_sys::Function>);
}
