use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = EventTarget , extends = :: js_sys :: Object , js_name = MediaDevices , typescript_name = MediaDevices ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MediaDevices` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaDevices)\n\n*This API requires the following crate features to be activated: `MediaDevices`*"]
    pub type MediaDevices;
    # [ wasm_bindgen ( structural , method , getter , js_name = ondevicechange ) ]
    #[doc = "Getter for the `ondevicechange` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaDevices/ondevicechange)\n\n*This API requires the following crate features to be activated: `MediaDevices`*"]
    pub fn ondevicechange(this: &MediaDevices) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = ondevicechange ) ]
    #[doc = "Setter for the `ondevicechange` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaDevices/ondevicechange)\n\n*This API requires the following crate features to be activated: `MediaDevices`*"]
    pub fn set_ondevicechange(this: &MediaDevices, value: Option<::js_sys::Function>);
    # [ wasm_bindgen ( catch , method , structural , js_name = enumerateDevices ) ]
    #[doc = "The `enumerateDevices()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaDevices/enumerateDevices)\n\n*This API requires the following crate features to be activated: `MediaDevices`*"]
    pub fn enumerate_devices(this: &MediaDevices) -> Result<::js_sys::Promise, JsValue>;
    #[cfg(feature = "MediaTrackSupportedConstraints")]
    # [ wasm_bindgen ( method , structural , js_name = getSupportedConstraints ) ]
    #[doc = "The `getSupportedConstraints()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaDevices/getSupportedConstraints)\n\n*This API requires the following crate features to be activated: `MediaDevices`, `MediaTrackSupportedConstraints`*"]
    pub fn get_supported_constraints(this: &MediaDevices) -> MediaTrackSupportedConstraints;
    # [ wasm_bindgen ( catch , method , structural , js_name = getUserMedia ) ]
    #[doc = "The `getUserMedia()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaDevices/getUserMedia)\n\n*This API requires the following crate features to be activated: `MediaDevices`*"]
    pub fn get_user_media(this: &MediaDevices) -> Result<::js_sys::Promise, JsValue>;
    #[cfg(feature = "MediaStreamConstraints")]
    # [ wasm_bindgen ( catch , method , structural , js_name = getUserMedia ) ]
    #[doc = "The `getUserMedia()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaDevices/getUserMedia)\n\n*This API requires the following crate features to be activated: `MediaDevices`, `MediaStreamConstraints`*"]
    pub fn get_user_media_with_constraints(
        this: &MediaDevices,
        constraints: &MediaStreamConstraints,
    ) -> Result<::js_sys::Promise, JsValue>;
}
