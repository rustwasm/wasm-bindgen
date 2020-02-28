use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = CSSStyleDeclaration , typescript_name = CSSStyleDeclaration ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `CssStyleDeclaration` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleDeclaration)\n\n*This API requires the following crate features to be activated: `CssStyleDeclaration`*"]
    pub type CssStyleDeclaration;
    # [ wasm_bindgen ( structural , method , getter , js_name = cssText ) ]
    #[doc = "Getter for the `cssText` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleDeclaration/cssText)\n\n*This API requires the following crate features to be activated: `CssStyleDeclaration`*"]
    pub fn css_text(this: &CssStyleDeclaration) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = cssText ) ]
    #[doc = "Setter for the `cssText` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleDeclaration/cssText)\n\n*This API requires the following crate features to be activated: `CssStyleDeclaration`*"]
    pub fn set_css_text(this: &CssStyleDeclaration, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_name = length ) ]
    #[doc = "Getter for the `length` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleDeclaration/length)\n\n*This API requires the following crate features to be activated: `CssStyleDeclaration`*"]
    pub fn length(this: &CssStyleDeclaration) -> u32;
    # [ wasm_bindgen ( structural , method , getter , js_name = parentRule ) ]
    #[cfg(feature = "CssRule")]
    #[doc = "Getter for the `parentRule` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleDeclaration/parentRule)\n\n*This API requires the following crate features to be activated: `CssRule`, `CssStyleDeclaration`*"]
    pub fn parent_rule(this: &CssStyleDeclaration) -> Option<CssRule>;
    # [ wasm_bindgen ( method , structural , js_name = getPropertyPriority ) ]
    #[doc = "The `getPropertyPriority()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleDeclaration/getPropertyPriority)\n\n*This API requires the following crate features to be activated: `CssStyleDeclaration`*"]
    pub fn get_property_priority(this: &CssStyleDeclaration, property: &str) -> String;
    # [ wasm_bindgen ( catch , method , structural , js_name = getPropertyValue ) ]
    #[doc = "The `getPropertyValue()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleDeclaration/getPropertyValue)\n\n*This API requires the following crate features to be activated: `CssStyleDeclaration`*"]
    pub fn get_property_value(
        this: &CssStyleDeclaration,
        property: &str,
    ) -> Result<String, JsValue>;
    # [ wasm_bindgen ( method , structural , js_name = item ) ]
    #[doc = "The `item()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleDeclaration/item)\n\n*This API requires the following crate features to be activated: `CssStyleDeclaration`*"]
    pub fn item(this: &CssStyleDeclaration, index: u32) -> String;
    # [ wasm_bindgen ( catch , method , structural , js_name = removeProperty ) ]
    #[doc = "The `removeProperty()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleDeclaration/removeProperty)\n\n*This API requires the following crate features to be activated: `CssStyleDeclaration`*"]
    pub fn remove_property(this: &CssStyleDeclaration, property: &str) -> Result<String, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = setProperty ) ]
    #[doc = "The `setProperty()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleDeclaration/setProperty)\n\n*This API requires the following crate features to be activated: `CssStyleDeclaration`*"]
    pub fn set_property(
        this: &CssStyleDeclaration,
        property: &str,
        value: &str,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = setProperty ) ]
    #[doc = "The `setProperty()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleDeclaration/setProperty)\n\n*This API requires the following crate features to be activated: `CssStyleDeclaration`*"]
    pub fn set_property_with_priority(
        this: &CssStyleDeclaration,
        property: &str,
        value: &str,
        priority: &str,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(method, structural, indexing_getter)]
    #[doc = "Indexing getter.\n\n\n\n*This API requires the following crate features to be activated: `CssStyleDeclaration`*"]
    pub fn get(this: &CssStyleDeclaration, index: u32) -> Option<String>;
}
