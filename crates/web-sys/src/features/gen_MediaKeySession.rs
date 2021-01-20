#![allow(unused_imports)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = EventTarget , extends = :: js_sys :: Object , js_name = MediaKeySession , typescript_type = "MediaKeySession")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MediaKeySession` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeySession)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeySession`*"]
    pub type MediaKeySession;
    #[cfg(feature = "MediaKeyError")]
    # [wasm_bindgen (structural , method , getter , js_class = "MediaKeySession" , js_name = error)]
    #[doc = "Getter for the `error` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeySession/error)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeyError`, `MediaKeySession`*"]
    pub fn error(this: &MediaKeySession) -> Option<MediaKeyError>;
    # [wasm_bindgen (structural , method , getter , js_class = "MediaKeySession" , js_name = sessionId)]
    #[doc = "Getter for the `sessionId` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeySession/sessionId)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeySession`*"]
    pub fn session_id(this: &MediaKeySession) -> String;
    # [wasm_bindgen (structural , method , getter , js_class = "MediaKeySession" , js_name = expiration)]
    #[doc = "Getter for the `expiration` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeySession/expiration)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeySession`*"]
    pub fn expiration(this: &MediaKeySession) -> f64;
    # [wasm_bindgen (structural , method , getter , js_class = "MediaKeySession" , js_name = closed)]
    #[doc = "Getter for the `closed` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeySession/closed)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeySession`*"]
    #[doc = ""]
    #[doc = "Return value: While the Promise can produce any JsValue as far as the type system is concerned, practically it is just used to indicate completion."]
    pub fn closed(this: &MediaKeySession) -> ::js_sys::Promise;
    #[cfg(feature = "MediaKeyStatusMap")]
    # [wasm_bindgen (structural , method , getter , js_class = "MediaKeySession" , js_name = keyStatuses)]
    #[doc = "Getter for the `keyStatuses` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeySession/keyStatuses)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeySession`, `MediaKeyStatusMap`*"]
    pub fn key_statuses(this: &MediaKeySession) -> MediaKeyStatusMap;
    # [wasm_bindgen (structural , method , getter , js_class = "MediaKeySession" , js_name = onkeystatuseschange)]
    #[doc = "Getter for the `onkeystatuseschange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeySession/onkeystatuseschange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeySession`*"]
    #[doc = ""]
    #[doc = "Return value: See the referenced MDN documentation or the IDL files for the signature of the callback inside the option."]
    pub fn onkeystatuseschange(this: &MediaKeySession) -> Option<::js_sys::Function>;
    # [wasm_bindgen (structural , method , setter , js_class = "MediaKeySession" , js_name = onkeystatuseschange)]
    #[doc = "Setter for the `onkeystatuseschange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeySession/onkeystatuseschange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeySession`*"]
    #[doc = ""]
    #[doc = "Argument: See the referenced MDN documentation or the IDL files for the signature of the callback inside the option."]
    pub fn set_onkeystatuseschange(this: &MediaKeySession, value: Option<&::js_sys::Function>);
    # [wasm_bindgen (structural , method , getter , js_class = "MediaKeySession" , js_name = onmessage)]
    #[doc = "Getter for the `onmessage` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeySession/onmessage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeySession`*"]
    #[doc = ""]
    #[doc = "Return value: See the referenced MDN documentation or the IDL files for the signature of the callback inside the option."]
    pub fn onmessage(this: &MediaKeySession) -> Option<::js_sys::Function>;
    # [wasm_bindgen (structural , method , setter , js_class = "MediaKeySession" , js_name = onmessage)]
    #[doc = "Setter for the `onmessage` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeySession/onmessage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeySession`*"]
    #[doc = ""]
    #[doc = "Argument: See the referenced MDN documentation or the IDL files for the signature of the callback inside the option."]
    pub fn set_onmessage(this: &MediaKeySession, value: Option<&::js_sys::Function>);
    # [wasm_bindgen (method , structural , js_class = "MediaKeySession" , js_name = close)]
    #[doc = "The `close()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeySession/close)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeySession`*"]
    #[doc = ""]
    #[doc = "Return value: While the Promise can produce any JsValue as far as the type system is concerned, practically it is just used to indicate completion."]
    pub fn close(this: &MediaKeySession) -> ::js_sys::Promise;
    # [wasm_bindgen (method , structural , js_class = "MediaKeySession" , js_name = generateRequest)]
    #[doc = "The `generateRequest()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeySession/generateRequest)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeySession`*"]
    #[doc = ""]
    #[doc = "Return value: While the Promise can produce any JsValue as far as the type system is concerned, practically it is just used to indicate completion."]
    pub fn generate_request_with_buffer_source(
        this: &MediaKeySession,
        init_data_type: &str,
        init_data: &::js_sys::Object,
    ) -> ::js_sys::Promise;
    # [wasm_bindgen (method , structural , js_class = "MediaKeySession" , js_name = generateRequest)]
    #[doc = "The `generateRequest()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeySession/generateRequest)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeySession`*"]
    #[doc = ""]
    #[doc = "Return value: While the Promise can produce any JsValue as far as the type system is concerned, practically it is just used to indicate completion."]
    pub fn generate_request_with_u8_array(
        this: &MediaKeySession,
        init_data_type: &str,
        init_data: &mut [u8],
    ) -> ::js_sys::Promise;
    # [wasm_bindgen (method , structural , js_class = "MediaKeySession" , js_name = load)]
    #[doc = "The `load()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeySession/load)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeySession`*"]
    #[doc = ""]
    #[doc = "Return value: While the Promise can produce any JsValue as far as the type system is concerned, practically it is expected to contain a <code>[bool]</code>. It can be converted like `<code>let result: [bool] = result.await.into();</code>."]
    pub fn load(this: &MediaKeySession, session_id: &str) -> ::js_sys::Promise;
    # [wasm_bindgen (method , structural , js_class = "MediaKeySession" , js_name = remove)]
    #[doc = "The `remove()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeySession/remove)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeySession`*"]
    #[doc = ""]
    #[doc = "Return value: While the Promise can produce any JsValue as far as the type system is concerned, practically it is just used to indicate completion."]
    pub fn remove(this: &MediaKeySession) -> ::js_sys::Promise;
    # [wasm_bindgen (method , structural , js_class = "MediaKeySession" , js_name = update)]
    #[doc = "The `update()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeySession/update)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeySession`*"]
    #[doc = ""]
    #[doc = "Return value: While the Promise can produce any JsValue as far as the type system is concerned, practically it is just used to indicate completion."]
    pub fn update_with_buffer_source(
        this: &MediaKeySession,
        response: &::js_sys::Object,
    ) -> ::js_sys::Promise;
    # [wasm_bindgen (method , structural , js_class = "MediaKeySession" , js_name = update)]
    #[doc = "The `update()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeySession/update)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeySession`*"]
    #[doc = ""]
    #[doc = "Return value: While the Promise can produce any JsValue as far as the type system is concerned, practically it is just used to indicate completion."]
    pub fn update_with_u8_array(this: &MediaKeySession, response: &mut [u8]) -> ::js_sys::Promise;
}
