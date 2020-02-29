use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = IdleDeadline , typescript_type = "IdleDeadline" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `IdleDeadline` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IdleDeadline)
    ///
    ///*This API requires the following crate features to be activated: `IdleDeadline`*
    pub type IdleDeadline;

    # [ wasm_bindgen ( structural , method , getter , js_class = "IdleDeadline" , js_name = didTimeout ) ]
    ///Getter for the `didTimeout` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IdleDeadline/didTimeout)
    ///
    ///*This API requires the following crate features to be activated: `IdleDeadline`*
    pub fn did_timeout(this: &IdleDeadline) -> bool;

    # [ wasm_bindgen ( method , structural , js_class = "IdleDeadline" , js_name = timeRemaining ) ]
    ///The `timeRemaining()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IdleDeadline/timeRemaining)
    ///
    ///*This API requires the following crate features to be activated: `IdleDeadline`*
    pub fn time_remaining(this: &IdleDeadline) -> f64;

}
