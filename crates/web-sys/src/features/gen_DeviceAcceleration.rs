use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( is_type_of = | _ | false , extends = :: js_sys :: Object , js_name = DeviceAcceleration , typescript_name = DeviceAcceleration ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DeviceAcceleration` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceAcceleration)\n\n*This API requires the following crate features to be activated: `DeviceAcceleration`*"]
    pub type DeviceAcceleration;
    # [ wasm_bindgen ( structural , method , getter , js_name = x ) ]
    #[doc = "Getter for the `x` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceAcceleration/x)\n\n*This API requires the following crate features to be activated: `DeviceAcceleration`*"]
    pub fn x(this: &DeviceAcceleration) -> Option<f64>;
    # [ wasm_bindgen ( structural , method , getter , js_name = y ) ]
    #[doc = "Getter for the `y` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceAcceleration/y)\n\n*This API requires the following crate features to be activated: `DeviceAcceleration`*"]
    pub fn y(this: &DeviceAcceleration) -> Option<f64>;
    # [ wasm_bindgen ( structural , method , getter , js_name = z ) ]
    #[doc = "Getter for the `z` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceAcceleration/z)\n\n*This API requires the following crate features to be activated: `DeviceAcceleration`*"]
    pub fn z(this: &DeviceAcceleration) -> Option<f64>;
}
