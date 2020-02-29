use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = VRDisplayCapabilities , typescript_name = VRDisplayCapabilities ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `VrDisplayCapabilities` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRDisplayCapabilities)
    ///
    ///*This API requires the following crate features to be activated: `VrDisplayCapabilities`*
    pub type VrDisplayCapabilities;

    # [ wasm_bindgen ( structural , method , getter , js_class = "VRDisplayCapabilities" , js_name = hasPosition ) ]
    ///Getter for the `hasPosition` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRDisplayCapabilities/hasPosition)
    ///
    ///*This API requires the following crate features to be activated: `VrDisplayCapabilities`*
    pub fn has_position(this: &VrDisplayCapabilities) -> bool;

    # [ wasm_bindgen ( structural , method , getter , js_class = "VRDisplayCapabilities" , js_name = hasOrientation ) ]
    ///Getter for the `hasOrientation` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRDisplayCapabilities/hasOrientation)
    ///
    ///*This API requires the following crate features to be activated: `VrDisplayCapabilities`*
    pub fn has_orientation(this: &VrDisplayCapabilities) -> bool;

    # [ wasm_bindgen ( structural , method , getter , js_class = "VRDisplayCapabilities" , js_name = hasExternalDisplay ) ]
    ///Getter for the `hasExternalDisplay` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRDisplayCapabilities/hasExternalDisplay)
    ///
    ///*This API requires the following crate features to be activated: `VrDisplayCapabilities`*
    pub fn has_external_display(this: &VrDisplayCapabilities) -> bool;

    # [ wasm_bindgen ( structural , method , getter , js_class = "VRDisplayCapabilities" , js_name = canPresent ) ]
    ///Getter for the `canPresent` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRDisplayCapabilities/canPresent)
    ///
    ///*This API requires the following crate features to be activated: `VrDisplayCapabilities`*
    pub fn can_present(this: &VrDisplayCapabilities) -> bool;

    # [ wasm_bindgen ( structural , method , getter , js_class = "VRDisplayCapabilities" , js_name = maxLayers ) ]
    ///Getter for the `maxLayers` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRDisplayCapabilities/maxLayers)
    ///
    ///*This API requires the following crate features to be activated: `VrDisplayCapabilities`*
    pub fn max_layers(this: &VrDisplayCapabilities) -> u32;

}
