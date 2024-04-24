#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = WebTransportErrorOptions)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `WebTransportErrorOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportErrorOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type WebTransportErrorOptions;
    #[cfg(feature = "WebTransportErrorSource")]
    #[wasm_bindgen(method, getter = "source")]
    fn source_shim(this: &WebTransportErrorOptions) -> WebTransportErrorSource;
    #[cfg(feature = "WebTransportErrorSource")]
    #[wasm_bindgen(method, setter = "source")]
    fn set_source_shim(this: &WebTransportErrorOptions, val: WebTransportErrorSource);
    #[wasm_bindgen(method, getter = "streamErrorCode")]
    fn stream_error_code_shim(this: &WebTransportErrorOptions) -> Option<u8>;
    #[wasm_bindgen(method, setter = "streamErrorCode")]
    fn set_stream_error_code_shim(this: &WebTransportErrorOptions, val: Option<u8>);
}
#[cfg(web_sys_unstable_apis)]
#[doc = "The trait to access properties on the `WebTransportErrorOptions` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `WebTransportErrorOptions`*"]
pub trait WebTransportErrorOptionsGetters {
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "WebTransportErrorSource")]
    #[doc = "Get the `source` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportErrorOptions`, `WebTransportErrorSource`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn source(&self) -> WebTransportErrorSource;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `streamErrorCode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportErrorOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn stream_error_code(&self) -> Option<u8>;
}
#[cfg(web_sys_unstable_apis)]
impl WebTransportErrorOptionsGetters for WebTransportErrorOptions {
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "WebTransportErrorSource")]
    fn source(&self) -> WebTransportErrorSource {
        self.source_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn stream_error_code(&self) -> Option<u8> {
        self.stream_error_code_shim()
    }
}
#[cfg(web_sys_unstable_apis)]
impl WebTransportErrorOptions {
    #[doc = "Construct a new `WebTransportErrorOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportErrorOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "WebTransportErrorSource")]
    #[doc = "Change the `source` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportErrorOptions`, `WebTransportErrorSource`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn source(&mut self, val: WebTransportErrorSource) -> &mut Self {
        self.set_source_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `streamErrorCode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportErrorOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn stream_error_code(&mut self, val: Option<u8>) -> &mut Self {
        self.set_stream_error_code_shim(val);
        self
    }
}
#[cfg(web_sys_unstable_apis)]
impl Default for WebTransportErrorOptions {
    fn default() -> Self {
        Self::new()
    }
}
