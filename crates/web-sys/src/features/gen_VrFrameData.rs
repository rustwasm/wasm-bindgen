use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = VRFrameData , typescript_name = VRFrameData ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `VrFrameData` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRFrameData)\n\n*This API requires the following crate features to be activated: `VrFrameData`*"]
    pub type VrFrameData;
    # [ wasm_bindgen ( structural , method , getter , js_name = timestamp ) ]
    #[doc = "Getter for the `timestamp` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRFrameData/timestamp)\n\n*This API requires the following crate features to be activated: `VrFrameData`*"]
    pub fn timestamp(this: &VrFrameData) -> f64;
    # [ wasm_bindgen ( structural , catch , method , getter , js_name = leftProjectionMatrix ) ]
    #[doc = "Getter for the `leftProjectionMatrix` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRFrameData/leftProjectionMatrix)\n\n*This API requires the following crate features to be activated: `VrFrameData`*"]
    pub fn left_projection_matrix(this: &VrFrameData) -> Result<Vec<f32>, JsValue>;
    # [ wasm_bindgen ( structural , catch , method , getter , js_name = leftViewMatrix ) ]
    #[doc = "Getter for the `leftViewMatrix` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRFrameData/leftViewMatrix)\n\n*This API requires the following crate features to be activated: `VrFrameData`*"]
    pub fn left_view_matrix(this: &VrFrameData) -> Result<Vec<f32>, JsValue>;
    # [ wasm_bindgen ( structural , catch , method , getter , js_name = rightProjectionMatrix ) ]
    #[doc = "Getter for the `rightProjectionMatrix` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRFrameData/rightProjectionMatrix)\n\n*This API requires the following crate features to be activated: `VrFrameData`*"]
    pub fn right_projection_matrix(this: &VrFrameData) -> Result<Vec<f32>, JsValue>;
    # [ wasm_bindgen ( structural , catch , method , getter , js_name = rightViewMatrix ) ]
    #[doc = "Getter for the `rightViewMatrix` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRFrameData/rightViewMatrix)\n\n*This API requires the following crate features to be activated: `VrFrameData`*"]
    pub fn right_view_matrix(this: &VrFrameData) -> Result<Vec<f32>, JsValue>;
    # [ wasm_bindgen ( structural , method , getter , js_name = pose ) ]
    #[cfg(feature = "VrPose")]
    #[doc = "Getter for the `pose` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRFrameData/pose)\n\n*This API requires the following crate features to be activated: `VrFrameData`, `VrPose`*"]
    pub fn pose(this: &VrFrameData) -> VrPose;
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new VrFrameData(..)` constructor, creating a new instance of `VrFrameData`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRFrameData/VRFrameData)\n\n*This API requires the following crate features to be activated: `VrFrameData`*"]
    pub fn new(this: &VrFrameData) -> Result<VrFrameData, JsValue>;
}
