use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = VRStageParameters , typescript_type = "VRStageParameters" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `VrStageParameters` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRStageParameters)
    ///
    ///*This API requires the following crate features to be activated: `VrStageParameters`*
    pub type VrStageParameters;

    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "VRStageParameters" , js_name = sittingToStandingTransform ) ]
    ///Getter for the `sittingToStandingTransform` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRStageParameters/sittingToStandingTransform)
    ///
    ///*This API requires the following crate features to be activated: `VrStageParameters`*
    pub fn sitting_to_standing_transform(this: &VrStageParameters) -> Result<Vec<f32>, JsValue>;

    # [ wasm_bindgen ( structural , method , getter , js_class = "VRStageParameters" , js_name = sizeX ) ]
    ///Getter for the `sizeX` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRStageParameters/sizeX)
    ///
    ///*This API requires the following crate features to be activated: `VrStageParameters`*
    pub fn size_x(this: &VrStageParameters) -> f32;

    # [ wasm_bindgen ( structural , method , getter , js_class = "VRStageParameters" , js_name = sizeZ ) ]
    ///Getter for the `sizeZ` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRStageParameters/sizeZ)
    ///
    ///*This API requires the following crate features to be activated: `VrStageParameters`*
    pub fn size_z(this: &VrStageParameters) -> f32;

}
