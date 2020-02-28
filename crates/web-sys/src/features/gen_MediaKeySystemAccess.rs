use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = MediaKeySystemAccess , typescript_name = MediaKeySystemAccess ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MediaKeySystemAccess` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeySystemAccess)\n\n*This API requires the following crate features to be activated: `MediaKeySystemAccess`*"]
    pub type MediaKeySystemAccess;
    # [ wasm_bindgen ( structural , method , getter , js_name = keySystem ) ]
    #[doc = "Getter for the `keySystem` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeySystemAccess/keySystem)\n\n*This API requires the following crate features to be activated: `MediaKeySystemAccess`*"]
    pub fn key_system(this: &MediaKeySystemAccess) -> String;
    # [ wasm_bindgen ( method , structural , js_name = createMediaKeys ) ]
    #[doc = "The `createMediaKeys()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeySystemAccess/createMediaKeys)\n\n*This API requires the following crate features to be activated: `MediaKeySystemAccess`*"]
    pub fn create_media_keys(this: &MediaKeySystemAccess) -> ::js_sys::Promise;
    #[cfg(feature = "MediaKeySystemConfiguration")]
    # [ wasm_bindgen ( method , structural , js_name = getConfiguration ) ]
    #[doc = "The `getConfiguration()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeySystemAccess/getConfiguration)\n\n*This API requires the following crate features to be activated: `MediaKeySystemAccess`, `MediaKeySystemConfiguration`*"]
    pub fn get_configuration(this: &MediaKeySystemAccess) -> MediaKeySystemConfiguration;
}
