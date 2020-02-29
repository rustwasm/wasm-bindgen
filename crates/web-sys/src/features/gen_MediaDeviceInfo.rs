use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = MediaDeviceInfo , typescript_name = MediaDeviceInfo ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `MediaDeviceInfo` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaDeviceInfo)
    ///
    ///*This API requires the following crate features to be activated: `MediaDeviceInfo`*
    pub type MediaDeviceInfo;

    # [ wasm_bindgen ( structural , method , getter , js_class = "MediaDeviceInfo" , js_name = deviceId ) ]
    ///Getter for the `deviceId` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaDeviceInfo/deviceId)
    ///
    ///*This API requires the following crate features to be activated: `MediaDeviceInfo`*
    pub fn device_id(this: &MediaDeviceInfo) -> String;

    #[cfg(feature = "MediaDeviceKind")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "MediaDeviceInfo" , js_name = kind ) ]
    ///Getter for the `kind` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaDeviceInfo/kind)
    ///
    ///*This API requires the following crate features to be activated: `MediaDeviceInfo`, `MediaDeviceKind`*
    pub fn kind(this: &MediaDeviceInfo) -> MediaDeviceKind;

    # [ wasm_bindgen ( structural , method , getter , js_class = "MediaDeviceInfo" , js_name = label ) ]
    ///Getter for the `label` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaDeviceInfo/label)
    ///
    ///*This API requires the following crate features to be activated: `MediaDeviceInfo`*
    pub fn label(this: &MediaDeviceInfo) -> String;

    # [ wasm_bindgen ( structural , method , getter , js_class = "MediaDeviceInfo" , js_name = groupId ) ]
    ///Getter for the `groupId` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaDeviceInfo/groupId)
    ///
    ///*This API requires the following crate features to be activated: `MediaDeviceInfo`*
    pub fn group_id(this: &MediaDeviceInfo) -> String;

    # [ wasm_bindgen ( method , structural , js_class = "MediaDeviceInfo" , js_name = toJSON ) ]
    ///The `toJSON()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaDeviceInfo/toJSON)
    ///
    ///*This API requires the following crate features to be activated: `MediaDeviceInfo`*
    pub fn to_json(this: &MediaDeviceInfo) -> ::js_sys::Object;

}
