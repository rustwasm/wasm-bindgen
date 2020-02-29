use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = Event , extends = :: js_sys :: Object , js_name = GamepadEvent , typescript_type = "GamepadEvent" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `GamepadEvent` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GamepadEvent)
    ///
    ///*This API requires the following crate features to be activated: `GamepadEvent`*
    pub type GamepadEvent;

    #[cfg(feature = "Gamepad")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "GamepadEvent" , js_name = gamepad ) ]
    ///Getter for the `gamepad` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GamepadEvent/gamepad)
    ///
    ///*This API requires the following crate features to be activated: `Gamepad`, `GamepadEvent`*
    pub fn gamepad(this: &GamepadEvent) -> Option<Gamepad>;

    #[wasm_bindgen(catch, constructor, js_class = "GamepadEvent")]
    ///The `new GamepadEvent(..)` constructor, creating a new instance of `GamepadEvent`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GamepadEvent/GamepadEvent)
    ///
    ///*This API requires the following crate features to be activated: `GamepadEvent`*
    pub fn new(type_: &str) -> Result<GamepadEvent, JsValue>;

    #[cfg(feature = "GamepadEventInit")]
    #[wasm_bindgen(catch, constructor, js_class = "GamepadEvent")]
    ///The `new GamepadEvent(..)` constructor, creating a new instance of `GamepadEvent`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GamepadEvent/GamepadEvent)
    ///
    ///*This API requires the following crate features to be activated: `GamepadEvent`, `GamepadEventInit`*
    pub fn new_with_event_init_dict(
        type_: &str,
        event_init_dict: &GamepadEventInit,
    ) -> Result<GamepadEvent, JsValue>;

}
