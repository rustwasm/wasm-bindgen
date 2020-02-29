use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = Request , typescript_name = Request ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `Request` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Request)
    ///
    ///*This API requires the following crate features to be activated: `Request`*
    pub type Request;

    # [ wasm_bindgen ( structural , method , getter , js_class = "Request" , js_name = method ) ]
    ///Getter for the `method` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Request/method)
    ///
    ///*This API requires the following crate features to be activated: `Request`*
    pub fn method(this: &Request) -> String;

    # [ wasm_bindgen ( structural , method , getter , js_class = "Request" , js_name = url ) ]
    ///Getter for the `url` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Request/url)
    ///
    ///*This API requires the following crate features to be activated: `Request`*
    pub fn url(this: &Request) -> String;

    #[cfg(feature = "Headers")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "Request" , js_name = headers ) ]
    ///Getter for the `headers` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Request/headers)
    ///
    ///*This API requires the following crate features to be activated: `Headers`, `Request`*
    pub fn headers(this: &Request) -> Headers;

    #[cfg(feature = "RequestDestination")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "Request" , js_name = destination ) ]
    ///Getter for the `destination` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Request/destination)
    ///
    ///*This API requires the following crate features to be activated: `Request`, `RequestDestination`*
    pub fn destination(this: &Request) -> RequestDestination;

    # [ wasm_bindgen ( structural , method , getter , js_class = "Request" , js_name = referrer ) ]
    ///Getter for the `referrer` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Request/referrer)
    ///
    ///*This API requires the following crate features to be activated: `Request`*
    pub fn referrer(this: &Request) -> String;

    #[cfg(feature = "ReferrerPolicy")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "Request" , js_name = referrerPolicy ) ]
    ///Getter for the `referrerPolicy` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Request/referrerPolicy)
    ///
    ///*This API requires the following crate features to be activated: `ReferrerPolicy`, `Request`*
    pub fn referrer_policy(this: &Request) -> ReferrerPolicy;

    #[cfg(feature = "RequestMode")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "Request" , js_name = mode ) ]
    ///Getter for the `mode` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Request/mode)
    ///
    ///*This API requires the following crate features to be activated: `Request`, `RequestMode`*
    pub fn mode(this: &Request) -> RequestMode;

    #[cfg(feature = "RequestCredentials")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "Request" , js_name = credentials ) ]
    ///Getter for the `credentials` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Request/credentials)
    ///
    ///*This API requires the following crate features to be activated: `Request`, `RequestCredentials`*
    pub fn credentials(this: &Request) -> RequestCredentials;

    #[cfg(feature = "RequestCache")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "Request" , js_name = cache ) ]
    ///Getter for the `cache` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Request/cache)
    ///
    ///*This API requires the following crate features to be activated: `Request`, `RequestCache`*
    pub fn cache(this: &Request) -> RequestCache;

    #[cfg(feature = "RequestRedirect")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "Request" , js_name = redirect ) ]
    ///Getter for the `redirect` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Request/redirect)
    ///
    ///*This API requires the following crate features to be activated: `Request`, `RequestRedirect`*
    pub fn redirect(this: &Request) -> RequestRedirect;

    # [ wasm_bindgen ( structural , method , getter , js_class = "Request" , js_name = integrity ) ]
    ///Getter for the `integrity` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Request/integrity)
    ///
    ///*This API requires the following crate features to be activated: `Request`*
    pub fn integrity(this: &Request) -> String;

    #[cfg(feature = "AbortSignal")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "Request" , js_name = signal ) ]
    ///Getter for the `signal` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Request/signal)
    ///
    ///*This API requires the following crate features to be activated: `AbortSignal`, `Request`*
    pub fn signal(this: &Request) -> AbortSignal;

    # [ wasm_bindgen ( structural , method , getter , js_class = "Request" , js_name = bodyUsed ) ]
    ///Getter for the `bodyUsed` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Request/bodyUsed)
    ///
    ///*This API requires the following crate features to be activated: `Request`*
    pub fn body_used(this: &Request) -> bool;

    #[cfg(feature = "ReadableStream")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "Request" , js_name = body ) ]
    ///Getter for the `body` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Request/body)
    ///
    ///*This API requires the following crate features to be activated: `ReadableStream`, `Request`*
    pub fn body(this: &Request) -> Option<ReadableStream>;

    #[wasm_bindgen(catch, constructor, js_class = "Request")]
    ///The `new Request(..)` constructor, creating a new instance of `Request`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Request/Request)
    ///
    ///*This API requires the following crate features to be activated: `Request`*
    pub fn new_with_request(input: &Request) -> Result<Request, JsValue>;

    #[wasm_bindgen(catch, constructor, js_class = "Request")]
    ///The `new Request(..)` constructor, creating a new instance of `Request`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Request/Request)
    ///
    ///*This API requires the following crate features to be activated: `Request`*
    pub fn new_with_str(input: &str) -> Result<Request, JsValue>;

    #[cfg(feature = "RequestInit")]
    #[wasm_bindgen(catch, constructor, js_class = "Request")]
    ///The `new Request(..)` constructor, creating a new instance of `Request`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Request/Request)
    ///
    ///*This API requires the following crate features to be activated: `Request`, `RequestInit`*
    pub fn new_with_request_and_init(
        input: &Request,
        init: &RequestInit,
    ) -> Result<Request, JsValue>;

    #[cfg(feature = "RequestInit")]
    #[wasm_bindgen(catch, constructor, js_class = "Request")]
    ///The `new Request(..)` constructor, creating a new instance of `Request`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Request/Request)
    ///
    ///*This API requires the following crate features to be activated: `Request`, `RequestInit`*
    pub fn new_with_str_and_init(input: &str, init: &RequestInit) -> Result<Request, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Request" , js_name = clone ) ]
    ///The `clone()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Request/clone)
    ///
    ///*This API requires the following crate features to be activated: `Request`*
    pub fn clone(this: &Request) -> Result<Request, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Request" , js_name = arrayBuffer ) ]
    ///The `arrayBuffer()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Request/arrayBuffer)
    ///
    ///*This API requires the following crate features to be activated: `Request`*
    pub fn array_buffer(this: &Request) -> Result<::js_sys::Promise, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Request" , js_name = blob ) ]
    ///The `blob()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Request/blob)
    ///
    ///*This API requires the following crate features to be activated: `Request`*
    pub fn blob(this: &Request) -> Result<::js_sys::Promise, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Request" , js_name = formData ) ]
    ///The `formData()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Request/formData)
    ///
    ///*This API requires the following crate features to be activated: `Request`*
    pub fn form_data(this: &Request) -> Result<::js_sys::Promise, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Request" , js_name = json ) ]
    ///The `json()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Request/json)
    ///
    ///*This API requires the following crate features to be activated: `Request`*
    pub fn json(this: &Request) -> Result<::js_sys::Promise, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Request" , js_name = text ) ]
    ///The `text()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Request/text)
    ///
    ///*This API requires the following crate features to be activated: `Request`*
    pub fn text(this: &Request) -> Result<::js_sys::Promise, JsValue>;

}
