use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( is_type_of = | _ | false , extends = :: js_sys :: Object , js_name = DeviceRotationRate , typescript_name = DeviceRotationRate ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DeviceRotationRate` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceRotationRate)\n\n*This API requires the following crate features to be activated: `DeviceRotationRate`*"]
    pub type DeviceRotationRate;
    # [ wasm_bindgen ( structural , method , getter , js_name = alpha ) ]
    #[doc = "Getter for the `alpha` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceRotationRate/alpha)\n\n*This API requires the following crate features to be activated: `DeviceRotationRate`*"]
    pub fn alpha(this: &DeviceRotationRate) -> Option<f64>;
    # [ wasm_bindgen ( structural , method , getter , js_name = beta ) ]
    #[doc = "Getter for the `beta` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceRotationRate/beta)\n\n*This API requires the following crate features to be activated: `DeviceRotationRate`*"]
    pub fn beta(this: &DeviceRotationRate) -> Option<f64>;
    # [ wasm_bindgen ( structural , method , getter , js_name = gamma ) ]
    #[doc = "Getter for the `gamma` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceRotationRate/gamma)\n\n*This API requires the following crate features to be activated: `DeviceRotationRate`*"]
    pub fn gamma(this: &DeviceRotationRate) -> Option<f64>;
}
