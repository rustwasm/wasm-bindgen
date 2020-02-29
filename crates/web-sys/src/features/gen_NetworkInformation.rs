use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = EventTarget , extends = :: js_sys :: Object , js_name = NetworkInformation , typescript_name = NetworkInformation ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `NetworkInformation` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NetworkInformation)
    ///
    ///*This API requires the following crate features to be activated: `NetworkInformation`*
    pub type NetworkInformation;

    #[cfg(feature = "ConnectionType")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "NetworkInformation" , js_name = type ) ]
    ///Getter for the `type` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NetworkInformation/type)
    ///
    ///*This API requires the following crate features to be activated: `ConnectionType`, `NetworkInformation`*
    pub fn type_(this: &NetworkInformation) -> ConnectionType;

    # [ wasm_bindgen ( structural , method , getter , js_class = "NetworkInformation" , js_name = ontypechange ) ]
    ///Getter for the `ontypechange` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NetworkInformation/ontypechange)
    ///
    ///*This API requires the following crate features to be activated: `NetworkInformation`*
    pub fn ontypechange(this: &NetworkInformation) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "NetworkInformation" , js_name = ontypechange ) ]
    ///Setter for the `ontypechange` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NetworkInformation/ontypechange)
    ///
    ///*This API requires the following crate features to be activated: `NetworkInformation`*
    pub fn set_ontypechange(this: &NetworkInformation, value: Option<&::js_sys::Function>);

}
