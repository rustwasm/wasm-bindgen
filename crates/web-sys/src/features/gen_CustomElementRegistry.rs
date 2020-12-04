#![allow(unused_imports)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = CustomElementRegistry , typescript_type = "CustomElementRegistry")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `CustomElementRegistry` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CustomElementRegistry)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CustomElementRegistry`*"]
    pub type CustomElementRegistry;
    # [wasm_bindgen (catch , method , structural , js_class = "CustomElementRegistry" , js_name = define)]
    #[doc = "The `define()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CustomElementRegistry/define)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CustomElementRegistry`*"]
    #[doc = ""]
    #[doc = "Argument `function_constructor`: See the referenced MDN documentation or the IDL files for the signature of the callback."]
    pub fn define(
        this: &CustomElementRegistry,
        name: &str,
        function_constructor: &::js_sys::Function,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "ElementDefinitionOptions")]
    # [wasm_bindgen (catch , method , structural , js_class = "CustomElementRegistry" , js_name = define)]
    #[doc = "The `define()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CustomElementRegistry/define)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CustomElementRegistry`, `ElementDefinitionOptions`*"]
    #[doc = ""]
    #[doc = "Argument `function_constructor`: See the referenced MDN documentation or the IDL files for the signature of the callback."]
    pub fn define_with_options(
        this: &CustomElementRegistry,
        name: &str,
        function_constructor: &::js_sys::Function,
        options: &ElementDefinitionOptions,
    ) -> Result<(), JsValue>;
    # [wasm_bindgen (method , structural , js_class = "CustomElementRegistry" , js_name = get)]
    #[doc = "The `get()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CustomElementRegistry/get)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CustomElementRegistry`*"]
    pub fn get(this: &CustomElementRegistry, name: &str) -> ::wasm_bindgen::JsValue;
    #[cfg(feature = "Node")]
    # [wasm_bindgen (method , structural , js_class = "CustomElementRegistry" , js_name = upgrade)]
    #[doc = "The `upgrade()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CustomElementRegistry/upgrade)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CustomElementRegistry`, `Node`*"]
    pub fn upgrade(this: &CustomElementRegistry, root: &Node);
    # [wasm_bindgen (catch , method , structural , js_class = "CustomElementRegistry" , js_name = whenDefined)]
    #[doc = "The `whenDefined()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CustomElementRegistry/whenDefined)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CustomElementRegistry`*"]
    #[doc = ""]
    #[doc = "Return value: While the Promise can produce any JsValue as far as the type system is concerned, practically it is just used to indicate completion."]
    pub fn when_defined(
        this: &CustomElementRegistry,
        name: &str,
    ) -> Result<::js_sys::Promise, JsValue>;
}
