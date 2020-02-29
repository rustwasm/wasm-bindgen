use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = GamepadEvent , extends = Event , extends = :: js_sys :: Object , js_name = GamepadAxisMoveEvent , typescript_name = GamepadAxisMoveEvent ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `GamepadAxisMoveEvent` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GamepadAxisMoveEvent)
    ///
    ///*This API requires the following crate features to be activated: `GamepadAxisMoveEvent`*
    pub type GamepadAxisMoveEvent;

    # [ wasm_bindgen ( structural , method , getter , js_class = "GamepadAxisMoveEvent" , js_name = axis ) ]
    ///Getter for the `axis` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GamepadAxisMoveEvent/axis)
    ///
    ///*This API requires the following crate features to be activated: `GamepadAxisMoveEvent`*
    pub fn axis(this: &GamepadAxisMoveEvent) -> u32;

    # [ wasm_bindgen ( structural , method , getter , js_class = "GamepadAxisMoveEvent" , js_name = value ) ]
    ///Getter for the `value` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GamepadAxisMoveEvent/value)
    ///
    ///*This API requires the following crate features to be activated: `GamepadAxisMoveEvent`*
    pub fn value(this: &GamepadAxisMoveEvent) -> f64;

    #[wasm_bindgen(catch, constructor, js_class = "GamepadAxisMoveEvent")]
    ///The `new GamepadAxisMoveEvent(..)` constructor, creating a new instance of `GamepadAxisMoveEvent`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GamepadAxisMoveEvent/GamepadAxisMoveEvent)
    ///
    ///*This API requires the following crate features to be activated: `GamepadAxisMoveEvent`*
    pub fn new(type_: &str) -> Result<GamepadAxisMoveEvent, JsValue>;

    #[cfg(feature = "GamepadAxisMoveEventInit")]
    #[wasm_bindgen(catch, constructor, js_class = "GamepadAxisMoveEvent")]
    ///The `new GamepadAxisMoveEvent(..)` constructor, creating a new instance of `GamepadAxisMoveEvent`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GamepadAxisMoveEvent/GamepadAxisMoveEvent)
    ///
    ///*This API requires the following crate features to be activated: `GamepadAxisMoveEvent`, `GamepadAxisMoveEventInit`*
    pub fn new_with_event_init_dict(
        type_: &str,
        event_init_dict: &GamepadAxisMoveEventInit,
    ) -> Result<GamepadAxisMoveEvent, JsValue>;

}
