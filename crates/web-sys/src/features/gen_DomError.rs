use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = DOMError , typescript_type = "DOMError" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `DomError` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMError)
    ///
    ///*This API requires the following crate features to be activated: `DomError`*
    pub type DomError;

    # [ wasm_bindgen ( structural , method , getter , js_class = "DOMError" , js_name = name ) ]
    ///Getter for the `name` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMError/name)
    ///
    ///*This API requires the following crate features to be activated: `DomError`*
    pub fn name(this: &DomError) -> String;

    # [ wasm_bindgen ( structural , method , getter , js_class = "DOMError" , js_name = message ) ]
    ///Getter for the `message` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMError/message)
    ///
    ///*This API requires the following crate features to be activated: `DomError`*
    pub fn message(this: &DomError) -> String;

    #[wasm_bindgen(catch, constructor, js_class = "DOMError")]
    ///The `new DomError(..)` constructor, creating a new instance of `DomError`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMError/DOMError)
    ///
    ///*This API requires the following crate features to be activated: `DomError`*
    pub fn new(name: &str) -> Result<DomError, JsValue>;

    #[wasm_bindgen(catch, constructor, js_class = "DOMError")]
    ///The `new DomError(..)` constructor, creating a new instance of `DomError`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMError/DOMError)
    ///
    ///*This API requires the following crate features to be activated: `DomError`*
    pub fn new_with_message(name: &str, message: &str) -> Result<DomError, JsValue>;

}
