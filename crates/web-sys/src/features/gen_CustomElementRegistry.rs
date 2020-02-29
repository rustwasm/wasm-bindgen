use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = CustomElementRegistry , typescript_name = CustomElementRegistry ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `CustomElementRegistry` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CustomElementRegistry)
    ///
    ///*This API requires the following crate features to be activated: `CustomElementRegistry`*
    pub type CustomElementRegistry;

    # [ wasm_bindgen ( catch , method , structural , js_class = "CustomElementRegistry" , js_name = define ) ]
    ///The `define()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CustomElementRegistry/define)
    ///
    ///*This API requires the following crate features to be activated: `CustomElementRegistry`*
    pub fn define(
        this: &CustomElementRegistry,
        name: &str,
        function_constructor: &::js_sys::Function,
    ) -> Result<(), JsValue>;

    #[cfg(feature = "ElementDefinitionOptions")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "CustomElementRegistry" , js_name = define ) ]
    ///The `define()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CustomElementRegistry/define)
    ///
    ///*This API requires the following crate features to be activated: `CustomElementRegistry`, `ElementDefinitionOptions`*
    pub fn define_with_options(
        this: &CustomElementRegistry,
        name: &str,
        function_constructor: &::js_sys::Function,
        options: &ElementDefinitionOptions,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( method , structural , js_class = "CustomElementRegistry" , js_name = get ) ]
    ///The `get()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CustomElementRegistry/get)
    ///
    ///*This API requires the following crate features to be activated: `CustomElementRegistry`*
    pub fn get(this: &CustomElementRegistry, name: &str) -> ::wasm_bindgen::JsValue;

    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( method , structural , js_class = "CustomElementRegistry" , js_name = upgrade ) ]
    ///The `upgrade()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CustomElementRegistry/upgrade)
    ///
    ///*This API requires the following crate features to be activated: `CustomElementRegistry`, `Node`*
    pub fn upgrade(this: &CustomElementRegistry, root: &Node);

    # [ wasm_bindgen ( catch , method , structural , js_class = "CustomElementRegistry" , js_name = whenDefined ) ]
    ///The `whenDefined()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CustomElementRegistry/whenDefined)
    ///
    ///*This API requires the following crate features to be activated: `CustomElementRegistry`*
    pub fn when_defined(
        this: &CustomElementRegistry,
        name: &str,
    ) -> Result<::js_sys::Promise, JsValue>;

}
