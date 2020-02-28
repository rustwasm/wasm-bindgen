use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = DOMError , typescript_name = DOMError ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DomError` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMError)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomError`*"]
    pub type DomError;
    # [ wasm_bindgen ( structural , method , getter , js_class = "DOMError" , js_name = name ) ]
    #[doc = "Getter for the `name` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMError/name)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomError`*"]
    pub fn name(this: &DomError) -> String;
    # [ wasm_bindgen ( structural , method , getter , js_class = "DOMError" , js_name = message ) ]
    #[doc = "Getter for the `message` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMError/message)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomError`*"]
    pub fn message(this: &DomError) -> String;
    #[wasm_bindgen(catch, js_class = "DOMError", constructor)]
    #[doc = "The `new DomError(..)` constructor, creating a new instance of `DomError`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMError/DOMError)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomError`*"]
    pub fn new(this: &DomError, name: &str) -> Result<DomError, JsValue>;
    #[wasm_bindgen(catch, js_class = "DOMError", constructor)]
    #[doc = "The `new DomError(..)` constructor, creating a new instance of `DomError`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMError/DOMError)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomError`*"]
    pub fn new_with_message(
        this: &DomError,
        name: &str,
        message: &str,
    ) -> Result<DomError, JsValue>;
}
