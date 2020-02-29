use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = XMLSerializer , typescript_name = XMLSerializer ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `XmlSerializer` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLSerializer)
    ///
    ///*This API requires the following crate features to be activated: `XmlSerializer`*
    pub type XmlSerializer;

    #[wasm_bindgen(catch, constructor, js_class = "XMLSerializer")]
    ///The `new XmlSerializer(..)` constructor, creating a new instance of `XmlSerializer`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLSerializer/XMLSerializer)
    ///
    ///*This API requires the following crate features to be activated: `XmlSerializer`*
    pub fn new() -> Result<XmlSerializer, JsValue>;

    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "XMLSerializer" , js_name = serializeToString ) ]
    ///The `serializeToString()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLSerializer/serializeToString)
    ///
    ///*This API requires the following crate features to be activated: `Node`, `XmlSerializer`*
    pub fn serialize_to_string(this: &XmlSerializer, root: &Node) -> Result<String, JsValue>;

}
