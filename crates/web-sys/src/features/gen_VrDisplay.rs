use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = EventTarget , extends = :: js_sys :: Object , js_name = VRDisplay , typescript_type = "VRDisplay" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `VrDisplay` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRDisplay)
    ///
    ///*This API requires the following crate features to be activated: `VrDisplay`*
    pub type VrDisplay;

    # [ wasm_bindgen ( structural , method , getter , js_class = "VRDisplay" , js_name = isConnected ) ]
    ///Getter for the `isConnected` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRDisplay/isConnected)
    ///
    ///*This API requires the following crate features to be activated: `VrDisplay`*
    pub fn is_connected(this: &VrDisplay) -> bool;

    # [ wasm_bindgen ( structural , method , getter , js_class = "VRDisplay" , js_name = isPresenting ) ]
    ///Getter for the `isPresenting` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRDisplay/isPresenting)
    ///
    ///*This API requires the following crate features to be activated: `VrDisplay`*
    pub fn is_presenting(this: &VrDisplay) -> bool;

    #[cfg(feature = "VrDisplayCapabilities")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "VRDisplay" , js_name = capabilities ) ]
    ///Getter for the `capabilities` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRDisplay/capabilities)
    ///
    ///*This API requires the following crate features to be activated: `VrDisplay`, `VrDisplayCapabilities`*
    pub fn capabilities(this: &VrDisplay) -> VrDisplayCapabilities;

    #[cfg(feature = "VrStageParameters")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "VRDisplay" , js_name = stageParameters ) ]
    ///Getter for the `stageParameters` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRDisplay/stageParameters)
    ///
    ///*This API requires the following crate features to be activated: `VrDisplay`, `VrStageParameters`*
    pub fn stage_parameters(this: &VrDisplay) -> Option<VrStageParameters>;

    # [ wasm_bindgen ( structural , method , getter , js_class = "VRDisplay" , js_name = displayId ) ]
    ///Getter for the `displayId` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRDisplay/displayId)
    ///
    ///*This API requires the following crate features to be activated: `VrDisplay`*
    pub fn display_id(this: &VrDisplay) -> u32;

    # [ wasm_bindgen ( structural , method , getter , js_class = "VRDisplay" , js_name = displayName ) ]
    ///Getter for the `displayName` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRDisplay/displayName)
    ///
    ///*This API requires the following crate features to be activated: `VrDisplay`*
    pub fn display_name(this: &VrDisplay) -> String;

    # [ wasm_bindgen ( structural , method , getter , js_class = "VRDisplay" , js_name = depthNear ) ]
    ///Getter for the `depthNear` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRDisplay/depthNear)
    ///
    ///*This API requires the following crate features to be activated: `VrDisplay`*
    pub fn depth_near(this: &VrDisplay) -> f64;

    # [ wasm_bindgen ( structural , method , setter , js_class = "VRDisplay" , js_name = depthNear ) ]
    ///Setter for the `depthNear` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRDisplay/depthNear)
    ///
    ///*This API requires the following crate features to be activated: `VrDisplay`*
    pub fn set_depth_near(this: &VrDisplay, value: f64);

    # [ wasm_bindgen ( structural , method , getter , js_class = "VRDisplay" , js_name = depthFar ) ]
    ///Getter for the `depthFar` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRDisplay/depthFar)
    ///
    ///*This API requires the following crate features to be activated: `VrDisplay`*
    pub fn depth_far(this: &VrDisplay) -> f64;

    # [ wasm_bindgen ( structural , method , setter , js_class = "VRDisplay" , js_name = depthFar ) ]
    ///Setter for the `depthFar` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRDisplay/depthFar)
    ///
    ///*This API requires the following crate features to be activated: `VrDisplay`*
    pub fn set_depth_far(this: &VrDisplay, value: f64);

    # [ wasm_bindgen ( catch , method , structural , js_class = "VRDisplay" , js_name = cancelAnimationFrame ) ]
    ///The `cancelAnimationFrame()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRDisplay/cancelAnimationFrame)
    ///
    ///*This API requires the following crate features to be activated: `VrDisplay`*
    pub fn cancel_animation_frame(this: &VrDisplay, handle: i32) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "VRDisplay" , js_name = exitPresent ) ]
    ///The `exitPresent()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRDisplay/exitPresent)
    ///
    ///*This API requires the following crate features to be activated: `VrDisplay`*
    pub fn exit_present(this: &VrDisplay) -> Result<::js_sys::Promise, JsValue>;

    #[cfg(all(feature = "VrEye", feature = "VrEyeParameters",))]
    # [ wasm_bindgen ( method , structural , js_class = "VRDisplay" , js_name = getEyeParameters ) ]
    ///The `getEyeParameters()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRDisplay/getEyeParameters)
    ///
    ///*This API requires the following crate features to be activated: `VrDisplay`, `VrEye`, `VrEyeParameters`*
    pub fn get_eye_parameters(this: &VrDisplay, which_eye: VrEye) -> VrEyeParameters;

    #[cfg(feature = "VrFrameData")]
    # [ wasm_bindgen ( method , structural , js_class = "VRDisplay" , js_name = getFrameData ) ]
    ///The `getFrameData()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRDisplay/getFrameData)
    ///
    ///*This API requires the following crate features to be activated: `VrDisplay`, `VrFrameData`*
    pub fn get_frame_data(this: &VrDisplay, frame_data: &VrFrameData) -> bool;

    # [ wasm_bindgen ( method , structural , js_class = "VRDisplay" , js_name = getLayers ) ]
    ///The `getLayers()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRDisplay/getLayers)
    ///
    ///*This API requires the following crate features to be activated: `VrDisplay`*
    pub fn get_layers(this: &VrDisplay) -> ::js_sys::Array;

    #[cfg(feature = "VrPose")]
    # [ wasm_bindgen ( method , structural , js_class = "VRDisplay" , js_name = getPose ) ]
    ///The `getPose()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRDisplay/getPose)
    ///
    ///*This API requires the following crate features to be activated: `VrDisplay`, `VrPose`*
    pub fn get_pose(this: &VrDisplay) -> VrPose;

    #[cfg(feature = "VrSubmitFrameResult")]
    # [ wasm_bindgen ( method , structural , js_class = "VRDisplay" , js_name = getSubmitFrameResult ) ]
    ///The `getSubmitFrameResult()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRDisplay/getSubmitFrameResult)
    ///
    ///*This API requires the following crate features to be activated: `VrDisplay`, `VrSubmitFrameResult`*
    pub fn get_submit_frame_result(this: &VrDisplay, result: &VrSubmitFrameResult) -> bool;

    # [ wasm_bindgen ( catch , method , structural , js_class = "VRDisplay" , js_name = requestAnimationFrame ) ]
    ///The `requestAnimationFrame()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRDisplay/requestAnimationFrame)
    ///
    ///*This API requires the following crate features to be activated: `VrDisplay`*
    pub fn request_animation_frame(
        this: &VrDisplay,
        callback: &::js_sys::Function,
    ) -> Result<i32, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "VRDisplay" , js_name = requestPresent ) ]
    ///The `requestPresent()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRDisplay/requestPresent)
    ///
    ///*This API requires the following crate features to be activated: `VrDisplay`*
    pub fn request_present(
        this: &VrDisplay,
        layers: &::wasm_bindgen::JsValue,
    ) -> Result<::js_sys::Promise, JsValue>;

    # [ wasm_bindgen ( method , structural , js_class = "VRDisplay" , js_name = resetPose ) ]
    ///The `resetPose()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRDisplay/resetPose)
    ///
    ///*This API requires the following crate features to be activated: `VrDisplay`*
    pub fn reset_pose(this: &VrDisplay);

    # [ wasm_bindgen ( method , structural , js_class = "VRDisplay" , js_name = submitFrame ) ]
    ///The `submitFrame()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRDisplay/submitFrame)
    ///
    ///*This API requires the following crate features to be activated: `VrDisplay`*
    pub fn submit_frame(this: &VrDisplay);

}
