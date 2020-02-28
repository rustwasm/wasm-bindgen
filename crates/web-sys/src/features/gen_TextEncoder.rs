use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = TextEncoder , typescript_name = TextEncoder ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `TextEncoder` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextEncoder)\n\n*This API requires the following crate features to be activated: `TextEncoder`*"]
    pub type TextEncoder;
    # [ wasm_bindgen ( structural , method , getter , js_name = encoding ) ]
    #[doc = "Getter for the `encoding` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextEncoder/encoding)\n\n*This API requires the following crate features to be activated: `TextEncoder`*"]
    pub fn encoding(this: &TextEncoder) -> String;
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new TextEncoder(..)` constructor, creating a new instance of `TextEncoder`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextEncoder/TextEncoder)\n\n*This API requires the following crate features to be activated: `TextEncoder`*"]
    pub fn new(this: &TextEncoder) -> Result<TextEncoder, JsValue>;
    # [ wasm_bindgen ( method , structural , js_name = encode ) ]
    #[doc = "The `encode()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextEncoder/encode)\n\n*This API requires the following crate features to be activated: `TextEncoder`*"]
    pub fn encode(this: &TextEncoder) -> Vec<u8>;
    # [ wasm_bindgen ( method , structural , js_name = encode ) ]
    #[doc = "The `encode()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextEncoder/encode)\n\n*This API requires the following crate features to be activated: `TextEncoder`*"]
    pub fn encode_with_input(this: &TextEncoder, input: &str) -> Vec<u8>;
}
