#![allow(unused_imports)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = Clients , typescript_type = "Clients")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `Clients` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Clients)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Clients`*"]
    pub type Clients;
    # [wasm_bindgen (method , structural , js_class = "Clients" , js_name = claim)]
    #[doc = "The `claim()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Clients/claim)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Clients`*"]
    #[doc = ""]
    #[doc = "Return value: There is additional information in the IDL file about the content of the promise, but it can not yet be explained any better."]
    pub fn claim(this: &Clients) -> ::js_sys::Promise;
    # [wasm_bindgen (method , structural , js_class = "Clients" , js_name = get)]
    #[doc = "The `get()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Clients/get)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Clients`*"]
    #[doc = ""]
    #[doc = "Return value: While the Promise can produce any JsValue as far as the type system is concerned, practically it is expected to contain a <code>[::wasm_bindgen::JsValue]</code>. It can be converted like `<code>let result: [::wasm_bindgen::JsValue] = result.await.into();</code>."]
    pub fn get(this: &Clients, id: &str) -> ::js_sys::Promise;
    # [wasm_bindgen (method , structural , js_class = "Clients" , js_name = matchAll)]
    #[doc = "The `matchAll()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Clients/matchAll)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Clients`*"]
    #[doc = ""]
    #[doc = "Return value: While the Promise can produce any JsValue as far as the type system is concerned, practically it is expected to contain a <code>[::js_sys::Array]</code>. It can be converted like `<code>let result: [::js_sys::Array] = result.await.into();</code>. More information is available in the source IDL file."]
    pub fn match_all(this: &Clients) -> ::js_sys::Promise;
    #[cfg(feature = "ClientQueryOptions")]
    # [wasm_bindgen (method , structural , js_class = "Clients" , js_name = matchAll)]
    #[doc = "The `matchAll()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Clients/matchAll)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ClientQueryOptions`, `Clients`*"]
    #[doc = ""]
    #[doc = "Return value: While the Promise can produce any JsValue as far as the type system is concerned, practically it is expected to contain a <code>[::js_sys::Array]</code>. It can be converted like `<code>let result: [::js_sys::Array] = result.await.into();</code>. More information is available in the source IDL file."]
    pub fn match_all_with_options(
        this: &Clients,
        options: &ClientQueryOptions,
    ) -> ::js_sys::Promise;
    # [wasm_bindgen (method , structural , js_class = "Clients" , js_name = openWindow)]
    #[doc = "The `openWindow()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Clients/openWindow)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Clients`*"]
    #[doc = ""]
    #[doc = "Return value: While the Promise can produce any JsValue as far as the type system is concerned, practically it is expected to contain a <code>[Option]<[WindowClient]></code>. It can be converted like `<code>let result: [Option]<[WindowClient]> = result.await.into();</code>."]
    pub fn open_window(this: &Clients, url: &str) -> ::js_sys::Promise;
}
