use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = VRFieldOfView , typescript_type = "VRFieldOfView" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `VrFieldOfView` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRFieldOfView)
    ///
    ///*This API requires the following crate features to be activated: `VrFieldOfView`*
    pub type VrFieldOfView;

    # [ wasm_bindgen ( structural , method , getter , js_class = "VRFieldOfView" , js_name = upDegrees ) ]
    ///Getter for the `upDegrees` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRFieldOfView/upDegrees)
    ///
    ///*This API requires the following crate features to be activated: `VrFieldOfView`*
    pub fn up_degrees(this: &VrFieldOfView) -> f64;

    # [ wasm_bindgen ( structural , method , getter , js_class = "VRFieldOfView" , js_name = rightDegrees ) ]
    ///Getter for the `rightDegrees` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRFieldOfView/rightDegrees)
    ///
    ///*This API requires the following crate features to be activated: `VrFieldOfView`*
    pub fn right_degrees(this: &VrFieldOfView) -> f64;

    # [ wasm_bindgen ( structural , method , getter , js_class = "VRFieldOfView" , js_name = downDegrees ) ]
    ///Getter for the `downDegrees` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRFieldOfView/downDegrees)
    ///
    ///*This API requires the following crate features to be activated: `VrFieldOfView`*
    pub fn down_degrees(this: &VrFieldOfView) -> f64;

    # [ wasm_bindgen ( structural , method , getter , js_class = "VRFieldOfView" , js_name = leftDegrees ) ]
    ///Getter for the `leftDegrees` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRFieldOfView/leftDegrees)
    ///
    ///*This API requires the following crate features to be activated: `VrFieldOfView`*
    pub fn left_degrees(this: &VrFieldOfView) -> f64;

}
