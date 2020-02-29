use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = GamepadServiceTest , typescript_name = GamepadServiceTest ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `GamepadServiceTest` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GamepadServiceTest)
    ///
    ///*This API requires the following crate features to be activated: `GamepadServiceTest`*
    pub type GamepadServiceTest;

    #[cfg(feature = "GamepadMappingType")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "GamepadServiceTest" , js_name = noMapping ) ]
    ///Getter for the `noMapping` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GamepadServiceTest/noMapping)
    ///
    ///*This API requires the following crate features to be activated: `GamepadMappingType`, `GamepadServiceTest`*
    pub fn no_mapping(this: &GamepadServiceTest) -> GamepadMappingType;

    #[cfg(feature = "GamepadMappingType")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "GamepadServiceTest" , js_name = standardMapping ) ]
    ///Getter for the `standardMapping` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GamepadServiceTest/standardMapping)
    ///
    ///*This API requires the following crate features to be activated: `GamepadMappingType`, `GamepadServiceTest`*
    pub fn standard_mapping(this: &GamepadServiceTest) -> GamepadMappingType;

    #[cfg(feature = "GamepadHand")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "GamepadServiceTest" , js_name = noHand ) ]
    ///Getter for the `noHand` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GamepadServiceTest/noHand)
    ///
    ///*This API requires the following crate features to be activated: `GamepadHand`, `GamepadServiceTest`*
    pub fn no_hand(this: &GamepadServiceTest) -> GamepadHand;

    #[cfg(feature = "GamepadHand")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "GamepadServiceTest" , js_name = leftHand ) ]
    ///Getter for the `leftHand` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GamepadServiceTest/leftHand)
    ///
    ///*This API requires the following crate features to be activated: `GamepadHand`, `GamepadServiceTest`*
    pub fn left_hand(this: &GamepadServiceTest) -> GamepadHand;

    #[cfg(feature = "GamepadHand")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "GamepadServiceTest" , js_name = rightHand ) ]
    ///Getter for the `rightHand` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GamepadServiceTest/rightHand)
    ///
    ///*This API requires the following crate features to be activated: `GamepadHand`, `GamepadServiceTest`*
    pub fn right_hand(this: &GamepadServiceTest) -> GamepadHand;

    #[cfg(all(feature = "GamepadHand", feature = "GamepadMappingType",))]
    # [ wasm_bindgen ( catch , method , structural , js_class = "GamepadServiceTest" , js_name = addGamepad ) ]
    ///The `addGamepad()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GamepadServiceTest/addGamepad)
    ///
    ///*This API requires the following crate features to be activated: `GamepadHand`, `GamepadMappingType`, `GamepadServiceTest`*
    pub fn add_gamepad(
        this: &GamepadServiceTest,
        id: &str,
        mapping: GamepadMappingType,
        hand: GamepadHand,
        num_buttons: u32,
        num_axes: u32,
        num_haptics: u32,
    ) -> Result<::js_sys::Promise, JsValue>;

    # [ wasm_bindgen ( method , structural , js_class = "GamepadServiceTest" , js_name = newAxisMoveEvent ) ]
    ///The `newAxisMoveEvent()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GamepadServiceTest/newAxisMoveEvent)
    ///
    ///*This API requires the following crate features to be activated: `GamepadServiceTest`*
    pub fn new_axis_move_event(this: &GamepadServiceTest, index: u32, axis: u32, value: f64);

    # [ wasm_bindgen ( method , structural , js_class = "GamepadServiceTest" , js_name = newButtonEvent ) ]
    ///The `newButtonEvent()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GamepadServiceTest/newButtonEvent)
    ///
    ///*This API requires the following crate features to be activated: `GamepadServiceTest`*
    pub fn new_button_event(
        this: &GamepadServiceTest,
        index: u32,
        button: u32,
        pressed: bool,
        touched: bool,
    );

    # [ wasm_bindgen ( method , structural , js_class = "GamepadServiceTest" , js_name = newButtonValueEvent ) ]
    ///The `newButtonValueEvent()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GamepadServiceTest/newButtonValueEvent)
    ///
    ///*This API requires the following crate features to be activated: `GamepadServiceTest`*
    pub fn new_button_value_event(
        this: &GamepadServiceTest,
        index: u32,
        button: u32,
        pressed: bool,
        touched: bool,
        value: f64,
    );

    # [ wasm_bindgen ( method , structural , js_class = "GamepadServiceTest" , js_name = newPoseMove ) ]
    ///The `newPoseMove()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GamepadServiceTest/newPoseMove)
    ///
    ///*This API requires the following crate features to be activated: `GamepadServiceTest`*
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

    # [ wasm_bindgen ( method , structural , js_class = "GamepadServiceTest" , js_name = removeGamepad ) ]
    ///The `removeGamepad()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GamepadServiceTest/removeGamepad)
    ///
    ///*This API requires the following crate features to be activated: `GamepadServiceTest`*
    pub fn remove_gamepad(this: &GamepadServiceTest, index: u32);

}
