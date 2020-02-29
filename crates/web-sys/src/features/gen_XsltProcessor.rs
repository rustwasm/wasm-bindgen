use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = XSLTProcessor , typescript_name = XSLTProcessor ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `XsltProcessor` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XSLTProcessor)
    ///
    ///*This API requires the following crate features to be activated: `XsltProcessor`*
    pub type XsltProcessor;

    #[wasm_bindgen(catch, constructor, js_class = "XSLTProcessor")]
    ///The `new XsltProcessor(..)` constructor, creating a new instance of `XsltProcessor`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XSLTProcessor/XSLTProcessor)
    ///
    ///*This API requires the following crate features to be activated: `XsltProcessor`*
    pub fn new() -> Result<XsltProcessor, JsValue>;

    # [ wasm_bindgen ( method , structural , js_class = "XSLTProcessor" , js_name = clearParameters ) ]
    ///The `clearParameters()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XSLTProcessor/clearParameters)
    ///
    ///*This API requires the following crate features to be activated: `XsltProcessor`*
    pub fn clear_parameters(this: &XsltProcessor);

    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "XSLTProcessor" , js_name = importStylesheet ) ]
    ///The `importStylesheet()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XSLTProcessor/importStylesheet)
    ///
    ///*This API requires the following crate features to be activated: `Node`, `XsltProcessor`*
    pub fn import_stylesheet(this: &XsltProcessor, style: &Node) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "XSLTProcessor" , js_name = removeParameter ) ]
    ///The `removeParameter()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XSLTProcessor/removeParameter)
    ///
    ///*This API requires the following crate features to be activated: `XsltProcessor`*
    pub fn remove_parameter(
        this: &XsltProcessor,
        namespace_uri: &str,
        local_name: &str,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( method , structural , js_class = "XSLTProcessor" , js_name = reset ) ]
    ///The `reset()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XSLTProcessor/reset)
    ///
    ///*This API requires the following crate features to be activated: `XsltProcessor`*
    pub fn reset(this: &XsltProcessor);

    # [ wasm_bindgen ( catch , method , structural , js_class = "XSLTProcessor" , js_name = setParameter ) ]
    ///The `setParameter()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XSLTProcessor/setParameter)
    ///
    ///*This API requires the following crate features to be activated: `XsltProcessor`*
    pub fn set_parameter(
        this: &XsltProcessor,
        namespace_uri: &str,
        local_name: &str,
        value: &::wasm_bindgen::JsValue,
    ) -> Result<(), JsValue>;

    #[cfg(all(feature = "Document", feature = "Node",))]
    # [ wasm_bindgen ( catch , method , structural , js_class = "XSLTProcessor" , js_name = transformToDocument ) ]
    ///The `transformToDocument()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XSLTProcessor/transformToDocument)
    ///
    ///*This API requires the following crate features to be activated: `Document`, `Node`, `XsltProcessor`*
    pub fn transform_to_document(this: &XsltProcessor, source: &Node) -> Result<Document, JsValue>;

    #[cfg(all(feature = "Document", feature = "DocumentFragment", feature = "Node",))]
    # [ wasm_bindgen ( catch , method , structural , js_class = "XSLTProcessor" , js_name = transformToFragment ) ]
    ///The `transformToFragment()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XSLTProcessor/transformToFragment)
    ///
    ///*This API requires the following crate features to be activated: `Document`, `DocumentFragment`, `Node`, `XsltProcessor`*
    pub fn transform_to_fragment(
        this: &XsltProcessor,
        source: &Node,
        output: &Document,
    ) -> Result<DocumentFragment, JsValue>;

}
