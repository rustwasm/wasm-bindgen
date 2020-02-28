use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = Event , extends = :: js_sys :: Object , js_name = UserProximityEvent , typescript_name = UserProximityEvent ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `UserProximityEvent` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/UserProximityEvent)\n\n*This API requires the following crate features to be activated: `UserProximityEvent`*"]
    pub type UserProximityEvent;
    # [ wasm_bindgen ( structural , method , getter , js_class = "UserProximityEvent" , js_name = near ) ]
    #[doc = "Getter for the `near` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/UserProximityEvent/near)\n\n*This API requires the following crate features to be activated: `UserProximityEvent`*"]
    pub fn near(this: &UserProximityEvent) -> bool;
    #[wasm_bindgen(catch, js_class = "UserProximityEvent", constructor)]
    #[doc = "The `new UserProximityEvent(..)` constructor, creating a new instance of `UserProximityEvent`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/UserProximityEvent/UserProximityEvent)\n\n*This API requires the following crate features to be activated: `UserProximityEvent`*"]
    pub fn new(this: &UserProximityEvent, type_: &str) -> Result<UserProximityEvent, JsValue>;
    #[cfg(feature = "UserProximityEventInit")]
    #[wasm_bindgen(catch, js_class = "UserProximityEvent", constructor)]
    #[doc = "The `new UserProximityEvent(..)` constructor, creating a new instance of `UserProximityEvent`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/UserProximityEvent/UserProximityEvent)\n\n*This API requires the following crate features to be activated: `UserProximityEvent`, `UserProximityEventInit`*"]
    pub fn new_with_event_init_dict(
        this: &UserProximityEvent,
        type_: &str,
        event_init_dict: &UserProximityEventInit,
    ) -> Result<UserProximityEvent, JsValue>;
}
