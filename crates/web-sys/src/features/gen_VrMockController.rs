use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = VRMockController , typescript_type = "VRMockController" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `VrMockController` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRMockController)
    ///
    ///*This API requires the following crate features to be activated: `VrMockController`*
    pub type VrMockController;

    # [ wasm_bindgen ( method , structural , js_class = "VRMockController" , js_name = newAxisMoveEvent ) ]
    ///The `newAxisMoveEvent()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRMockController/newAxisMoveEvent)
    ///
    ///*This API requires the following crate features to be activated: `VrMockController`*
    pub fn new_axis_move_event(this: &VrMockController, axis: u32, value: f64);

    # [ wasm_bindgen ( method , structural , js_class = "VRMockController" , js_name = newButtonEvent ) ]
    ///The `newButtonEvent()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRMockController/newButtonEvent)
    ///
    ///*This API requires the following crate features to be activated: `VrMockController`*
    pub fn new_button_event(this: &VrMockController, button: u32, pressed: bool);

    # [ wasm_bindgen ( method , structural , js_class = "VRMockController" , js_name = newPoseMove ) ]
    ///The `newPoseMove()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRMockController/newPoseMove)
    ///
    ///*This API requires the following crate features to be activated: `VrMockController`*
    pub fn new_pose_move(
        this: &VrMockController,
        position: Option<&mut [f32]>,
        linear_velocity: Option<&mut [f32]>,
        linear_acceleration: Option<&mut [f32]>,
        orientation: Option<&mut [f32]>,
        angular_velocity: Option<&mut [f32]>,
        angular_acceleration: Option<&mut [f32]>,
    );

}
