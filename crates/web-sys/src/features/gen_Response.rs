use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = Response , typescript_name = Response ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `Response` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response)
    ///
    ///*This API requires the following crate features to be activated: `Response`*
    pub type Response;

    #[cfg(feature = "ResponseType")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "Response" , js_name = type ) ]
    ///Getter for the `type` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/type)
    ///
    ///*This API requires the following crate features to be activated: `Response`, `ResponseType`*
    pub fn type_(this: &Response) -> ResponseType;

    # [ wasm_bindgen ( structural , method , getter , js_class = "Response" , js_name = url ) ]
    ///Getter for the `url` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/url)
    ///
    ///*This API requires the following crate features to be activated: `Response`*
    pub fn url(this: &Response) -> String;

    # [ wasm_bindgen ( structural , method , getter , js_class = "Response" , js_name = redirected ) ]
    ///Getter for the `redirected` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/redirected)
    ///
    ///*This API requires the following crate features to be activated: `Response`*
    pub fn redirected(this: &Response) -> bool;

    # [ wasm_bindgen ( structural , method , getter , js_class = "Response" , js_name = status ) ]
    ///Getter for the `status` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/status)
    ///
    ///*This API requires the following crate features to be activated: `Response`*
    pub fn status(this: &Response) -> u16;

    # [ wasm_bindgen ( structural , method , getter , js_class = "Response" , js_name = ok ) ]
    ///Getter for the `ok` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/ok)
    ///
    ///*This API requires the following crate features to be activated: `Response`*
    pub fn ok(this: &Response) -> bool;

    # [ wasm_bindgen ( structural , method , getter , js_class = "Response" , js_name = statusText ) ]
    ///Getter for the `statusText` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/statusText)
    ///
    ///*This API requires the following crate features to be activated: `Response`*
    pub fn status_text(this: &Response) -> String;

    #[cfg(feature = "Headers")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "Response" , js_name = headers ) ]
    ///Getter for the `headers` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/headers)
    ///
    ///*This API requires the following crate features to be activated: `Headers`, `Response`*
    pub fn headers(this: &Response) -> Headers;

    # [ wasm_bindgen ( structural , method , getter , js_class = "Response" , js_name = bodyUsed ) ]
    ///Getter for the `bodyUsed` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/bodyUsed)
    ///
    ///*This API requires the following crate features to be activated: `Response`*
    pub fn body_used(this: &Response) -> bool;

    #[cfg(feature = "ReadableStream")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "Response" , js_name = body ) ]
    ///Getter for the `body` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/body)
    ///
    ///*This API requires the following crate features to be activated: `ReadableStream`, `Response`*
    pub fn body(this: &Response) -> Option<ReadableStream>;

    #[wasm_bindgen(catch, constructor, js_class = "Response")]
    ///The `new Response(..)` constructor, creating a new instance of `Response`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/Response)
    ///
    ///*This API requires the following crate features to be activated: `Response`*
    pub fn new() -> Result<Response, JsValue>;

    #[cfg(feature = "Blob")]
    #[wasm_bindgen(catch, constructor, js_class = "Response")]
    ///The `new Response(..)` constructor, creating a new instance of `Response`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/Response)
    ///
    ///*This API requires the following crate features to be activated: `Blob`, `Response`*
    pub fn new_with_opt_blob(body: Option<&Blob>) -> Result<Response, JsValue>;

    #[wasm_bindgen(catch, constructor, js_class = "Response")]
    ///The `new Response(..)` constructor, creating a new instance of `Response`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/Response)
    ///
    ///*This API requires the following crate features to be activated: `Response`*
    pub fn new_with_opt_buffer_source(body: Option<&::js_sys::Object>)
        -> Result<Response, JsValue>;

    #[wasm_bindgen(catch, constructor, js_class = "Response")]
    ///The `new Response(..)` constructor, creating a new instance of `Response`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/Response)
    ///
    ///*This API requires the following crate features to be activated: `Response`*
    pub fn new_with_opt_u8_array(body: Option<&mut [u8]>) -> Result<Response, JsValue>;

    #[cfg(feature = "FormData")]
    #[wasm_bindgen(catch, constructor, js_class = "Response")]
    ///The `new Response(..)` constructor, creating a new instance of `Response`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/Response)
    ///
    ///*This API requires the following crate features to be activated: `FormData`, `Response`*
    pub fn new_with_opt_form_data(body: Option<&FormData>) -> Result<Response, JsValue>;

    #[cfg(feature = "UrlSearchParams")]
    #[wasm_bindgen(catch, constructor, js_class = "Response")]
    ///The `new Response(..)` constructor, creating a new instance of `Response`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/Response)
    ///
    ///*This API requires the following crate features to be activated: `Response`, `UrlSearchParams`*
    pub fn new_with_opt_url_search_params(
        body: Option<&UrlSearchParams>,
    ) -> Result<Response, JsValue>;

    #[wasm_bindgen(catch, constructor, js_class = "Response")]
    ///The `new Response(..)` constructor, creating a new instance of `Response`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/Response)
    ///
    ///*This API requires the following crate features to be activated: `Response`*
    pub fn new_with_opt_str(body: Option<&str>) -> Result<Response, JsValue>;

    #[cfg(feature = "ReadableStream")]
    #[wasm_bindgen(catch, constructor, js_class = "Response")]
    ///The `new Response(..)` constructor, creating a new instance of `Response`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/Response)
    ///
    ///*This API requires the following crate features to be activated: `ReadableStream`, `Response`*
    pub fn new_with_opt_readable_stream(body: Option<&ReadableStream>)
        -> Result<Response, JsValue>;

    #[cfg(all(feature = "Blob", feature = "ResponseInit",))]
    #[wasm_bindgen(catch, constructor, js_class = "Response")]
    ///The `new Response(..)` constructor, creating a new instance of `Response`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/Response)
    ///
    ///*This API requires the following crate features to be activated: `Blob`, `Response`, `ResponseInit`*
    pub fn new_with_opt_blob_and_init(
        body: Option<&Blob>,
        init: &ResponseInit,
    ) -> Result<Response, JsValue>;

    #[cfg(feature = "ResponseInit")]
    #[wasm_bindgen(catch, constructor, js_class = "Response")]
    ///The `new Response(..)` constructor, creating a new instance of `Response`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/Response)
    ///
    ///*This API requires the following crate features to be activated: `Response`, `ResponseInit`*
    pub fn new_with_opt_buffer_source_and_init(
        body: Option<&::js_sys::Object>,
        init: &ResponseInit,
    ) -> Result<Response, JsValue>;

    #[cfg(feature = "ResponseInit")]
    #[wasm_bindgen(catch, constructor, js_class = "Response")]
    ///The `new Response(..)` constructor, creating a new instance of `Response`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/Response)
    ///
    ///*This API requires the following crate features to be activated: `Response`, `ResponseInit`*
    pub fn new_with_opt_u8_array_and_init(
        body: Option<&mut [u8]>,
        init: &ResponseInit,
    ) -> Result<Response, JsValue>;

    #[cfg(all(feature = "FormData", feature = "ResponseInit",))]
    #[wasm_bindgen(catch, constructor, js_class = "Response")]
    ///The `new Response(..)` constructor, creating a new instance of `Response`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/Response)
    ///
    ///*This API requires the following crate features to be activated: `FormData`, `Response`, `ResponseInit`*
    pub fn new_with_opt_form_data_and_init(
        body: Option<&FormData>,
        init: &ResponseInit,
    ) -> Result<Response, JsValue>;

    #[cfg(all(feature = "ResponseInit", feature = "UrlSearchParams",))]
    #[wasm_bindgen(catch, constructor, js_class = "Response")]
    ///The `new Response(..)` constructor, creating a new instance of `Response`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/Response)
    ///
    ///*This API requires the following crate features to be activated: `Response`, `ResponseInit`, `UrlSearchParams`*
    pub fn new_with_opt_url_search_params_and_init(
        body: Option<&UrlSearchParams>,
        init: &ResponseInit,
    ) -> Result<Response, JsValue>;

    #[cfg(feature = "ResponseInit")]
    #[wasm_bindgen(catch, constructor, js_class = "Response")]
    ///The `new Response(..)` constructor, creating a new instance of `Response`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/Response)
    ///
    ///*This API requires the following crate features to be activated: `Response`, `ResponseInit`*
    pub fn new_with_opt_str_and_init(
        body: Option<&str>,
        init: &ResponseInit,
    ) -> Result<Response, JsValue>;

    #[cfg(all(feature = "ReadableStream", feature = "ResponseInit",))]
    #[wasm_bindgen(catch, constructor, js_class = "Response")]
    ///The `new Response(..)` constructor, creating a new instance of `Response`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/Response)
    ///
    ///*This API requires the following crate features to be activated: `ReadableStream`, `Response`, `ResponseInit`*
    pub fn new_with_opt_readable_stream_and_init(
        body: Option<&ReadableStream>,
        init: &ResponseInit,
    ) -> Result<Response, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Response" , js_name = clone ) ]
    ///The `clone()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/clone)
    ///
    ///*This API requires the following crate features to be activated: `Response`*
    pub fn clone(this: &Response) -> Result<Response, JsValue>;

    # [ wasm_bindgen ( static_method_of = Response , js_class = "Response" , js_name = error ) ]
    ///The `error()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/error)
    ///
    ///*This API requires the following crate features to be activated: `Response`*
    pub fn error() -> Response;

    # [ wasm_bindgen ( catch , static_method_of = Response , js_class = "Response" , js_name = redirect ) ]
    ///The `redirect()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/redirect)
    ///
    ///*This API requires the following crate features to be activated: `Response`*
    pub fn redirect(url: &str) -> Result<Response, JsValue>;

    # [ wasm_bindgen ( catch , static_method_of = Response , js_class = "Response" , js_name = redirect ) ]
    ///The `redirect()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/redirect)
    ///
    ///*This API requires the following crate features to be activated: `Response`*
    pub fn redirect_with_status(url: &str, status: u16) -> Result<Response, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Response" , js_name = arrayBuffer ) ]
    ///The `arrayBuffer()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/arrayBuffer)
    ///
    ///*This API requires the following crate features to be activated: `Response`*
    pub fn array_buffer(this: &Response) -> Result<::js_sys::Promise, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Response" , js_name = blob ) ]
    ///The `blob()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/blob)
    ///
    ///*This API requires the following crate features to be activated: `Response`*
    pub fn blob(this: &Response) -> Result<::js_sys::Promise, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Response" , js_name = formData ) ]
    ///The `formData()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/formData)
    ///
    ///*This API requires the following crate features to be activated: `Response`*
    pub fn form_data(this: &Response) -> Result<::js_sys::Promise, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Response" , js_name = json ) ]
    ///The `json()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/json)
    ///
    ///*This API requires the following crate features to be activated: `Response`*
    pub fn json(this: &Response) -> Result<::js_sys::Promise, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Response" , js_name = text ) ]
    ///The `text()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/text)
    ///
    ///*This API requires the following crate features to be activated: `Response`*
    pub fn text(this: &Response) -> Result<::js_sys::Promise, JsValue>;

}
