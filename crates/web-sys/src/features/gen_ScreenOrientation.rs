use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = EventTarget , extends = :: js_sys :: Object , js_name = ScreenOrientation , typescript_name = ScreenOrientation ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `ScreenOrientation` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScreenOrientation)
    ///
    ///*This API requires the following crate features to be activated: `ScreenOrientation`*
    pub type ScreenOrientation;

    #[cfg(feature = "OrientationType")]
    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "ScreenOrientation" , js_name = type ) ]
    ///Getter for the `type` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScreenOrientation/type)
    ///
    ///*This API requires the following crate features to be activated: `OrientationType`, `ScreenOrientation`*
    pub fn type_(this: &ScreenOrientation) -> Result<OrientationType, JsValue>;

    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "ScreenOrientation" , js_name = angle ) ]
    ///Getter for the `angle` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScreenOrientation/angle)
    ///
    ///*This API requires the following crate features to be activated: `ScreenOrientation`*
    pub fn angle(this: &ScreenOrientation) -> Result<u16, JsValue>;

    # [ wasm_bindgen ( structural , method , getter , js_class = "ScreenOrientation" , js_name = onchange ) ]
    ///Getter for the `onchange` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScreenOrientation/onchange)
    ///
    ///*This API requires the following crate features to be activated: `ScreenOrientation`*
    pub fn onchange(this: &ScreenOrientation) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "ScreenOrientation" , js_name = onchange ) ]
    ///Setter for the `onchange` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScreenOrientation/onchange)
    ///
    ///*This API requires the following crate features to be activated: `ScreenOrientation`*
    pub fn set_onchange(this: &ScreenOrientation, value: Option<&::js_sys::Function>);

    #[cfg(feature = "OrientationLockType")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "ScreenOrientation" , js_name = lock ) ]
    ///The `lock()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScreenOrientation/lock)
    ///
    ///*This API requires the following crate features to be activated: `OrientationLockType`, `ScreenOrientation`*
    pub fn lock(
        this: &ScreenOrientation,
        orientation: OrientationLockType,
    ) -> Result<::js_sys::Promise, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "ScreenOrientation" , js_name = unlock ) ]
    ///The `unlock()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScreenOrientation/unlock)
    ///
    ///*This API requires the following crate features to be activated: `ScreenOrientation`*
    pub fn unlock(this: &ScreenOrientation) -> Result<(), JsValue>;

}
