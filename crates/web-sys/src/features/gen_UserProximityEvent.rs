use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = Event , extends = :: js_sys :: Object , js_name = UserProximityEvent , typescript_type = "UserProximityEvent" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `UserProximityEvent` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/UserProximityEvent)
    ///
    ///*This API requires the following crate features to be activated: `UserProximityEvent`*
    pub type UserProximityEvent;

    # [ wasm_bindgen ( structural , method , getter , js_class = "UserProximityEvent" , js_name = near ) ]
    ///Getter for the `near` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/UserProximityEvent/near)
    ///
    ///*This API requires the following crate features to be activated: `UserProximityEvent`*
    pub fn near(this: &UserProximityEvent) -> bool;

    #[wasm_bindgen(catch, constructor, js_class = "UserProximityEvent")]
    ///The `new UserProximityEvent(..)` constructor, creating a new instance of `UserProximityEvent`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/UserProximityEvent/UserProximityEvent)
    ///
    ///*This API requires the following crate features to be activated: `UserProximityEvent`*
    pub fn new(type_: &str) -> Result<UserProximityEvent, JsValue>;

    #[cfg(feature = "UserProximityEventInit")]
    #[wasm_bindgen(catch, constructor, js_class = "UserProximityEvent")]
    ///The `new UserProximityEvent(..)` constructor, creating a new instance of `UserProximityEvent`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/UserProximityEvent/UserProximityEvent)
    ///
    ///*This API requires the following crate features to be activated: `UserProximityEvent`, `UserProximityEventInit`*
    pub fn new_with_event_init_dict(
        type_: &str,
        event_init_dict: &UserProximityEventInit,
    ) -> Result<UserProximityEvent, JsValue>;

}
