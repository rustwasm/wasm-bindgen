use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = GamepadEvent , extends = Event , extends = :: js_sys :: Object , js_name = GamepadButtonEvent , typescript_name = GamepadButtonEvent ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `GamepadButtonEvent` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GamepadButtonEvent)
    ///
    ///*This API requires the following crate features to be activated: `GamepadButtonEvent`*
    pub type GamepadButtonEvent;

    # [ wasm_bindgen ( structural , method , getter , js_class = "GamepadButtonEvent" , js_name = button ) ]
    ///Getter for the `button` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GamepadButtonEvent/button)
    ///
    ///*This API requires the following crate features to be activated: `GamepadButtonEvent`*
    pub fn button(this: &GamepadButtonEvent) -> u32;

    #[wasm_bindgen(catch, constructor, js_class = "GamepadButtonEvent")]
    ///The `new GamepadButtonEvent(..)` constructor, creating a new instance of `GamepadButtonEvent`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GamepadButtonEvent/GamepadButtonEvent)
    ///
    ///*This API requires the following crate features to be activated: `GamepadButtonEvent`*
    pub fn new(type_: &str) -> Result<GamepadButtonEvent, JsValue>;

    #[cfg(feature = "GamepadButtonEventInit")]
    #[wasm_bindgen(catch, constructor, js_class = "GamepadButtonEvent")]
    ///The `new GamepadButtonEvent(..)` constructor, creating a new instance of `GamepadButtonEvent`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GamepadButtonEvent/GamepadButtonEvent)
    ///
    ///*This API requires the following crate features to be activated: `GamepadButtonEvent`, `GamepadButtonEventInit`*
    pub fn new_with_event_init_dict(
        type_: &str,
        event_init_dict: &GamepadButtonEventInit,
    ) -> Result<GamepadButtonEvent, JsValue>;

}
