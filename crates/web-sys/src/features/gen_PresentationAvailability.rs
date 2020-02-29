use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = EventTarget , extends = :: js_sys :: Object , js_name = PresentationAvailability , typescript_name = PresentationAvailability ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `PresentationAvailability` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationAvailability)
    ///
    ///*This API requires the following crate features to be activated: `PresentationAvailability`*
    pub type PresentationAvailability;

    # [ wasm_bindgen ( structural , method , getter , js_class = "PresentationAvailability" , js_name = value ) ]
    ///Getter for the `value` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationAvailability/value)
    ///
    ///*This API requires the following crate features to be activated: `PresentationAvailability`*
    pub fn value(this: &PresentationAvailability) -> bool;

    # [ wasm_bindgen ( structural , method , getter , js_class = "PresentationAvailability" , js_name = onchange ) ]
    ///Getter for the `onchange` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationAvailability/onchange)
    ///
    ///*This API requires the following crate features to be activated: `PresentationAvailability`*
    pub fn onchange(this: &PresentationAvailability) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "PresentationAvailability" , js_name = onchange ) ]
    ///Setter for the `onchange` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationAvailability/onchange)
    ///
    ///*This API requires the following crate features to be activated: `PresentationAvailability`*
    pub fn set_onchange(this: &PresentationAvailability, value: Option<&::js_sys::Function>);

}
