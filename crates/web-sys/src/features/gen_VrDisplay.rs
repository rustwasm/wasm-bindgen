use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = EventTarget , extends = :: js_sys :: Object , js_name = VRDisplay , typescript_name = VRDisplay ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `VrDisplay` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRDisplay)\n\n*This API requires the following crate features to be activated: `VrDisplay`*"]
    pub type VrDisplay;
    # [ wasm_bindgen ( structural , method , getter , js_name = isConnected ) ]
    #[doc = "Getter for the `isConnected` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRDisplay/isConnected)\n\n*This API requires the following crate features to be activated: `VrDisplay`*"]
    pub fn is_connected(this: &VrDisplay) -> bool;
    # [ wasm_bindgen ( structural , method , getter , js_name = isPresenting ) ]
    #[doc = "Getter for the `isPresenting` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRDisplay/isPresenting)\n\n*This API requires the following crate features to be activated: `VrDisplay`*"]
    pub fn is_presenting(this: &VrDisplay) -> bool;
    # [ wasm_bindgen ( structural , method , getter , js_name = capabilities ) ]
    #[cfg(feature = "VrDisplayCapabilities")]
    #[doc = "Getter for the `capabilities` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRDisplay/capabilities)\n\n*This API requires the following crate features to be activated: `VrDisplay`, `VrDisplayCapabilities`*"]
    pub fn capabilities(this: &VrDisplay) -> VrDisplayCapabilities;
    # [ wasm_bindgen ( structural , method , getter , js_name = stageParameters ) ]
    #[cfg(feature = "VrStageParameters")]
    #[doc = "Getter for the `stageParameters` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRDisplay/stageParameters)\n\n*This API requires the following crate features to be activated: `VrDisplay`, `VrStageParameters`*"]
    pub fn stage_parameters(this: &VrDisplay) -> Option<VrStageParameters>;
    # [ wasm_bindgen ( structural , method , getter , js_name = displayId ) ]
    #[doc = "Getter for the `displayId` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRDisplay/displayId)\n\n*This API requires the following crate features to be activated: `VrDisplay`*"]
    pub fn display_id(this: &VrDisplay) -> u32;
    # [ wasm_bindgen ( structural , method , getter , js_name = displayName ) ]
    #[doc = "Getter for the `displayName` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRDisplay/displayName)\n\n*This API requires the following crate features to be activated: `VrDisplay`*"]
    pub fn display_name(this: &VrDisplay) -> String;
    # [ wasm_bindgen ( structural , method , getter , js_name = depthNear ) ]
    #[doc = "Getter for the `depthNear` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRDisplay/depthNear)\n\n*This API requires the following crate features to be activated: `VrDisplay`*"]
    pub fn depth_near(this: &VrDisplay) -> f64;
    # [ wasm_bindgen ( structural , method , setter , js_name = depthNear ) ]
    #[doc = "Setter for the `depthNear` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRDisplay/depthNear)\n\n*This API requires the following crate features to be activated: `VrDisplay`*"]
    pub fn set_depth_near(this: &VrDisplay, value: f64);
    # [ wasm_bindgen ( structural , method , getter , js_name = depthFar ) ]
    #[doc = "Getter for the `depthFar` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRDisplay/depthFar)\n\n*This API requires the following crate features to be activated: `VrDisplay`*"]
    pub fn depth_far(this: &VrDisplay) -> f64;
    # [ wasm_bindgen ( structural , method , setter , js_name = depthFar ) ]
    #[doc = "Setter for the `depthFar` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRDisplay/depthFar)\n\n*This API requires the following crate features to be activated: `VrDisplay`*"]
    pub fn set_depth_far(this: &VrDisplay, value: f64);
    # [ wasm_bindgen ( catch , method , structural , js_name = cancelAnimationFrame ) ]
    #[doc = "The `cancelAnimationFrame()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRDisplay/cancelAnimationFrame)\n\n*This API requires the following crate features to be activated: `VrDisplay`*"]
    pub fn cancel_animation_frame(this: &VrDisplay, handle: i32) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = exitPresent ) ]
    #[doc = "The `exitPresent()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRDisplay/exitPresent)\n\n*This API requires the following crate features to be activated: `VrDisplay`*"]
    pub fn exit_present(this: &VrDisplay) -> Result<::js_sys::Promise, JsValue>;
    #[cfg(all(feature = "VrEye", feature = "VrEyeParameters",))]
    # [ wasm_bindgen ( method , structural , js_name = getEyeParameters ) ]
    #[doc = "The `getEyeParameters()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRDisplay/getEyeParameters)\n\n*This API requires the following crate features to be activated: `VrDisplay`, `VrEye`, `VrEyeParameters`*"]
    pub fn get_eye_parameters(this: &VrDisplay, which_eye: VrEye) -> VrEyeParameters;
    #[cfg(feature = "VrFrameData")]
    # [ wasm_bindgen ( method , structural , js_name = getFrameData ) ]
    #[doc = "The `getFrameData()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRDisplay/getFrameData)\n\n*This API requires the following crate features to be activated: `VrDisplay`, `VrFrameData`*"]
    pub fn get_frame_data(this: &VrDisplay, frame_data: &VrFrameData) -> bool;
    # [ wasm_bindgen ( method , structural , js_name = getLayers ) ]
    #[doc = "The `getLayers()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRDisplay/getLayers)\n\n*This API requires the following crate features to be activated: `VrDisplay`*"]
    pub fn get_layers(this: &VrDisplay) -> ::js_sys::Array;
    #[cfg(feature = "VrPose")]
    # [ wasm_bindgen ( method , structural , js_name = getPose ) ]
    #[doc = "The `getPose()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRDisplay/getPose)\n\n*This API requires the following crate features to be activated: `VrDisplay`, `VrPose`*"]
    pub fn get_pose(this: &VrDisplay) -> VrPose;
    #[cfg(feature = "VrSubmitFrameResult")]
    # [ wasm_bindgen ( method , structural , js_name = getSubmitFrameResult ) ]
    #[doc = "The `getSubmitFrameResult()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRDisplay/getSubmitFrameResult)\n\n*This API requires the following crate features to be activated: `VrDisplay`, `VrSubmitFrameResult`*"]
    pub fn get_submit_frame_result(this: &VrDisplay, result: &VrSubmitFrameResult) -> bool;
    # [ wasm_bindgen ( catch , method , structural , js_name = requestAnimationFrame ) ]
    #[doc = "The `requestAnimationFrame()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRDisplay/requestAnimationFrame)\n\n*This API requires the following crate features to be activated: `VrDisplay`*"]
    pub fn request_animation_frame(
        this: &VrDisplay,
        callback: &::js_sys::Function,
    ) -> Result<i32, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = requestPresent ) ]
    #[doc = "The `requestPresent()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRDisplay/requestPresent)\n\n*This API requires the following crate features to be activated: `VrDisplay`*"]
    pub fn request_present(
        this: &VrDisplay,
        layers: &::wasm_bindgen::JsValue,
    ) -> Result<::js_sys::Promise, JsValue>;
    # [ wasm_bindgen ( method , structural , js_name = resetPose ) ]
    #[doc = "The `resetPose()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRDisplay/resetPose)\n\n*This API requires the following crate features to be activated: `VrDisplay`*"]
    pub fn reset_pose(this: &VrDisplay);
    # [ wasm_bindgen ( method , structural , js_name = submitFrame ) ]
    #[doc = "The `submitFrame()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRDisplay/submitFrame)\n\n*This API requires the following crate features to be activated: `VrDisplay`*"]
    pub fn submit_frame(this: &VrDisplay);
}
