use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = TextDecoder , typescript_name = TextDecoder ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `TextDecoder` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextDecoder)
    ///
    ///*This API requires the following crate features to be activated: `TextDecoder`*
    pub type TextDecoder;

    # [ wasm_bindgen ( structural , method , getter , js_class = "TextDecoder" , js_name = encoding ) ]
    ///Getter for the `encoding` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextDecoder/encoding)
    ///
    ///*This API requires the following crate features to be activated: `TextDecoder`*
    pub fn encoding(this: &TextDecoder) -> String;

    # [ wasm_bindgen ( structural , method , getter , js_class = "TextDecoder" , js_name = fatal ) ]
    ///Getter for the `fatal` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextDecoder/fatal)
    ///
    ///*This API requires the following crate features to be activated: `TextDecoder`*
    pub fn fatal(this: &TextDecoder) -> bool;

    #[wasm_bindgen(catch, constructor, js_class = "TextDecoder")]
    ///The `new TextDecoder(..)` constructor, creating a new instance of `TextDecoder`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextDecoder/TextDecoder)
    ///
    ///*This API requires the following crate features to be activated: `TextDecoder`*
    pub fn new() -> Result<TextDecoder, JsValue>;

    #[wasm_bindgen(catch, constructor, js_class = "TextDecoder")]
    ///The `new TextDecoder(..)` constructor, creating a new instance of `TextDecoder`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextDecoder/TextDecoder)
    ///
    ///*This API requires the following crate features to be activated: `TextDecoder`*
    pub fn new_with_label(label: &str) -> Result<TextDecoder, JsValue>;

    #[cfg(feature = "TextDecoderOptions")]
    #[wasm_bindgen(catch, constructor, js_class = "TextDecoder")]
    ///The `new TextDecoder(..)` constructor, creating a new instance of `TextDecoder`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextDecoder/TextDecoder)
    ///
    ///*This API requires the following crate features to be activated: `TextDecoder`, `TextDecoderOptions`*
    pub fn new_with_label_and_options(
        label: &str,
        options: &TextDecoderOptions,
    ) -> Result<TextDecoder, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "TextDecoder" , js_name = decode ) ]
    ///The `decode()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextDecoder/decode)
    ///
    ///*This API requires the following crate features to be activated: `TextDecoder`*
    pub fn decode(this: &TextDecoder) -> Result<String, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "TextDecoder" , js_name = decode ) ]
    ///The `decode()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextDecoder/decode)
    ///
    ///*This API requires the following crate features to be activated: `TextDecoder`*
    pub fn decode_with_buffer_source(
        this: &TextDecoder,
        input: &::js_sys::Object,
    ) -> Result<String, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "TextDecoder" , js_name = decode ) ]
    ///The `decode()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextDecoder/decode)
    ///
    ///*This API requires the following crate features to be activated: `TextDecoder`*
    pub fn decode_with_u8_array(this: &TextDecoder, input: &mut [u8]) -> Result<String, JsValue>;

    #[cfg(feature = "TextDecodeOptions")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "TextDecoder" , js_name = decode ) ]
    ///The `decode()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextDecoder/decode)
    ///
    ///*This API requires the following crate features to be activated: `TextDecodeOptions`, `TextDecoder`*
    pub fn decode_with_buffer_source_and_options(
        this: &TextDecoder,
        input: &::js_sys::Object,
        options: &TextDecodeOptions,
    ) -> Result<String, JsValue>;

    #[cfg(feature = "TextDecodeOptions")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "TextDecoder" , js_name = decode ) ]
    ///The `decode()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextDecoder/decode)
    ///
    ///*This API requires the following crate features to be activated: `TextDecodeOptions`, `TextDecoder`*
    pub fn decode_with_u8_array_and_options(
        this: &TextDecoder,
        input: &mut [u8],
        options: &TextDecodeOptions,
    ) -> Result<String, JsValue>;

}
