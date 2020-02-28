use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = Event , extends = :: js_sys :: Object , js_name = GamepadEvent , typescript_name = GamepadEvent ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GamepadEvent` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GamepadEvent)\n\n*This API requires the following crate features to be activated: `GamepadEvent`*"]
    pub type GamepadEvent;
    # [ wasm_bindgen ( structural , method , getter , js_name = gamepad ) ]
    #[cfg(feature = "Gamepad")]
    #[doc = "Getter for the `gamepad` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GamepadEvent/gamepad)\n\n*This API requires the following crate features to be activated: `Gamepad`, `GamepadEvent`*"]
    pub fn gamepad(this: &GamepadEvent) -> Option<Gamepad>;
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new GamepadEvent(..)` constructor, creating a new instance of `GamepadEvent`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GamepadEvent/GamepadEvent)\n\n*This API requires the following crate features to be activated: `GamepadEvent`*"]
    pub fn new(this: &GamepadEvent, type_: &str) -> Result<GamepadEvent, JsValue>;
    #[cfg(feature = "GamepadEventInit")]
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new GamepadEvent(..)` constructor, creating a new instance of `GamepadEvent`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GamepadEvent/GamepadEvent)\n\n*This API requires the following crate features to be activated: `GamepadEvent`, `GamepadEventInit`*"]
    pub fn new_with_event_init_dict(
        this: &GamepadEvent,
        type_: &str,
        event_init_dict: &GamepadEventInit,
    ) -> Result<GamepadEvent, JsValue>;
}
