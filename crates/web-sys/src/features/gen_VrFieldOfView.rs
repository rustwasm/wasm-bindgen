use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = VRFieldOfView , typescript_name = VRFieldOfView ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `VrFieldOfView` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRFieldOfView)\n\n*This API requires the following crate features to be activated: `VrFieldOfView`*"]
    pub type VrFieldOfView;
    # [ wasm_bindgen ( structural , method , getter , js_name = upDegrees ) ]
    #[doc = "Getter for the `upDegrees` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRFieldOfView/upDegrees)\n\n*This API requires the following crate features to be activated: `VrFieldOfView`*"]
    pub fn up_degrees(this: &VrFieldOfView) -> f64;
    # [ wasm_bindgen ( structural , method , getter , js_name = rightDegrees ) ]
    #[doc = "Getter for the `rightDegrees` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRFieldOfView/rightDegrees)\n\n*This API requires the following crate features to be activated: `VrFieldOfView`*"]
    pub fn right_degrees(this: &VrFieldOfView) -> f64;
    # [ wasm_bindgen ( structural , method , getter , js_name = downDegrees ) ]
    #[doc = "Getter for the `downDegrees` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRFieldOfView/downDegrees)\n\n*This API requires the following crate features to be activated: `VrFieldOfView`*"]
    pub fn down_degrees(this: &VrFieldOfView) -> f64;
    # [ wasm_bindgen ( structural , method , getter , js_name = leftDegrees ) ]
    #[doc = "Getter for the `leftDegrees` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRFieldOfView/leftDegrees)\n\n*This API requires the following crate features to be activated: `VrFieldOfView`*"]
    pub fn left_degrees(this: &VrFieldOfView) -> f64;
}
