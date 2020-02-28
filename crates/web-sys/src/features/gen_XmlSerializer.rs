use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = XMLSerializer , typescript_name = XMLSerializer ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `XmlSerializer` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLSerializer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XmlSerializer`*"]
    pub type XmlSerializer;
    #[wasm_bindgen(catch, js_class = "XMLSerializer", constructor)]
    #[doc = "The `new XmlSerializer(..)` constructor, creating a new instance of `XmlSerializer`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLSerializer/XMLSerializer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XmlSerializer`*"]
    pub fn new(this: &XmlSerializer) -> Result<XmlSerializer, JsValue>;
    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "XMLSerializer" , js_name = serializeToString ) ]
    #[doc = "The `serializeToString()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLSerializer/serializeToString)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`, `XmlSerializer`*"]
    pub fn serialize_to_string(this: &XmlSerializer, root: &Node) -> Result<String, JsValue>;
}
