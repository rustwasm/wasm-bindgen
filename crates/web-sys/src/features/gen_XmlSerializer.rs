use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = XMLSerializer , typescript_name = XMLSerializer ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `XmlSerializer` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLSerializer)\n\n*This API requires the following crate features to be activated: `XmlSerializer`*"]
    pub type XmlSerializer;
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new XmlSerializer(..)` constructor, creating a new instance of `XmlSerializer`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLSerializer/XMLSerializer)\n\n*This API requires the following crate features to be activated: `XmlSerializer`*"]
    pub fn new(this: &XmlSerializer) -> Result<XmlSerializer, JsValue>;
    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( catch , method , structural , js_name = serializeToString ) ]
    #[doc = "The `serializeToString()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLSerializer/serializeToString)\n\n*This API requires the following crate features to be activated: `Node`, `XmlSerializer`*"]
    pub fn serialize_to_string(this: &XmlSerializer, root: &Node) -> Result<String, JsValue>;
}
