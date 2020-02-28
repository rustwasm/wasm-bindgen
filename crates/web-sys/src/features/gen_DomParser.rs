use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = DOMParser , typescript_name = DOMParser ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DomParser` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMParser)\n\n*This API requires the following crate features to be activated: `DomParser`*"]
    pub type DomParser;
    #[wasm_bindgen(catch, js_class = "DOMParser", constructor)]
    #[doc = "The `new DomParser(..)` constructor, creating a new instance of `DomParser`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMParser/DOMParser)\n\n*This API requires the following crate features to be activated: `DomParser`*"]
    pub fn new(this: &DomParser) -> Result<DomParser, JsValue>;
    #[cfg(all(feature = "Document", feature = "SupportedType",))]
    # [ wasm_bindgen ( catch , method , structural , js_class = "DOMParser" , js_name = parseFromString ) ]
    #[doc = "The `parseFromString()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMParser/parseFromString)\n\n*This API requires the following crate features to be activated: `Document`, `DomParser`, `SupportedType`*"]
    pub fn parse_from_string(
        this: &DomParser,
        str: &str,
        type_: SupportedType,
    ) -> Result<Document, JsValue>;
}
