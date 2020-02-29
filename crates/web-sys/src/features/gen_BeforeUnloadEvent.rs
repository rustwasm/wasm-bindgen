use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = Event , extends = :: js_sys :: Object , js_name = BeforeUnloadEvent , typescript_type = "BeforeUnloadEvent" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `BeforeUnloadEvent` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BeforeUnloadEvent)
    ///
    ///*This API requires the following crate features to be activated: `BeforeUnloadEvent`*
    pub type BeforeUnloadEvent;

    # [ wasm_bindgen ( structural , method , getter , js_class = "BeforeUnloadEvent" , js_name = returnValue ) ]
    ///Getter for the `returnValue` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BeforeUnloadEvent/returnValue)
    ///
    ///*This API requires the following crate features to be activated: `BeforeUnloadEvent`*
    pub fn return_value(this: &BeforeUnloadEvent) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "BeforeUnloadEvent" , js_name = returnValue ) ]
    ///Setter for the `returnValue` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BeforeUnloadEvent/returnValue)
    ///
    ///*This API requires the following crate features to be activated: `BeforeUnloadEvent`*
    pub fn set_return_value(this: &BeforeUnloadEvent, value: &str);

}
