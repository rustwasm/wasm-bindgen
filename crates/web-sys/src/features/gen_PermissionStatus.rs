use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = EventTarget , extends = :: js_sys :: Object , js_name = PermissionStatus , typescript_name = PermissionStatus ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PermissionStatus` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PermissionStatus)\n\n*This API requires the following crate features to be activated: `PermissionStatus`*"]
    pub type PermissionStatus;
    # [ wasm_bindgen ( structural , method , getter , js_name = state ) ]
    #[cfg(feature = "PermissionState")]
    #[doc = "Getter for the `state` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PermissionStatus/state)\n\n*This API requires the following crate features to be activated: `PermissionState`, `PermissionStatus`*"]
    pub fn state(this: &PermissionStatus) -> PermissionState;
    # [ wasm_bindgen ( structural , method , getter , js_name = onchange ) ]
    #[doc = "Getter for the `onchange` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PermissionStatus/onchange)\n\n*This API requires the following crate features to be activated: `PermissionStatus`*"]
    pub fn onchange(this: &PermissionStatus) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onchange ) ]
    #[doc = "Setter for the `onchange` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PermissionStatus/onchange)\n\n*This API requires the following crate features to be activated: `PermissionStatus`*"]
    pub fn set_onchange(this: &PermissionStatus, value: Option<::js_sys::Function>);
}
