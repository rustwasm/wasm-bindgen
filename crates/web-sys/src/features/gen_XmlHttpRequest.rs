use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = XmlHttpRequestEventTarget , extends = EventTarget , extends = :: js_sys :: Object , js_name = XMLHttpRequest , typescript_name = XMLHttpRequest ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `XmlHttpRequest` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest)\n\n*This API requires the following crate features to be activated: `XmlHttpRequest`*"]
    pub type XmlHttpRequest;
    # [ wasm_bindgen ( structural , method , getter , js_name = onreadystatechange ) ]
    #[doc = "Getter for the `onreadystatechange` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/onreadystatechange)\n\n*This API requires the following crate features to be activated: `XmlHttpRequest`*"]
    pub fn onreadystatechange(this: &XmlHttpRequest) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onreadystatechange ) ]
    #[doc = "Setter for the `onreadystatechange` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/onreadystatechange)\n\n*This API requires the following crate features to be activated: `XmlHttpRequest`*"]
    pub fn set_onreadystatechange(this: &XmlHttpRequest, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = readyState ) ]
    #[doc = "Getter for the `readyState` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/readyState)\n\n*This API requires the following crate features to be activated: `XmlHttpRequest`*"]
    pub fn ready_state(this: &XmlHttpRequest) -> u16;
    # [ wasm_bindgen ( structural , method , getter , js_name = timeout ) ]
    #[doc = "Getter for the `timeout` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/timeout)\n\n*This API requires the following crate features to be activated: `XmlHttpRequest`*"]
    pub fn timeout(this: &XmlHttpRequest) -> u32;
    # [ wasm_bindgen ( structural , method , setter , js_name = timeout ) ]
    #[doc = "Setter for the `timeout` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/timeout)\n\n*This API requires the following crate features to be activated: `XmlHttpRequest`*"]
    pub fn set_timeout(this: &XmlHttpRequest, value: u32);
    # [ wasm_bindgen ( structural , method , getter , js_name = withCredentials ) ]
    #[doc = "Getter for the `withCredentials` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/withCredentials)\n\n*This API requires the following crate features to be activated: `XmlHttpRequest`*"]
    pub fn with_credentials(this: &XmlHttpRequest) -> bool;
    # [ wasm_bindgen ( structural , method , setter , js_name = withCredentials ) ]
    #[doc = "Setter for the `withCredentials` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/withCredentials)\n\n*This API requires the following crate features to be activated: `XmlHttpRequest`*"]
    pub fn set_with_credentials(this: &XmlHttpRequest, value: bool);
    # [ wasm_bindgen ( structural , catch , method , getter , js_name = upload ) ]
    #[cfg(feature = "XmlHttpRequestUpload")]
    #[doc = "Getter for the `upload` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/upload)\n\n*This API requires the following crate features to be activated: `XmlHttpRequest`, `XmlHttpRequestUpload`*"]
    pub fn upload(this: &XmlHttpRequest) -> Result<XmlHttpRequestUpload, JsValue>;
    # [ wasm_bindgen ( structural , method , getter , js_name = responseURL ) ]
    #[doc = "Getter for the `responseURL` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/responseURL)\n\n*This API requires the following crate features to be activated: `XmlHttpRequest`*"]
    pub fn response_url(this: &XmlHttpRequest) -> String;
    # [ wasm_bindgen ( structural , catch , method , getter , js_name = status ) ]
    #[doc = "Getter for the `status` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/status)\n\n*This API requires the following crate features to be activated: `XmlHttpRequest`*"]
    pub fn status(this: &XmlHttpRequest) -> Result<u16, JsValue>;
    # [ wasm_bindgen ( structural , catch , method , getter , js_name = statusText ) ]
    #[doc = "Getter for the `statusText` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/statusText)\n\n*This API requires the following crate features to be activated: `XmlHttpRequest`*"]
    pub fn status_text(this: &XmlHttpRequest) -> Result<String, JsValue>;
    # [ wasm_bindgen ( structural , method , getter , js_name = responseType ) ]
    #[cfg(feature = "XmlHttpRequestResponseType")]
    #[doc = "Getter for the `responseType` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/responseType)\n\n*This API requires the following crate features to be activated: `XmlHttpRequest`, `XmlHttpRequestResponseType`*"]
    pub fn response_type(this: &XmlHttpRequest) -> XmlHttpRequestResponseType;
    # [ wasm_bindgen ( structural , method , setter , js_name = responseType ) ]
    #[cfg(feature = "XmlHttpRequestResponseType")]
    #[doc = "Setter for the `responseType` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/responseType)\n\n*This API requires the following crate features to be activated: `XmlHttpRequest`, `XmlHttpRequestResponseType`*"]
    pub fn set_response_type(this: &XmlHttpRequest, value: XmlHttpRequestResponseType);
    # [ wasm_bindgen ( structural , catch , method , getter , js_name = response ) ]
    #[doc = "Getter for the `response` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/response)\n\n*This API requires the following crate features to be activated: `XmlHttpRequest`*"]
    pub fn response(this: &XmlHttpRequest) -> Result<::wasm_bindgen::JsValue, JsValue>;
    # [ wasm_bindgen ( structural , catch , method , getter , js_name = responseText ) ]
    #[doc = "Getter for the `responseText` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/responseText)\n\n*This API requires the following crate features to be activated: `XmlHttpRequest`*"]
    pub fn response_text(this: &XmlHttpRequest) -> Result<Option<String>, JsValue>;
    # [ wasm_bindgen ( structural , catch , method , getter , js_name = responseXML ) ]
    #[cfg(feature = "Document")]
    #[doc = "Getter for the `responseXML` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/responseXML)\n\n*This API requires the following crate features to be activated: `Document`, `XmlHttpRequest`*"]
    pub fn response_xml(this: &XmlHttpRequest) -> Result<Option<Document>, JsValue>;
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new XmlHttpRequest(..)` constructor, creating a new instance of `XmlHttpRequest`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/XMLHttpRequest)\n\n*This API requires the following crate features to be activated: `XmlHttpRequest`*"]
    pub fn new(this: &XmlHttpRequest) -> Result<XmlHttpRequest, JsValue>;
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new XmlHttpRequest(..)` constructor, creating a new instance of `XmlHttpRequest`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/XMLHttpRequest)\n\n*This API requires the following crate features to be activated: `XmlHttpRequest`*"]
    pub fn new_with_ignored(
        this: &XmlHttpRequest,
        ignored: &str,
    ) -> Result<XmlHttpRequest, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = abort ) ]
    #[doc = "The `abort()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/abort)\n\n*This API requires the following crate features to be activated: `XmlHttpRequest`*"]
    pub fn abort(this: &XmlHttpRequest) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = getAllResponseHeaders ) ]
    #[doc = "The `getAllResponseHeaders()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/getAllResponseHeaders)\n\n*This API requires the following crate features to be activated: `XmlHttpRequest`*"]
    pub fn get_all_response_headers(this: &XmlHttpRequest) -> Result<String, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = getResponseHeader ) ]
    #[doc = "The `getResponseHeader()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/getResponseHeader)\n\n*This API requires the following crate features to be activated: `XmlHttpRequest`*"]
    pub fn get_response_header(
        this: &XmlHttpRequest,
        header: &str,
    ) -> Result<Option<String>, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = open ) ]
    #[doc = "The `open()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/open)\n\n*This API requires the following crate features to be activated: `XmlHttpRequest`*"]
    pub fn open(this: &XmlHttpRequest, method: &str, url: &str) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = open ) ]
    #[doc = "The `open()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/open)\n\n*This API requires the following crate features to be activated: `XmlHttpRequest`*"]
    pub fn open_with_async(
        this: &XmlHttpRequest,
        method: &str,
        url: &str,
        r#async: bool,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = open ) ]
    #[doc = "The `open()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/open)\n\n*This API requires the following crate features to be activated: `XmlHttpRequest`*"]
    pub fn open_with_async_and_user(
        this: &XmlHttpRequest,
        method: &str,
        url: &str,
        r#async: bool,
        user: Option<&str>,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = open ) ]
    #[doc = "The `open()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/open)\n\n*This API requires the following crate features to be activated: `XmlHttpRequest`*"]
    pub fn open_with_async_and_user_and_password(
        this: &XmlHttpRequest,
        method: &str,
        url: &str,
        r#async: bool,
        user: Option<&str>,
        password: Option<&str>,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = overrideMimeType ) ]
    #[doc = "The `overrideMimeType()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/overrideMimeType)\n\n*This API requires the following crate features to be activated: `XmlHttpRequest`*"]
    pub fn override_mime_type(this: &XmlHttpRequest, mime: &str) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = send ) ]
    #[doc = "The `send()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/send)\n\n*This API requires the following crate features to be activated: `XmlHttpRequest`*"]
    pub fn send(this: &XmlHttpRequest) -> Result<(), JsValue>;
    #[cfg(feature = "Document")]
    # [ wasm_bindgen ( catch , method , structural , js_name = send ) ]
    #[doc = "The `send()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/send)\n\n*This API requires the following crate features to be activated: `Document`, `XmlHttpRequest`*"]
    pub fn send_with_opt_document(
        this: &XmlHttpRequest,
        body: Option<&Document>,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "Blob")]
    # [ wasm_bindgen ( catch , method , structural , js_name = send ) ]
    #[doc = "The `send()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/send)\n\n*This API requires the following crate features to be activated: `Blob`, `XmlHttpRequest`*"]
    pub fn send_with_opt_blob(this: &XmlHttpRequest, body: Option<&Blob>) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = send ) ]
    #[doc = "The `send()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/send)\n\n*This API requires the following crate features to be activated: `XmlHttpRequest`*"]
    pub fn send_with_opt_buffer_source(
        this: &XmlHttpRequest,
        body: Option<&::js_sys::Object>,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = send ) ]
    #[doc = "The `send()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/send)\n\n*This API requires the following crate features to be activated: `XmlHttpRequest`*"]
    pub fn send_with_opt_u8_array(
        this: &XmlHttpRequest,
        body: Option<&mut [u8]>,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "FormData")]
    # [ wasm_bindgen ( catch , method , structural , js_name = send ) ]
    #[doc = "The `send()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/send)\n\n*This API requires the following crate features to be activated: `FormData`, `XmlHttpRequest`*"]
    pub fn send_with_opt_form_data(
        this: &XmlHttpRequest,
        body: Option<&FormData>,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "UrlSearchParams")]
    # [ wasm_bindgen ( catch , method , structural , js_name = send ) ]
    #[doc = "The `send()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/send)\n\n*This API requires the following crate features to be activated: `UrlSearchParams`, `XmlHttpRequest`*"]
    pub fn send_with_opt_url_search_params(
        this: &XmlHttpRequest,
        body: Option<&UrlSearchParams>,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = send ) ]
    #[doc = "The `send()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/send)\n\n*This API requires the following crate features to be activated: `XmlHttpRequest`*"]
    pub fn send_with_opt_str(this: &XmlHttpRequest, body: Option<&str>) -> Result<(), JsValue>;
    #[cfg(feature = "ReadableStream")]
    # [ wasm_bindgen ( catch , method , structural , js_name = send ) ]
    #[doc = "The `send()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/send)\n\n*This API requires the following crate features to be activated: `ReadableStream`, `XmlHttpRequest`*"]
    pub fn send_with_opt_readable_stream(
        this: &XmlHttpRequest,
        body: Option<&ReadableStream>,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = setRequestHeader ) ]
    #[doc = "The `setRequestHeader()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/setRequestHeader)\n\n*This API requires the following crate features to be activated: `XmlHttpRequest`*"]
    pub fn set_request_header(
        this: &XmlHttpRequest,
        header: &str,
        value: &str,
    ) -> Result<(), JsValue>;
}
impl XmlHttpRequest {
    pub const UNSENT: u16 = 0i64 as u16;
    pub const OPENED: u16 = 1u64 as u16;
    pub const HEADERS_RECEIVED: u16 = 2u64 as u16;
    pub const LOADING: u16 = 3u64 as u16;
    pub const DONE: u16 = 4u64 as u16;
}
