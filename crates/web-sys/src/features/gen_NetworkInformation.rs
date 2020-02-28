use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = EventTarget , extends = :: js_sys :: Object , js_name = NetworkInformation , typescript_name = NetworkInformation ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `NetworkInformation` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NetworkInformation)\n\n*This API requires the following crate features to be activated: `NetworkInformation`*"]
    pub type NetworkInformation;
    # [ wasm_bindgen ( structural , method , getter , js_name = type ) ]
    #[cfg(feature = "ConnectionType")]
    #[doc = "Getter for the `type` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NetworkInformation/type)\n\n*This API requires the following crate features to be activated: `ConnectionType`, `NetworkInformation`*"]
    pub fn type_(this: &NetworkInformation) -> ConnectionType;
    # [ wasm_bindgen ( structural , method , getter , js_name = ontypechange ) ]
    #[doc = "Getter for the `ontypechange` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NetworkInformation/ontypechange)\n\n*This API requires the following crate features to be activated: `NetworkInformation`*"]
    pub fn ontypechange(this: &NetworkInformation) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = ontypechange ) ]
    #[doc = "Setter for the `ontypechange` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NetworkInformation/ontypechange)\n\n*This API requires the following crate features to be activated: `NetworkInformation`*"]
    pub fn set_ontypechange(this: &NetworkInformation, value: Option<&::js_sys::Function>);
}
