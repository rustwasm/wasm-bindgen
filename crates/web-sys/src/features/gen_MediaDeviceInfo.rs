use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = MediaDeviceInfo , typescript_name = MediaDeviceInfo ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MediaDeviceInfo` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaDeviceInfo)\n\n*This API requires the following crate features to be activated: `MediaDeviceInfo`*"]
    pub type MediaDeviceInfo;
    # [ wasm_bindgen ( structural , method , getter , js_name = deviceId ) ]
    #[doc = "Getter for the `deviceId` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaDeviceInfo/deviceId)\n\n*This API requires the following crate features to be activated: `MediaDeviceInfo`*"]
    pub fn device_id(this: &MediaDeviceInfo) -> String;
    # [ wasm_bindgen ( structural , method , getter , js_name = kind ) ]
    #[cfg(feature = "MediaDeviceKind")]
    #[doc = "Getter for the `kind` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaDeviceInfo/kind)\n\n*This API requires the following crate features to be activated: `MediaDeviceInfo`, `MediaDeviceKind`*"]
    pub fn kind(this: &MediaDeviceInfo) -> MediaDeviceKind;
    # [ wasm_bindgen ( structural , method , getter , js_name = label ) ]
    #[doc = "Getter for the `label` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaDeviceInfo/label)\n\n*This API requires the following crate features to be activated: `MediaDeviceInfo`*"]
    pub fn label(this: &MediaDeviceInfo) -> String;
    # [ wasm_bindgen ( structural , method , getter , js_name = groupId ) ]
    #[doc = "Getter for the `groupId` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaDeviceInfo/groupId)\n\n*This API requires the following crate features to be activated: `MediaDeviceInfo`*"]
    pub fn group_id(this: &MediaDeviceInfo) -> String;
    # [ wasm_bindgen ( method , structural , js_name = toJSON ) ]
    #[doc = "The `toJSON()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaDeviceInfo/toJSON)\n\n*This API requires the following crate features to be activated: `MediaDeviceInfo`*"]
    pub fn to_json(this: &MediaDeviceInfo) -> ::js_sys::Object;
}
