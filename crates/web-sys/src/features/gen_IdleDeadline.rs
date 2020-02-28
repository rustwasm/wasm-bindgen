use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = IdleDeadline , typescript_name = IdleDeadline ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `IdleDeadline` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IdleDeadline)\n\n*This API requires the following crate features to be activated: `IdleDeadline`*"]
    pub type IdleDeadline;
    # [ wasm_bindgen ( structural , method , getter , js_class = "IdleDeadline" , js_name = didTimeout ) ]
    #[doc = "Getter for the `didTimeout` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IdleDeadline/didTimeout)\n\n*This API requires the following crate features to be activated: `IdleDeadline`*"]
    pub fn did_timeout(this: &IdleDeadline) -> bool;
    # [ wasm_bindgen ( method , structural , js_class = "IdleDeadline" , js_name = timeRemaining ) ]
    #[doc = "The `timeRemaining()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IdleDeadline/timeRemaining)\n\n*This API requires the following crate features to be activated: `IdleDeadline`*"]
    pub fn time_remaining(this: &IdleDeadline) -> f64;
}
