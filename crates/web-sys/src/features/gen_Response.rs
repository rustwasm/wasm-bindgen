use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = Response , typescript_name = Response ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `Response` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response)\n\n*This API requires the following crate features to be activated: `Response`*"]
    pub type Response;
    # [ wasm_bindgen ( structural , method , getter , js_name = type ) ]
    #[cfg(feature = "ResponseType")]
    #[doc = "Getter for the `type` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/type)\n\n*This API requires the following crate features to be activated: `Response`, `ResponseType`*"]
    pub fn type_(this: &Response) -> ResponseType;
    # [ wasm_bindgen ( structural , method , getter , js_name = url ) ]
    #[doc = "Getter for the `url` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/url)\n\n*This API requires the following crate features to be activated: `Response`*"]
    pub fn url(this: &Response) -> String;
    # [ wasm_bindgen ( structural , method , getter , js_name = redirected ) ]
    #[doc = "Getter for the `redirected` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/redirected)\n\n*This API requires the following crate features to be activated: `Response`*"]
    pub fn redirected(this: &Response) -> bool;
    # [ wasm_bindgen ( structural , method , getter , js_name = status ) ]
    #[doc = "Getter for the `status` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/status)\n\n*This API requires the following crate features to be activated: `Response`*"]
    pub fn status(this: &Response) -> u16;
    # [ wasm_bindgen ( structural , method , getter , js_name = ok ) ]
    #[doc = "Getter for the `ok` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/ok)\n\n*This API requires the following crate features to be activated: `Response`*"]
    pub fn ok(this: &Response) -> bool;
    # [ wasm_bindgen ( structural , method , getter , js_name = statusText ) ]
    #[doc = "Getter for the `statusText` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/statusText)\n\n*This API requires the following crate features to be activated: `Response`*"]
    pub fn status_text(this: &Response) -> String;
    # [ wasm_bindgen ( structural , method , getter , js_name = headers ) ]
    #[cfg(feature = "Headers")]
    #[doc = "Getter for the `headers` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/headers)\n\n*This API requires the following crate features to be activated: `Headers`, `Response`*"]
    pub fn headers(this: &Response) -> Headers;
    # [ wasm_bindgen ( structural , method , getter , js_name = bodyUsed ) ]
    #[doc = "Getter for the `bodyUsed` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/bodyUsed)\n\n*This API requires the following crate features to be activated: `Response`*"]
    pub fn body_used(this: &Response) -> bool;
    # [ wasm_bindgen ( structural , method , getter , js_name = body ) ]
    #[cfg(feature = "ReadableStream")]
    #[doc = "Getter for the `body` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/body)\n\n*This API requires the following crate features to be activated: `ReadableStream`, `Response`*"]
    pub fn body(this: &Response) -> Option<ReadableStream>;
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new Response(..)` constructor, creating a new instance of `Response`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/Response)\n\n*This API requires the following crate features to be activated: `Response`*"]
    pub fn new(this: &Response) -> Result<Response, JsValue>;
    #[cfg(feature = "Blob")]
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new Response(..)` constructor, creating a new instance of `Response`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/Response)\n\n*This API requires the following crate features to be activated: `Blob`, `Response`*"]
    pub fn new_with_opt_blob(this: &Response, body: Option<&Blob>) -> Result<Response, JsValue>;
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new Response(..)` constructor, creating a new instance of `Response`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/Response)\n\n*This API requires the following crate features to be activated: `Response`*"]
    pub fn new_with_opt_buffer_source(
        this: &Response,
        body: Option<&::js_sys::Object>,
    ) -> Result<Response, JsValue>;
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new Response(..)` constructor, creating a new instance of `Response`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/Response)\n\n*This API requires the following crate features to be activated: `Response`*"]
    pub fn new_with_opt_u8_array(
        this: &Response,
        body: Option<&mut [u8]>,
    ) -> Result<Response, JsValue>;
    #[cfg(feature = "FormData")]
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new Response(..)` constructor, creating a new instance of `Response`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/Response)\n\n*This API requires the following crate features to be activated: `FormData`, `Response`*"]
    pub fn new_with_opt_form_data(
        this: &Response,
        body: Option<&FormData>,
    ) -> Result<Response, JsValue>;
    #[cfg(feature = "UrlSearchParams")]
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new Response(..)` constructor, creating a new instance of `Response`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/Response)\n\n*This API requires the following crate features to be activated: `Response`, `UrlSearchParams`*"]
    pub fn new_with_opt_url_search_params(
        this: &Response,
        body: Option<&UrlSearchParams>,
    ) -> Result<Response, JsValue>;
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new Response(..)` constructor, creating a new instance of `Response`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/Response)\n\n*This API requires the following crate features to be activated: `Response`*"]
    pub fn new_with_opt_str(this: &Response, body: Option<&str>) -> Result<Response, JsValue>;
    #[cfg(feature = "ReadableStream")]
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new Response(..)` constructor, creating a new instance of `Response`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/Response)\n\n*This API requires the following crate features to be activated: `ReadableStream`, `Response`*"]
    pub fn new_with_opt_readable_stream(
        this: &Response,
        body: Option<&ReadableStream>,
    ) -> Result<Response, JsValue>;
    #[cfg(all(feature = "Blob", feature = "ResponseInit",))]
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new Response(..)` constructor, creating a new instance of `Response`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/Response)\n\n*This API requires the following crate features to be activated: `Blob`, `Response`, `ResponseInit`*"]
    pub fn new_with_opt_blob_and_init(
        this: &Response,
        body: Option<&Blob>,
        init: &ResponseInit,
    ) -> Result<Response, JsValue>;
    #[cfg(feature = "ResponseInit")]
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new Response(..)` constructor, creating a new instance of `Response`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/Response)\n\n*This API requires the following crate features to be activated: `Response`, `ResponseInit`*"]
    pub fn new_with_opt_buffer_source_and_init(
        this: &Response,
        body: Option<&::js_sys::Object>,
        init: &ResponseInit,
    ) -> Result<Response, JsValue>;
    #[cfg(feature = "ResponseInit")]
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new Response(..)` constructor, creating a new instance of `Response`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/Response)\n\n*This API requires the following crate features to be activated: `Response`, `ResponseInit`*"]
    pub fn new_with_opt_u8_array_and_init(
        this: &Response,
        body: Option<&mut [u8]>,
        init: &ResponseInit,
    ) -> Result<Response, JsValue>;
    #[cfg(all(feature = "FormData", feature = "ResponseInit",))]
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new Response(..)` constructor, creating a new instance of `Response`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/Response)\n\n*This API requires the following crate features to be activated: `FormData`, `Response`, `ResponseInit`*"]
    pub fn new_with_opt_form_data_and_init(
        this: &Response,
        body: Option<&FormData>,
        init: &ResponseInit,
    ) -> Result<Response, JsValue>;
    #[cfg(all(feature = "ResponseInit", feature = "UrlSearchParams",))]
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new Response(..)` constructor, creating a new instance of `Response`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/Response)\n\n*This API requires the following crate features to be activated: `Response`, `ResponseInit`, `UrlSearchParams`*"]
    pub fn new_with_opt_url_search_params_and_init(
        this: &Response,
        body: Option<&UrlSearchParams>,
        init: &ResponseInit,
    ) -> Result<Response, JsValue>;
    #[cfg(feature = "ResponseInit")]
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new Response(..)` constructor, creating a new instance of `Response`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/Response)\n\n*This API requires the following crate features to be activated: `Response`, `ResponseInit`*"]
    pub fn new_with_opt_str_and_init(
        this: &Response,
        body: Option<&str>,
        init: &ResponseInit,
    ) -> Result<Response, JsValue>;
    #[cfg(all(feature = "ReadableStream", feature = "ResponseInit",))]
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new Response(..)` constructor, creating a new instance of `Response`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/Response)\n\n*This API requires the following crate features to be activated: `ReadableStream`, `Response`, `ResponseInit`*"]
    pub fn new_with_opt_readable_stream_and_init(
        this: &Response,
        body: Option<&ReadableStream>,
        init: &ResponseInit,
    ) -> Result<Response, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = clone ) ]
    #[doc = "The `clone()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/clone)\n\n*This API requires the following crate features to be activated: `Response`*"]
    pub fn clone(this: &Response) -> Result<Response, JsValue>;
    # [ wasm_bindgen ( static_method_of = Response , js_name = error ) ]
    #[doc = "The `error()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/error)\n\n*This API requires the following crate features to be activated: `Response`*"]
    pub fn error() -> Response;
    # [ wasm_bindgen ( catch , static_method_of = Response , js_name = redirect ) ]
    #[doc = "The `redirect()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/redirect)\n\n*This API requires the following crate features to be activated: `Response`*"]
    pub fn redirect(url: &str) -> Result<Response, JsValue>;
    # [ wasm_bindgen ( catch , static_method_of = Response , js_name = redirect ) ]
    #[doc = "The `redirect()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/redirect)\n\n*This API requires the following crate features to be activated: `Response`*"]
    pub fn redirect_with_status(url: &str, status: u16) -> Result<Response, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = arrayBuffer ) ]
    #[doc = "The `arrayBuffer()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/arrayBuffer)\n\n*This API requires the following crate features to be activated: `Response`*"]
    pub fn array_buffer(this: &Response) -> Result<::js_sys::Promise, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = blob ) ]
    #[doc = "The `blob()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/blob)\n\n*This API requires the following crate features to be activated: `Response`*"]
    pub fn blob(this: &Response) -> Result<::js_sys::Promise, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = formData ) ]
    #[doc = "The `formData()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/formData)\n\n*This API requires the following crate features to be activated: `Response`*"]
    pub fn form_data(this: &Response) -> Result<::js_sys::Promise, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = json ) ]
    #[doc = "The `json()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/json)\n\n*This API requires the following crate features to be activated: `Response`*"]
    pub fn json(this: &Response) -> Result<::js_sys::Promise, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = text ) ]
    #[doc = "The `text()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/text)\n\n*This API requires the following crate features to be activated: `Response`*"]
    pub fn text(this: &Response) -> Result<::js_sys::Promise, JsValue>;
}
