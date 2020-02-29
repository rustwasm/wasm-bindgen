use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = TextEncoder , typescript_name = TextEncoder ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `TextEncoder` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextEncoder)
    ///
    ///*This API requires the following crate features to be activated: `TextEncoder`*
    pub type TextEncoder;

    # [ wasm_bindgen ( structural , method , getter , js_class = "TextEncoder" , js_name = encoding ) ]
    ///Getter for the `encoding` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextEncoder/encoding)
    ///
    ///*This API requires the following crate features to be activated: `TextEncoder`*
    pub fn encoding(this: &TextEncoder) -> String;

    #[wasm_bindgen(catch, constructor, js_class = "TextEncoder")]
    ///The `new TextEncoder(..)` constructor, creating a new instance of `TextEncoder`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextEncoder/TextEncoder)
    ///
    ///*This API requires the following crate features to be activated: `TextEncoder`*
    pub fn new() -> Result<TextEncoder, JsValue>;

    # [ wasm_bindgen ( method , structural , js_class = "TextEncoder" , js_name = encode ) ]
    ///The `encode()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextEncoder/encode)
    ///
    ///*This API requires the following crate features to be activated: `TextEncoder`*
    pub fn encode(this: &TextEncoder) -> Vec<u8>;

    # [ wasm_bindgen ( method , structural , js_class = "TextEncoder" , js_name = encode ) ]
    ///The `encode()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextEncoder/encode)
    ///
    ///*This API requires the following crate features to be activated: `TextEncoder`*
    pub fn encode_with_input(this: &TextEncoder, input: &str) -> Vec<u8>;

}
