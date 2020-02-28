use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = EventTarget , extends = :: js_sys :: Object , js_name = MediaKeySession , typescript_name = MediaKeySession ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MediaKeySession` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeySession)\n\n*This API requires the following crate features to be activated: `MediaKeySession`*"]
    pub type MediaKeySession;
    # [ wasm_bindgen ( structural , method , getter , js_class = "MediaKeySession" , js_name = error ) ]
    #[cfg(feature = "MediaKeyError")]
    #[doc = "Getter for the `error` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeySession/error)\n\n*This API requires the following crate features to be activated: `MediaKeyError`, `MediaKeySession`*"]
    pub fn error(this: &MediaKeySession) -> Option<MediaKeyError>;
    # [ wasm_bindgen ( structural , method , getter , js_class = "MediaKeySession" , js_name = sessionId ) ]
    #[doc = "Getter for the `sessionId` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeySession/sessionId)\n\n*This API requires the following crate features to be activated: `MediaKeySession`*"]
    pub fn session_id(this: &MediaKeySession) -> String;
    # [ wasm_bindgen ( structural , method , getter , js_class = "MediaKeySession" , js_name = expiration ) ]
    #[doc = "Getter for the `expiration` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeySession/expiration)\n\n*This API requires the following crate features to be activated: `MediaKeySession`*"]
    pub fn expiration(this: &MediaKeySession) -> f64;
    # [ wasm_bindgen ( structural , method , getter , js_class = "MediaKeySession" , js_name = closed ) ]
    #[doc = "Getter for the `closed` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeySession/closed)\n\n*This API requires the following crate features to be activated: `MediaKeySession`*"]
    pub fn closed(this: &MediaKeySession) -> ::js_sys::Promise;
    # [ wasm_bindgen ( structural , method , getter , js_class = "MediaKeySession" , js_name = keyStatuses ) ]
    #[cfg(feature = "MediaKeyStatusMap")]
    #[doc = "Getter for the `keyStatuses` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeySession/keyStatuses)\n\n*This API requires the following crate features to be activated: `MediaKeySession`, `MediaKeyStatusMap`*"]
    pub fn key_statuses(this: &MediaKeySession) -> MediaKeyStatusMap;
    # [ wasm_bindgen ( structural , method , getter , js_class = "MediaKeySession" , js_name = onkeystatuseschange ) ]
    #[doc = "Getter for the `onkeystatuseschange` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeySession/onkeystatuseschange)\n\n*This API requires the following crate features to be activated: `MediaKeySession`*"]
    pub fn onkeystatuseschange(this: &MediaKeySession) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_class = "MediaKeySession" , js_name = onkeystatuseschange ) ]
    #[doc = "Setter for the `onkeystatuseschange` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeySession/onkeystatuseschange)\n\n*This API requires the following crate features to be activated: `MediaKeySession`*"]
    pub fn set_onkeystatuseschange(this: &MediaKeySession, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_class = "MediaKeySession" , js_name = onmessage ) ]
    #[doc = "Getter for the `onmessage` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeySession/onmessage)\n\n*This API requires the following crate features to be activated: `MediaKeySession`*"]
    pub fn onmessage(this: &MediaKeySession) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_class = "MediaKeySession" , js_name = onmessage ) ]
    #[doc = "Setter for the `onmessage` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeySession/onmessage)\n\n*This API requires the following crate features to be activated: `MediaKeySession`*"]
    pub fn set_onmessage(this: &MediaKeySession, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( method , structural , js_class = "MediaKeySession" , js_name = close ) ]
    #[doc = "The `close()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeySession/close)\n\n*This API requires the following crate features to be activated: `MediaKeySession`*"]
    pub fn close(this: &MediaKeySession) -> ::js_sys::Promise;
    # [ wasm_bindgen ( method , structural , js_class = "MediaKeySession" , js_name = generateRequest ) ]
    #[doc = "The `generateRequest()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeySession/generateRequest)\n\n*This API requires the following crate features to be activated: `MediaKeySession`*"]
    pub fn generate_request_with_buffer_source(
        this: &MediaKeySession,
        init_data_type: &str,
        init_data: &::js_sys::Object,
    ) -> ::js_sys::Promise;
    # [ wasm_bindgen ( method , structural , js_class = "MediaKeySession" , js_name = generateRequest ) ]
    #[doc = "The `generateRequest()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeySession/generateRequest)\n\n*This API requires the following crate features to be activated: `MediaKeySession`*"]
    pub fn generate_request_with_u8_array(
        this: &MediaKeySession,
        init_data_type: &str,
        init_data: &mut [u8],
    ) -> ::js_sys::Promise;
    # [ wasm_bindgen ( method , structural , js_class = "MediaKeySession" , js_name = load ) ]
    #[doc = "The `load()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeySession/load)\n\n*This API requires the following crate features to be activated: `MediaKeySession`*"]
    pub fn load(this: &MediaKeySession, session_id: &str) -> ::js_sys::Promise;
    # [ wasm_bindgen ( method , structural , js_class = "MediaKeySession" , js_name = remove ) ]
    #[doc = "The `remove()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeySession/remove)\n\n*This API requires the following crate features to be activated: `MediaKeySession`*"]
    pub fn remove(this: &MediaKeySession) -> ::js_sys::Promise;
    # [ wasm_bindgen ( method , structural , js_class = "MediaKeySession" , js_name = update ) ]
    #[doc = "The `update()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeySession/update)\n\n*This API requires the following crate features to be activated: `MediaKeySession`*"]
    pub fn update_with_buffer_source(
        this: &MediaKeySession,
        response: &::js_sys::Object,
    ) -> ::js_sys::Promise;
    # [ wasm_bindgen ( method , structural , js_class = "MediaKeySession" , js_name = update ) ]
    #[doc = "The `update()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeySession/update)\n\n*This API requires the following crate features to be activated: `MediaKeySession`*"]
    pub fn update_with_u8_array(this: &MediaKeySession, response: &mut [u8]) -> ::js_sys::Promise;
}
