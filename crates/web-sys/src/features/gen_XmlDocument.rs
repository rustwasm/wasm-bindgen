use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = Document , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = XMLDocument , typescript_name = XMLDocument ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `XmlDocument` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLDocument)
    ///
    ///*This API requires the following crate features to be activated: `XmlDocument`*
    pub type XmlDocument;

    # [ wasm_bindgen ( structural , method , getter , js_class = "XMLDocument" , js_name = async ) ]
    ///Getter for the `async` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLDocument/async)
    ///
    ///*This API requires the following crate features to be activated: `XmlDocument`*
    pub fn r#async(this: &XmlDocument) -> bool;

    # [ wasm_bindgen ( structural , method , setter , js_class = "XMLDocument" , js_name = async ) ]
    ///Setter for the `async` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLDocument/async)
    ///
    ///*This API requires the following crate features to be activated: `XmlDocument`*
    pub fn set_async(this: &XmlDocument, value: bool);

    # [ wasm_bindgen ( catch , method , structural , js_class = "XMLDocument" , js_name = load ) ]
    ///The `load()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLDocument/load)
    ///
    ///*This API requires the following crate features to be activated: `XmlDocument`*
    pub fn load(this: &XmlDocument, url: &str) -> Result<bool, JsValue>;

}
