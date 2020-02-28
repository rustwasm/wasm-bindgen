use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = GamepadServiceTest , typescript_name = GamepadServiceTest ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GamepadServiceTest` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GamepadServiceTest)\n\n*This API requires the following crate features to be activated: `GamepadServiceTest`*"]
    pub type GamepadServiceTest;
    # [ wasm_bindgen ( structural , method , getter , js_name = noMapping ) ]
    #[cfg(feature = "GamepadMappingType")]
    #[doc = "Getter for the `noMapping` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GamepadServiceTest/noMapping)\n\n*This API requires the following crate features to be activated: `GamepadMappingType`, `GamepadServiceTest`*"]
    pub fn no_mapping(this: &GamepadServiceTest) -> GamepadMappingType;
    # [ wasm_bindgen ( structural , method , getter , js_name = standardMapping ) ]
    #[cfg(feature = "GamepadMappingType")]
    #[doc = "Getter for the `standardMapping` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GamepadServiceTest/standardMapping)\n\n*This API requires the following crate features to be activated: `GamepadMappingType`, `GamepadServiceTest`*"]
    pub fn standard_mapping(this: &GamepadServiceTest) -> GamepadMappingType;
    # [ wasm_bindgen ( structural , method , getter , js_name = noHand ) ]
    #[cfg(feature = "GamepadHand")]
    #[doc = "Getter for the `noHand` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GamepadServiceTest/noHand)\n\n*This API requires the following crate features to be activated: `GamepadHand`, `GamepadServiceTest`*"]
    pub fn no_hand(this: &GamepadServiceTest) -> GamepadHand;
    # [ wasm_bindgen ( structural , method , getter , js_name = leftHand ) ]
    #[cfg(feature = "GamepadHand")]
    #[doc = "Getter for the `leftHand` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GamepadServiceTest/leftHand)\n\n*This API requires the following crate features to be activated: `GamepadHand`, `GamepadServiceTest`*"]
    pub fn left_hand(this: &GamepadServiceTest) -> GamepadHand;
    # [ wasm_bindgen ( structural , method , getter , js_name = rightHand ) ]
    #[cfg(feature = "GamepadHand")]
    #[doc = "Getter for the `rightHand` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GamepadServiceTest/rightHand)\n\n*This API requires the following crate features to be activated: `GamepadHand`, `GamepadServiceTest`*"]
    pub fn right_hand(this: &GamepadServiceTest) -> GamepadHand;
    #[cfg(all(feature = "GamepadHand", feature = "GamepadMappingType",))]
    # [ wasm_bindgen ( catch , method , structural , js_name = addGamepad ) ]
    #[doc = "The `addGamepad()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GamepadServiceTest/addGamepad)\n\n*This API requires the following crate features to be activated: `GamepadHand`, `GamepadMappingType`, `GamepadServiceTest`*"]
    pub fn add_gamepad(
        this: &GamepadServiceTest,
        id: &str,
        mapping: GamepadMappingType,
        hand: GamepadHand,
        num_buttons: u32,
        num_axes: u32,
        num_haptics: u32,
    ) -> Result<::js_sys::Promise, JsValue>;
    # [ wasm_bindgen ( method , structural , js_name = newAxisMoveEvent ) ]
    #[doc = "The `newAxisMoveEvent()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GamepadServiceTest/newAxisMoveEvent)\n\n*This API requires the following crate features to be activated: `GamepadServiceTest`*"]
    pub fn new_axis_move_event(this: &GamepadServiceTest, index: u32, axis: u32, value: f64);
    # [ wasm_bindgen ( method , structural , js_name = newButtonEvent ) ]
    #[doc = "The `newButtonEvent()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GamepadServiceTest/newButtonEvent)\n\n*This API requires the following crate features to be activated: `GamepadServiceTest`*"]
    pub fn new_button_event(
        this: &GamepadServiceTest,
        index: u32,
        button: u32,
        pressed: bool,
        touched: bool,
    );
    # [ wasm_bindgen ( method , structural , js_name = newButtonValueEvent ) ]
    #[doc = "The `newButtonValueEvent()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GamepadServiceTest/newButtonValueEvent)\n\n*This API requires the following crate features to be activated: `GamepadServiceTest`*"]
    pub fn new_button_value_event(
        this: &GamepadServiceTest,
        index: u32,
        button: u32,
        pressed: bool,
        touched: bool,
        value: f64,
    );
    # [ wasm_bindgen ( method , structural , js_name = newPoseMove ) ]
    #[doc = "The `newPoseMove()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GamepadServiceTest/newPoseMove)\n\n*This API requires the following crate features to be activated: `GamepadServiceTest`*"]
    pub fn new_pose_move(
        this: &GamepadServiceTest,
        index: u32,
        orient: Option<&mut [f32]>,
        pos: Option<&mut [f32]>,
        ang_velocity: Option<&mut [f32]>,
        ang_acceleration: Option<&mut [f32]>,
        lin_velocity: Option<&mut [f32]>,
        lin_acceleration: Option<&mut [f32]>,
    );
    # [ wasm_bindgen ( method , structural , js_name = removeGamepad ) ]
    #[doc = "The `removeGamepad()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GamepadServiceTest/removeGamepad)\n\n*This API requires the following crate features to be activated: `GamepadServiceTest`*"]
    pub fn remove_gamepad(this: &GamepadServiceTest, index: u32);
}
