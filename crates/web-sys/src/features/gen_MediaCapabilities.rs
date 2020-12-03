#![allow(unused_imports)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = MediaCapabilities , typescript_type = "MediaCapabilities")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MediaCapabilities` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaCapabilities)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaCapabilities`*"]
    pub type MediaCapabilities;
    #[cfg(feature = "MediaDecodingConfiguration")]
    # [wasm_bindgen (method , structural , js_class = "MediaCapabilities" , js_name = decodingInfo)]
    #[doc = "The `decodingInfo()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaCapabilities/decodingInfo)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaCapabilities`, `MediaDecodingConfiguration`*"]
    #[doc = ""]
    #[doc = "Return value: While the Promise can produce any JsValue as far as the type system is concerned, practically it is expected to contain a `MediaCapabilitiesInfo`. It can be converted like `let result: MediaCapabilitiesInfo = result.await.into();`."]
    pub fn decoding_info(
        this: &MediaCapabilities,
        configuration: &MediaDecodingConfiguration,
    ) -> ::js_sys::Promise;
    #[cfg(feature = "MediaEncodingConfiguration")]
    # [wasm_bindgen (method , structural , js_class = "MediaCapabilities" , js_name = encodingInfo)]
    #[doc = "The `encodingInfo()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaCapabilities/encodingInfo)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaCapabilities`, `MediaEncodingConfiguration`*"]
    #[doc = ""]
    #[doc = "Return value: While the Promise can produce any JsValue as far as the type system is concerned, practically it is expected to contain a `MediaCapabilitiesInfo`. It can be converted like `let result: MediaCapabilitiesInfo = result.await.into();`."]
    pub fn encoding_info(
        this: &MediaCapabilities,
        configuration: &MediaEncodingConfiguration,
    ) -> ::js_sys::Promise;
}
