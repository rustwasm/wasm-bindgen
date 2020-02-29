use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = XPathExpression , typescript_name = XPathExpression ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `XPathExpression` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XPathExpression)
    ///
    ///*This API requires the following crate features to be activated: `XPathExpression`*
    pub type XPathExpression;

    #[cfg(all(feature = "Node", feature = "XPathResult",))]
    # [ wasm_bindgen ( catch , method , structural , js_class = "XPathExpression" , js_name = evaluate ) ]
    ///The `evaluate()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XPathExpression/evaluate)
    ///
    ///*This API requires the following crate features to be activated: `Node`, `XPathExpression`, `XPathResult`*
    pub fn evaluate(this: &XPathExpression, context_node: &Node) -> Result<XPathResult, JsValue>;

    #[cfg(all(feature = "Node", feature = "XPathResult",))]
    # [ wasm_bindgen ( catch , method , structural , js_class = "XPathExpression" , js_name = evaluate ) ]
    ///The `evaluate()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XPathExpression/evaluate)
    ///
    ///*This API requires the following crate features to be activated: `Node`, `XPathExpression`, `XPathResult`*
    pub fn evaluate_with_type(
        this: &XPathExpression,
        context_node: &Node,
        type_: u16,
    ) -> Result<XPathResult, JsValue>;

    #[cfg(all(feature = "Node", feature = "XPathResult",))]
    # [ wasm_bindgen ( catch , method , structural , js_class = "XPathExpression" , js_name = evaluate ) ]
    ///The `evaluate()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XPathExpression/evaluate)
    ///
    ///*This API requires the following crate features to be activated: `Node`, `XPathExpression`, `XPathResult`*
    pub fn evaluate_with_type_and_result(
        this: &XPathExpression,
        context_node: &Node,
        type_: u16,
        result: Option<&::js_sys::Object>,
    ) -> Result<XPathResult, JsValue>;

}
