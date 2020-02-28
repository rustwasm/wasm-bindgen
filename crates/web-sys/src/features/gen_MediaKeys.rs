use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = MediaKeys , typescript_name = MediaKeys ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MediaKeys` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeys)\n\n*This API requires the following crate features to be activated: `MediaKeys`*"]
    pub type MediaKeys;
    # [ wasm_bindgen ( structural , method , getter , js_class = "MediaKeys" , js_name = keySystem ) ]
    #[doc = "Getter for the `keySystem` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeys/keySystem)\n\n*This API requires the following crate features to be activated: `MediaKeys`*"]
    pub fn key_system(this: &MediaKeys) -> String;
    #[cfg(feature = "MediaKeySession")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "MediaKeys" , js_name = createSession ) ]
    #[doc = "The `createSession()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeys/createSession)\n\n*This API requires the following crate features to be activated: `MediaKeySession`, `MediaKeys`*"]
    pub fn create_session(this: &MediaKeys) -> Result<MediaKeySession, JsValue>;
    #[cfg(all(feature = "MediaKeySession", feature = "MediaKeySessionType",))]
    # [ wasm_bindgen ( catch , method , structural , js_class = "MediaKeys" , js_name = createSession ) ]
    #[doc = "The `createSession()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeys/createSession)\n\n*This API requires the following crate features to be activated: `MediaKeySession`, `MediaKeySessionType`, `MediaKeys`*"]
    pub fn create_session_with_session_type(
        this: &MediaKeys,
        session_type: MediaKeySessionType,
    ) -> Result<MediaKeySession, JsValue>;
    # [ wasm_bindgen ( method , structural , js_class = "MediaKeys" , js_name = getStatusForPolicy ) ]
    #[doc = "The `getStatusForPolicy()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeys/getStatusForPolicy)\n\n*This API requires the following crate features to be activated: `MediaKeys`*"]
    pub fn get_status_for_policy(this: &MediaKeys) -> ::js_sys::Promise;
    #[cfg(feature = "MediaKeysPolicy")]
    # [ wasm_bindgen ( method , structural , js_class = "MediaKeys" , js_name = getStatusForPolicy ) ]
    #[doc = "The `getStatusForPolicy()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeys/getStatusForPolicy)\n\n*This API requires the following crate features to be activated: `MediaKeys`, `MediaKeysPolicy`*"]
    pub fn get_status_for_policy_with_policy(
        this: &MediaKeys,
        policy: &MediaKeysPolicy,
    ) -> ::js_sys::Promise;
    # [ wasm_bindgen ( method , structural , js_class = "MediaKeys" , js_name = setServerCertificate ) ]
    #[doc = "The `setServerCertificate()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeys/setServerCertificate)\n\n*This API requires the following crate features to be activated: `MediaKeys`*"]
    pub fn set_server_certificate_with_buffer_source(
        this: &MediaKeys,
        server_certificate: &::js_sys::Object,
    ) -> ::js_sys::Promise;
    # [ wasm_bindgen ( method , structural , js_class = "MediaKeys" , js_name = setServerCertificate ) ]
    #[doc = "The `setServerCertificate()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeys/setServerCertificate)\n\n*This API requires the following crate features to be activated: `MediaKeys`*"]
    pub fn set_server_certificate_with_u8_array(
        this: &MediaKeys,
        server_certificate: &mut [u8],
    ) -> ::js_sys::Promise;
}
