use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = EventTarget , extends = :: js_sys :: Object , js_name = MediaDevices , typescript_type = "MediaDevices" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `MediaDevices` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaDevices)
    ///
    ///*This API requires the following crate features to be activated: `MediaDevices`*
    pub type MediaDevices;

    # [ wasm_bindgen ( structural , method , getter , js_class = "MediaDevices" , js_name = ondevicechange ) ]
    ///Getter for the `ondevicechange` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaDevices/ondevicechange)
    ///
    ///*This API requires the following crate features to be activated: `MediaDevices`*
    pub fn ondevicechange(this: &MediaDevices) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "MediaDevices" , js_name = ondevicechange ) ]
    ///Setter for the `ondevicechange` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaDevices/ondevicechange)
    ///
    ///*This API requires the following crate features to be activated: `MediaDevices`*
    pub fn set_ondevicechange(this: &MediaDevices, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( catch , method , structural , js_class = "MediaDevices" , js_name = enumerateDevices ) ]
    ///The `enumerateDevices()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaDevices/enumerateDevices)
    ///
    ///*This API requires the following crate features to be activated: `MediaDevices`*
    pub fn enumerate_devices(this: &MediaDevices) -> Result<::js_sys::Promise, JsValue>;

    #[cfg(feature = "MediaTrackSupportedConstraints")]
    # [ wasm_bindgen ( method , structural , js_class = "MediaDevices" , js_name = getSupportedConstraints ) ]
    ///The `getSupportedConstraints()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaDevices/getSupportedConstraints)
    ///
    ///*This API requires the following crate features to be activated: `MediaDevices`, `MediaTrackSupportedConstraints`*
    pub fn get_supported_constraints(this: &MediaDevices) -> MediaTrackSupportedConstraints;

    # [ wasm_bindgen ( catch , method , structural , js_class = "MediaDevices" , js_name = getUserMedia ) ]
    ///The `getUserMedia()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaDevices/getUserMedia)
    ///
    ///*This API requires the following crate features to be activated: `MediaDevices`*
    pub fn get_user_media(this: &MediaDevices) -> Result<::js_sys::Promise, JsValue>;

    #[cfg(feature = "MediaStreamConstraints")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "MediaDevices" , js_name = getUserMedia ) ]
    ///The `getUserMedia()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaDevices/getUserMedia)
    ///
    ///*This API requires the following crate features to be activated: `MediaDevices`, `MediaStreamConstraints`*
    pub fn get_user_media_with_constraints(
        this: &MediaDevices,
        constraints: &MediaStreamConstraints,
    ) -> Result<::js_sys::Promise, JsValue>;

}
