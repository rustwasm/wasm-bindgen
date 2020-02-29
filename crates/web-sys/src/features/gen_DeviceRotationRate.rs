use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( is_type_of = | _ | false , extends = :: js_sys :: Object , js_name = DeviceRotationRate , typescript_name = DeviceRotationRate ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `DeviceRotationRate` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceRotationRate)
    ///
    ///*This API requires the following crate features to be activated: `DeviceRotationRate`*
    pub type DeviceRotationRate;

    # [ wasm_bindgen ( structural , method , getter , js_class = "DeviceRotationRate" , js_name = alpha ) ]
    ///Getter for the `alpha` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceRotationRate/alpha)
    ///
    ///*This API requires the following crate features to be activated: `DeviceRotationRate`*
    pub fn alpha(this: &DeviceRotationRate) -> Option<f64>;

    # [ wasm_bindgen ( structural , method , getter , js_class = "DeviceRotationRate" , js_name = beta ) ]
    ///Getter for the `beta` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceRotationRate/beta)
    ///
    ///*This API requires the following crate features to be activated: `DeviceRotationRate`*
    pub fn beta(this: &DeviceRotationRate) -> Option<f64>;

    # [ wasm_bindgen ( structural , method , getter , js_class = "DeviceRotationRate" , js_name = gamma ) ]
    ///Getter for the `gamma` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceRotationRate/gamma)
    ///
    ///*This API requires the following crate features to be activated: `DeviceRotationRate`*
    pub fn gamma(this: &DeviceRotationRate) -> Option<f64>;

}
