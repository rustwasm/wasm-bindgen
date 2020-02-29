use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = EventTarget , extends = :: js_sys :: Object , js_name = PermissionStatus , typescript_type = "PermissionStatus" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `PermissionStatus` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PermissionStatus)
    ///
    ///*This API requires the following crate features to be activated: `PermissionStatus`*
    pub type PermissionStatus;

    #[cfg(feature = "PermissionState")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "PermissionStatus" , js_name = state ) ]
    ///Getter for the `state` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PermissionStatus/state)
    ///
    ///*This API requires the following crate features to be activated: `PermissionState`, `PermissionStatus`*
    pub fn state(this: &PermissionStatus) -> PermissionState;

    # [ wasm_bindgen ( structural , method , getter , js_class = "PermissionStatus" , js_name = onchange ) ]
    ///Getter for the `onchange` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PermissionStatus/onchange)
    ///
    ///*This API requires the following crate features to be activated: `PermissionStatus`*
    pub fn onchange(this: &PermissionStatus) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "PermissionStatus" , js_name = onchange ) ]
    ///Setter for the `onchange` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PermissionStatus/onchange)
    ///
    ///*This API requires the following crate features to be activated: `PermissionStatus`*
    pub fn set_onchange(this: &PermissionStatus, value: Option<&::js_sys::Function>);

}
