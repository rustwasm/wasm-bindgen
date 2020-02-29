use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = MediaKeySystemAccess , typescript_type = "MediaKeySystemAccess" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `MediaKeySystemAccess` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeySystemAccess)
    ///
    ///*This API requires the following crate features to be activated: `MediaKeySystemAccess`*
    pub type MediaKeySystemAccess;

    # [ wasm_bindgen ( structural , method , getter , js_class = "MediaKeySystemAccess" , js_name = keySystem ) ]
    ///Getter for the `keySystem` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeySystemAccess/keySystem)
    ///
    ///*This API requires the following crate features to be activated: `MediaKeySystemAccess`*
    pub fn key_system(this: &MediaKeySystemAccess) -> String;

    # [ wasm_bindgen ( method , structural , js_class = "MediaKeySystemAccess" , js_name = createMediaKeys ) ]
    ///The `createMediaKeys()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeySystemAccess/createMediaKeys)
    ///
    ///*This API requires the following crate features to be activated: `MediaKeySystemAccess`*
    pub fn create_media_keys(this: &MediaKeySystemAccess) -> ::js_sys::Promise;

    #[cfg(feature = "MediaKeySystemConfiguration")]
    # [ wasm_bindgen ( method , structural , js_class = "MediaKeySystemAccess" , js_name = getConfiguration ) ]
    ///The `getConfiguration()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeySystemAccess/getConfiguration)
    ///
    ///*This API requires the following crate features to be activated: `MediaKeySystemAccess`, `MediaKeySystemConfiguration`*
    pub fn get_configuration(this: &MediaKeySystemAccess) -> MediaKeySystemConfiguration;

}
