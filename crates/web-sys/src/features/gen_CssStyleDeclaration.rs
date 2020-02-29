use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = CSSStyleDeclaration , typescript_type = "CSSStyleDeclaration" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `CssStyleDeclaration` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleDeclaration)
    ///
    ///*This API requires the following crate features to be activated: `CssStyleDeclaration`*
    pub type CssStyleDeclaration;

    # [ wasm_bindgen ( structural , method , getter , js_class = "CSSStyleDeclaration" , js_name = cssText ) ]
    ///Getter for the `cssText` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleDeclaration/cssText)
    ///
    ///*This API requires the following crate features to be activated: `CssStyleDeclaration`*
    pub fn css_text(this: &CssStyleDeclaration) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "CSSStyleDeclaration" , js_name = cssText ) ]
    ///Setter for the `cssText` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleDeclaration/cssText)
    ///
    ///*This API requires the following crate features to be activated: `CssStyleDeclaration`*
    pub fn set_css_text(this: &CssStyleDeclaration, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "CSSStyleDeclaration" , js_name = length ) ]
    ///Getter for the `length` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleDeclaration/length)
    ///
    ///*This API requires the following crate features to be activated: `CssStyleDeclaration`*
    pub fn length(this: &CssStyleDeclaration) -> u32;

    #[cfg(feature = "CssRule")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "CSSStyleDeclaration" , js_name = parentRule ) ]
    ///Getter for the `parentRule` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleDeclaration/parentRule)
    ///
    ///*This API requires the following crate features to be activated: `CssRule`, `CssStyleDeclaration`*
    pub fn parent_rule(this: &CssStyleDeclaration) -> Option<CssRule>;

    # [ wasm_bindgen ( method , structural , js_class = "CSSStyleDeclaration" , js_name = getPropertyPriority ) ]
    ///The `getPropertyPriority()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleDeclaration/getPropertyPriority)
    ///
    ///*This API requires the following crate features to be activated: `CssStyleDeclaration`*
    pub fn get_property_priority(this: &CssStyleDeclaration, property: &str) -> String;

    # [ wasm_bindgen ( catch , method , structural , js_class = "CSSStyleDeclaration" , js_name = getPropertyValue ) ]
    ///The `getPropertyValue()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleDeclaration/getPropertyValue)
    ///
    ///*This API requires the following crate features to be activated: `CssStyleDeclaration`*
    pub fn get_property_value(
        this: &CssStyleDeclaration,
        property: &str,
    ) -> Result<String, JsValue>;

    # [ wasm_bindgen ( method , structural , js_class = "CSSStyleDeclaration" , js_name = item ) ]
    ///The `item()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleDeclaration/item)
    ///
    ///*This API requires the following crate features to be activated: `CssStyleDeclaration`*
    pub fn item(this: &CssStyleDeclaration, index: u32) -> String;

    # [ wasm_bindgen ( catch , method , structural , js_class = "CSSStyleDeclaration" , js_name = removeProperty ) ]
    ///The `removeProperty()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleDeclaration/removeProperty)
    ///
    ///*This API requires the following crate features to be activated: `CssStyleDeclaration`*
    pub fn remove_property(this: &CssStyleDeclaration, property: &str) -> Result<String, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "CSSStyleDeclaration" , js_name = setProperty ) ]
    ///The `setProperty()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleDeclaration/setProperty)
    ///
    ///*This API requires the following crate features to be activated: `CssStyleDeclaration`*
    pub fn set_property(
        this: &CssStyleDeclaration,
        property: &str,
        value: &str,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "CSSStyleDeclaration" , js_name = setProperty ) ]
    ///The `setProperty()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleDeclaration/setProperty)
    ///
    ///*This API requires the following crate features to be activated: `CssStyleDeclaration`*
    pub fn set_property_with_priority(
        this: &CssStyleDeclaration,
        property: &str,
        value: &str,
        priority: &str,
    ) -> Result<(), JsValue>;

    #[wasm_bindgen(method, structural, js_class = "CSSStyleDeclaration", indexing_getter)]
    ///Indexing getter.
    ///
    ///
    ///
    ///*This API requires the following crate features to be activated: `CssStyleDeclaration`*
    pub fn get(this: &CssStyleDeclaration, index: u32) -> Option<String>;

}
