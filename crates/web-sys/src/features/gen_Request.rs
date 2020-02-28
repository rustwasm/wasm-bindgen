use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = Request , typescript_name = Request ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `Request` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Request)\n\n*This API requires the following crate features to be activated: `Request`*"]
    pub type Request;
    # [ wasm_bindgen ( structural , method , getter , js_name = method ) ]
    #[doc = "Getter for the `method` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Request/method)\n\n*This API requires the following crate features to be activated: `Request`*"]
    pub fn method(this: &Request) -> String;
    # [ wasm_bindgen ( structural , method , getter , js_name = url ) ]
    #[doc = "Getter for the `url` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Request/url)\n\n*This API requires the following crate features to be activated: `Request`*"]
    pub fn url(this: &Request) -> String;
    # [ wasm_bindgen ( structural , method , getter , js_name = headers ) ]
    #[cfg(feature = "Headers")]
    #[doc = "Getter for the `headers` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Request/headers)\n\n*This API requires the following crate features to be activated: `Headers`, `Request`*"]
    pub fn headers(this: &Request) -> Headers;
    # [ wasm_bindgen ( structural , method , getter , js_name = destination ) ]
    #[cfg(feature = "RequestDestination")]
    #[doc = "Getter for the `destination` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Request/destination)\n\n*This API requires the following crate features to be activated: `Request`, `RequestDestination`*"]
    pub fn destination(this: &Request) -> RequestDestination;
    # [ wasm_bindgen ( structural , method , getter , js_name = referrer ) ]
    #[doc = "Getter for the `referrer` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Request/referrer)\n\n*This API requires the following crate features to be activated: `Request`*"]
    pub fn referrer(this: &Request) -> String;
    # [ wasm_bindgen ( structural , method , getter , js_name = referrerPolicy ) ]
    #[cfg(feature = "ReferrerPolicy")]
    #[doc = "Getter for the `referrerPolicy` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Request/referrerPolicy)\n\n*This API requires the following crate features to be activated: `ReferrerPolicy`, `Request`*"]
    pub fn referrer_policy(this: &Request) -> ReferrerPolicy;
    # [ wasm_bindgen ( structural , method , getter , js_name = mode ) ]
    #[cfg(feature = "RequestMode")]
    #[doc = "Getter for the `mode` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Request/mode)\n\n*This API requires the following crate features to be activated: `Request`, `RequestMode`*"]
    pub fn mode(this: &Request) -> RequestMode;
    # [ wasm_bindgen ( structural , method , getter , js_name = credentials ) ]
    #[cfg(feature = "RequestCredentials")]
    #[doc = "Getter for the `credentials` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Request/credentials)\n\n*This API requires the following crate features to be activated: `Request`, `RequestCredentials`*"]
    pub fn credentials(this: &Request) -> RequestCredentials;
    # [ wasm_bindgen ( structural , method , getter , js_name = cache ) ]
    #[cfg(feature = "RequestCache")]
    #[doc = "Getter for the `cache` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Request/cache)\n\n*This API requires the following crate features to be activated: `Request`, `RequestCache`*"]
    pub fn cache(this: &Request) -> RequestCache;
    # [ wasm_bindgen ( structural , method , getter , js_name = redirect ) ]
    #[cfg(feature = "RequestRedirect")]
    #[doc = "Getter for the `redirect` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Request/redirect)\n\n*This API requires the following crate features to be activated: `Request`, `RequestRedirect`*"]
    pub fn redirect(this: &Request) -> RequestRedirect;
    # [ wasm_bindgen ( structural , method , getter , js_name = integrity ) ]
    #[doc = "Getter for the `integrity` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Request/integrity)\n\n*This API requires the following crate features to be activated: `Request`*"]
    pub fn integrity(this: &Request) -> String;
    # [ wasm_bindgen ( structural , method , getter , js_name = signal ) ]
    #[cfg(feature = "AbortSignal")]
    #[doc = "Getter for the `signal` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Request/signal)\n\n*This API requires the following crate features to be activated: `AbortSignal`, `Request`*"]
    pub fn signal(this: &Request) -> AbortSignal;
    # [ wasm_bindgen ( structural , method , getter , js_name = bodyUsed ) ]
    #[doc = "Getter for the `bodyUsed` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Request/bodyUsed)\n\n*This API requires the following crate features to be activated: `Request`*"]
    pub fn body_used(this: &Request) -> bool;
    # [ wasm_bindgen ( structural , method , getter , js_name = body ) ]
    #[cfg(feature = "ReadableStream")]
    #[doc = "Getter for the `body` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Request/body)\n\n*This API requires the following crate features to be activated: `ReadableStream`, `Request`*"]
    pub fn body(this: &Request) -> Option<ReadableStream>;
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new Request(..)` constructor, creating a new instance of `Request`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Request/Request)\n\n*This API requires the following crate features to be activated: `Request`*"]
    pub fn new_with_request(this: &Request, input: &Request) -> Result<Request, JsValue>;
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new Request(..)` constructor, creating a new instance of `Request`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Request/Request)\n\n*This API requires the following crate features to be activated: `Request`*"]
    pub fn new_with_str(this: &Request, input: &str) -> Result<Request, JsValue>;
    #[cfg(feature = "RequestInit")]
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new Request(..)` constructor, creating a new instance of `Request`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Request/Request)\n\n*This API requires the following crate features to be activated: `Request`, `RequestInit`*"]
    pub fn new_with_request_and_init(
        this: &Request,
        input: &Request,
        init: &RequestInit,
    ) -> Result<Request, JsValue>;
    #[cfg(feature = "RequestInit")]
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new Request(..)` constructor, creating a new instance of `Request`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Request/Request)\n\n*This API requires the following crate features to be activated: `Request`, `RequestInit`*"]
    pub fn new_with_str_and_init(
        this: &Request,
        input: &str,
        init: &RequestInit,
    ) -> Result<Request, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = clone ) ]
    #[doc = "The `clone()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Request/clone)\n\n*This API requires the following crate features to be activated: `Request`*"]
    pub fn clone(this: &Request) -> Result<Request, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = arrayBuffer ) ]
    #[doc = "The `arrayBuffer()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Request/arrayBuffer)\n\n*This API requires the following crate features to be activated: `Request`*"]
    pub fn array_buffer(this: &Request) -> Result<::js_sys::Promise, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = blob ) ]
    #[doc = "The `blob()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Request/blob)\n\n*This API requires the following crate features to be activated: `Request`*"]
    pub fn blob(this: &Request) -> Result<::js_sys::Promise, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = formData ) ]
    #[doc = "The `formData()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Request/formData)\n\n*This API requires the following crate features to be activated: `Request`*"]
    pub fn form_data(this: &Request) -> Result<::js_sys::Promise, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = json ) ]
    #[doc = "The `json()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Request/json)\n\n*This API requires the following crate features to be activated: `Request`*"]
    pub fn json(this: &Request) -> Result<::js_sys::Promise, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = text ) ]
    #[doc = "The `text()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Request/text)\n\n*This API requires the following crate features to be activated: `Request`*"]
    pub fn text(this: &Request) -> Result<::js_sys::Promise, JsValue>;
}
