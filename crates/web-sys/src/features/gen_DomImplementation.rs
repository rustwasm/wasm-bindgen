use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = DOMImplementation , typescript_name = DOMImplementation ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `DomImplementation` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMImplementation)
    ///
    ///*This API requires the following crate features to be activated: `DomImplementation`*
    pub type DomImplementation;

    #[cfg(feature = "Document")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "DOMImplementation" , js_name = createDocument ) ]
    ///The `createDocument()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMImplementation/createDocument)
    ///
    ///*This API requires the following crate features to be activated: `Document`, `DomImplementation`*
    pub fn create_document(
        this: &DomImplementation,
        namespace: Option<&str>,
        qualified_name: &str,
    ) -> Result<Document, JsValue>;

    #[cfg(all(feature = "Document", feature = "DocumentType",))]
    # [ wasm_bindgen ( catch , method , structural , js_class = "DOMImplementation" , js_name = createDocument ) ]
    ///The `createDocument()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMImplementation/createDocument)
    ///
    ///*This API requires the following crate features to be activated: `Document`, `DocumentType`, `DomImplementation`*
    pub fn create_document_with_doctype(
        this: &DomImplementation,
        namespace: Option<&str>,
        qualified_name: &str,
        doctype: Option<&DocumentType>,
    ) -> Result<Document, JsValue>;

    #[cfg(feature = "DocumentType")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "DOMImplementation" , js_name = createDocumentType ) ]
    ///The `createDocumentType()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMImplementation/createDocumentType)
    ///
    ///*This API requires the following crate features to be activated: `DocumentType`, `DomImplementation`*
    pub fn create_document_type(
        this: &DomImplementation,
        qualified_name: &str,
        public_id: &str,
        system_id: &str,
    ) -> Result<DocumentType, JsValue>;

    #[cfg(feature = "Document")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "DOMImplementation" , js_name = createHTMLDocument ) ]
    ///The `createHTMLDocument()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMImplementation/createHTMLDocument)
    ///
    ///*This API requires the following crate features to be activated: `Document`, `DomImplementation`*
    pub fn create_html_document(this: &DomImplementation) -> Result<Document, JsValue>;

    #[cfg(feature = "Document")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "DOMImplementation" , js_name = createHTMLDocument ) ]
    ///The `createHTMLDocument()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMImplementation/createHTMLDocument)
    ///
    ///*This API requires the following crate features to be activated: `Document`, `DomImplementation`*
    pub fn create_html_document_with_title(
        this: &DomImplementation,
        title: &str,
    ) -> Result<Document, JsValue>;

    # [ wasm_bindgen ( method , structural , js_class = "DOMImplementation" , js_name = hasFeature ) ]
    ///The `hasFeature()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMImplementation/hasFeature)
    ///
    ///*This API requires the following crate features to be activated: `DomImplementation`*
    pub fn has_feature(this: &DomImplementation) -> bool;

}
