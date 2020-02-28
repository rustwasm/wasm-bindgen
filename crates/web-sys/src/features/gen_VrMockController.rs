use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = VRMockController , typescript_name = VRMockController ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `VrMockController` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRMockController)\n\n*This API requires the following crate features to be activated: `VrMockController`*"]
    pub type VrMockController;
    # [ wasm_bindgen ( method , structural , js_name = newAxisMoveEvent ) ]
    #[doc = "The `newAxisMoveEvent()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRMockController/newAxisMoveEvent)\n\n*This API requires the following crate features to be activated: `VrMockController`*"]
    pub fn new_axis_move_event(this: &VrMockController, axis: u32, value: f64);
    # [ wasm_bindgen ( method , structural , js_name = newButtonEvent ) ]
    #[doc = "The `newButtonEvent()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRMockController/newButtonEvent)\n\n*This API requires the following crate features to be activated: `VrMockController`*"]
    pub fn new_button_event(this: &VrMockController, button: u32, pressed: bool);
    # [ wasm_bindgen ( method , structural , js_name = newPoseMove ) ]
    #[doc = "The `newPoseMove()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRMockController/newPoseMove)\n\n*This API requires the following crate features to be activated: `VrMockController`*"]
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
