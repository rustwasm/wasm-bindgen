use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = EventTarget , extends = :: js_sys :: Object , js_name = MediaKeySession , typescript_type = "MediaKeySession" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `MediaKeySession` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeySession)
    ///
    ///*This API requires the following crate features to be activated: `MediaKeySession`*
    pub type MediaKeySession;

    #[cfg(feature = "MediaKeyError")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "MediaKeySession" , js_name = error ) ]
    ///Getter for the `error` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeySession/error)
    ///
    ///*This API requires the following crate features to be activated: `MediaKeyError`, `MediaKeySession`*
    pub fn error(this: &MediaKeySession) -> Option<MediaKeyError>;

    # [ wasm_bindgen ( structural , method , getter , js_class = "MediaKeySession" , js_name = sessionId ) ]
    ///Getter for the `sessionId` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeySession/sessionId)
    ///
    ///*This API requires the following crate features to be activated: `MediaKeySession`*
    pub fn session_id(this: &MediaKeySession) -> String;

    # [ wasm_bindgen ( structural , method , getter , js_class = "MediaKeySession" , js_name = expiration ) ]
    ///Getter for the `expiration` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeySession/expiration)
    ///
    ///*This API requires the following crate features to be activated: `MediaKeySession`*
    pub fn expiration(this: &MediaKeySession) -> f64;

    # [ wasm_bindgen ( structural , method , getter , js_class = "MediaKeySession" , js_name = closed ) ]
    ///Getter for the `closed` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeySession/closed)
    ///
    ///*This API requires the following crate features to be activated: `MediaKeySession`*
    pub fn closed(this: &MediaKeySession) -> ::js_sys::Promise;

    #[cfg(feature = "MediaKeyStatusMap")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "MediaKeySession" , js_name = keyStatuses ) ]
    ///Getter for the `keyStatuses` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeySession/keyStatuses)
    ///
    ///*This API requires the following crate features to be activated: `MediaKeySession`, `MediaKeyStatusMap`*
    pub fn key_statuses(this: &MediaKeySession) -> MediaKeyStatusMap;

    # [ wasm_bindgen ( structural , method , getter , js_class = "MediaKeySession" , js_name = onkeystatuseschange ) ]
    ///Getter for the `onkeystatuseschange` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeySession/onkeystatuseschange)
    ///
    ///*This API requires the following crate features to be activated: `MediaKeySession`*
    pub fn onkeystatuseschange(this: &MediaKeySession) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "MediaKeySession" , js_name = onkeystatuseschange ) ]
    ///Setter for the `onkeystatuseschange` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeySession/onkeystatuseschange)
    ///
    ///*This API requires the following crate features to be activated: `MediaKeySession`*
    pub fn set_onkeystatuseschange(this: &MediaKeySession, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "MediaKeySession" , js_name = onmessage ) ]
    ///Getter for the `onmessage` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeySession/onmessage)
    ///
    ///*This API requires the following crate features to be activated: `MediaKeySession`*
    pub fn onmessage(this: &MediaKeySession) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "MediaKeySession" , js_name = onmessage ) ]
    ///Setter for the `onmessage` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeySession/onmessage)
    ///
    ///*This API requires the following crate features to be activated: `MediaKeySession`*
    pub fn set_onmessage(this: &MediaKeySession, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( method , structural , js_class = "MediaKeySession" , js_name = close ) ]
    ///The `close()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeySession/close)
    ///
    ///*This API requires the following crate features to be activated: `MediaKeySession`*
    pub fn close(this: &MediaKeySession) -> ::js_sys::Promise;

    # [ wasm_bindgen ( method , structural , js_class = "MediaKeySession" , js_name = generateRequest ) ]
    ///The `generateRequest()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeySession/generateRequest)
    ///
    ///*This API requires the following crate features to be activated: `MediaKeySession`*
    pub fn generate_request_with_buffer_source(
        this: &MediaKeySession,
        init_data_type: &str,
        init_data: &::js_sys::Object,
    ) -> ::js_sys::Promise;

    # [ wasm_bindgen ( method , structural , js_class = "MediaKeySession" , js_name = generateRequest ) ]
    ///The `generateRequest()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeySession/generateRequest)
    ///
    ///*This API requires the following crate features to be activated: `MediaKeySession`*
    pub fn generate_request_with_u8_array(
        this: &MediaKeySession,
        init_data_type: &str,
        init_data: &mut [u8],
    ) -> ::js_sys::Promise;

    # [ wasm_bindgen ( method , structural , js_class = "MediaKeySession" , js_name = load ) ]
    ///The `load()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeySession/load)
    ///
    ///*This API requires the following crate features to be activated: `MediaKeySession`*
    pub fn load(this: &MediaKeySession, session_id: &str) -> ::js_sys::Promise;

    # [ wasm_bindgen ( method , structural , js_class = "MediaKeySession" , js_name = remove ) ]
    ///The `remove()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeySession/remove)
    ///
    ///*This API requires the following crate features to be activated: `MediaKeySession`*
    pub fn remove(this: &MediaKeySession) -> ::js_sys::Promise;

    # [ wasm_bindgen ( method , structural , js_class = "MediaKeySession" , js_name = update ) ]
    ///The `update()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeySession/update)
    ///
    ///*This API requires the following crate features to be activated: `MediaKeySession`*
    pub fn update_with_buffer_source(
        this: &MediaKeySession,
        response: &::js_sys::Object,
    ) -> ::js_sys::Promise;

    # [ wasm_bindgen ( method , structural , js_class = "MediaKeySession" , js_name = update ) ]
    ///The `update()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeySession/update)
    ///
    ///*This API requires the following crate features to be activated: `MediaKeySession`*
    pub fn update_with_u8_array(this: &MediaKeySession, response: &mut [u8]) -> ::js_sys::Promise;

}
