use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = EventTarget , extends = :: js_sys :: Object , js_name = ScreenOrientation , typescript_name = ScreenOrientation ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ScreenOrientation` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScreenOrientation)\n\n*This API requires the following crate features to be activated: `ScreenOrientation`*"]
    pub type ScreenOrientation;
    # [ wasm_bindgen ( structural , catch , method , getter , js_name = type ) ]
    #[cfg(feature = "OrientationType")]
    #[doc = "Getter for the `type` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScreenOrientation/type)\n\n*This API requires the following crate features to be activated: `OrientationType`, `ScreenOrientation`*"]
    pub fn type_(this: &ScreenOrientation) -> Result<OrientationType, JsValue>;
    # [ wasm_bindgen ( structural , catch , method , getter , js_name = angle ) ]
    #[doc = "Getter for the `angle` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScreenOrientation/angle)\n\n*This API requires the following crate features to be activated: `ScreenOrientation`*"]
    pub fn angle(this: &ScreenOrientation) -> Result<u16, JsValue>;
    # [ wasm_bindgen ( structural , method , getter , js_name = onchange ) ]
    #[doc = "Getter for the `onchange` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScreenOrientation/onchange)\n\n*This API requires the following crate features to be activated: `ScreenOrientation`*"]
    pub fn onchange(this: &ScreenOrientation) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onchange ) ]
    #[doc = "Setter for the `onchange` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScreenOrientation/onchange)\n\n*This API requires the following crate features to be activated: `ScreenOrientation`*"]
    pub fn set_onchange(this: &ScreenOrientation, value: Option<&::js_sys::Function>);
    #[cfg(feature = "OrientationLockType")]
    # [ wasm_bindgen ( catch , method , structural , js_name = lock ) ]
    #[doc = "The `lock()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScreenOrientation/lock)\n\n*This API requires the following crate features to be activated: `OrientationLockType`, `ScreenOrientation`*"]
    pub fn lock(
        this: &ScreenOrientation,
        orientation: OrientationLockType,
    ) -> Result<::js_sys::Promise, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = unlock ) ]
    #[doc = "The `unlock()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScreenOrientation/unlock)\n\n*This API requires the following crate features to be activated: `ScreenOrientation`*"]
    pub fn unlock(this: &ScreenOrientation) -> Result<(), JsValue>;
}
