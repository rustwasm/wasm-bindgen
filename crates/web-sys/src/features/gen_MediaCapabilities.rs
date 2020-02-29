use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = MediaCapabilities , typescript_name = MediaCapabilities ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `MediaCapabilities` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaCapabilities)
    ///
    ///*This API requires the following crate features to be activated: `MediaCapabilities`*
    pub type MediaCapabilities;

    #[cfg(feature = "MediaDecodingConfiguration")]
    # [ wasm_bindgen ( method , structural , js_class = "MediaCapabilities" , js_name = decodingInfo ) ]
    ///The `decodingInfo()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaCapabilities/decodingInfo)
    ///
    ///*This API requires the following crate features to be activated: `MediaCapabilities`, `MediaDecodingConfiguration`*
    pub fn decoding_info(
        this: &MediaCapabilities,
        configuration: &MediaDecodingConfiguration,
    ) -> ::js_sys::Promise;

    #[cfg(feature = "MediaEncodingConfiguration")]
    # [ wasm_bindgen ( method , structural , js_class = "MediaCapabilities" , js_name = encodingInfo ) ]
    ///The `encodingInfo()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaCapabilities/encodingInfo)
    ///
    ///*This API requires the following crate features to be activated: `MediaCapabilities`, `MediaEncodingConfiguration`*
    pub fn encoding_info(
        this: &MediaCapabilities,
        configuration: &MediaEncodingConfiguration,
    ) -> ::js_sys::Promise;

}
