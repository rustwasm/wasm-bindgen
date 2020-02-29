use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = VRFrameData , typescript_type = "VRFrameData" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `VrFrameData` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRFrameData)
    ///
    ///*This API requires the following crate features to be activated: `VrFrameData`*
    pub type VrFrameData;

    # [ wasm_bindgen ( structural , method , getter , js_class = "VRFrameData" , js_name = timestamp ) ]
    ///Getter for the `timestamp` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRFrameData/timestamp)
    ///
    ///*This API requires the following crate features to be activated: `VrFrameData`*
    pub fn timestamp(this: &VrFrameData) -> f64;

    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "VRFrameData" , js_name = leftProjectionMatrix ) ]
    ///Getter for the `leftProjectionMatrix` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRFrameData/leftProjectionMatrix)
    ///
    ///*This API requires the following crate features to be activated: `VrFrameData`*
    pub fn left_projection_matrix(this: &VrFrameData) -> Result<Vec<f32>, JsValue>;

    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "VRFrameData" , js_name = leftViewMatrix ) ]
    ///Getter for the `leftViewMatrix` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRFrameData/leftViewMatrix)
    ///
    ///*This API requires the following crate features to be activated: `VrFrameData`*
    pub fn left_view_matrix(this: &VrFrameData) -> Result<Vec<f32>, JsValue>;

    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "VRFrameData" , js_name = rightProjectionMatrix ) ]
    ///Getter for the `rightProjectionMatrix` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRFrameData/rightProjectionMatrix)
    ///
    ///*This API requires the following crate features to be activated: `VrFrameData`*
    pub fn right_projection_matrix(this: &VrFrameData) -> Result<Vec<f32>, JsValue>;

    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "VRFrameData" , js_name = rightViewMatrix ) ]
    ///Getter for the `rightViewMatrix` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRFrameData/rightViewMatrix)
    ///
    ///*This API requires the following crate features to be activated: `VrFrameData`*
    pub fn right_view_matrix(this: &VrFrameData) -> Result<Vec<f32>, JsValue>;

    #[cfg(feature = "VrPose")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "VRFrameData" , js_name = pose ) ]
    ///Getter for the `pose` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRFrameData/pose)
    ///
    ///*This API requires the following crate features to be activated: `VrFrameData`, `VrPose`*
    pub fn pose(this: &VrFrameData) -> VrPose;

    #[wasm_bindgen(catch, constructor, js_class = "VRFrameData")]
    ///The `new VrFrameData(..)` constructor, creating a new instance of `VrFrameData`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRFrameData/VRFrameData)
    ///
    ///*This API requires the following crate features to be activated: `VrFrameData`*
    pub fn new() -> Result<VrFrameData, JsValue>;

}
