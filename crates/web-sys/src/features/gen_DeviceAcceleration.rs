use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( is_type_of = | _ | false , extends = :: js_sys :: Object , js_name = DeviceAcceleration , typescript_type = "DeviceAcceleration" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `DeviceAcceleration` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceAcceleration)
    ///
    ///*This API requires the following crate features to be activated: `DeviceAcceleration`*
    pub type DeviceAcceleration;

    # [ wasm_bindgen ( structural , method , getter , js_class = "DeviceAcceleration" , js_name = x ) ]
    ///Getter for the `x` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceAcceleration/x)
    ///
    ///*This API requires the following crate features to be activated: `DeviceAcceleration`*
    pub fn x(this: &DeviceAcceleration) -> Option<f64>;

    # [ wasm_bindgen ( structural , method , getter , js_class = "DeviceAcceleration" , js_name = y ) ]
    ///Getter for the `y` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceAcceleration/y)
    ///
    ///*This API requires the following crate features to be activated: `DeviceAcceleration`*
    pub fn y(this: &DeviceAcceleration) -> Option<f64>;

    # [ wasm_bindgen ( structural , method , getter , js_class = "DeviceAcceleration" , js_name = z ) ]
    ///Getter for the `z` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceAcceleration/z)
    ///
    ///*This API requires the following crate features to be activated: `DeviceAcceleration`*
    pub fn z(this: &DeviceAcceleration) -> Option<f64>;

}
