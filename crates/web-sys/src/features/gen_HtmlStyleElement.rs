use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = HtmlElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = HTMLStyleElement , typescript_name = HTMLStyleElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlStyleElement` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLStyleElement)\n\n*This API requires the following crate features to be activated: `HtmlStyleElement`*"]
    pub type HtmlStyleElement;
    # [ wasm_bindgen ( structural , method , getter , js_name = disabled ) ]
    #[doc = "Getter for the `disabled` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLStyleElement/disabled)\n\n*This API requires the following crate features to be activated: `HtmlStyleElement`*"]
    pub fn disabled(this: &HtmlStyleElement) -> bool;
    # [ wasm_bindgen ( structural , method , setter , js_name = disabled ) ]
    #[doc = "Setter for the `disabled` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLStyleElement/disabled)\n\n*This API requires the following crate features to be activated: `HtmlStyleElement`*"]
    pub fn set_disabled(this: &HtmlStyleElement, value: bool);
    # [ wasm_bindgen ( structural , method , getter , js_name = media ) ]
    #[doc = "Getter for the `media` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLStyleElement/media)\n\n*This API requires the following crate features to be activated: `HtmlStyleElement`*"]
    pub fn media(this: &HtmlStyleElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = media ) ]
    #[doc = "Setter for the `media` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLStyleElement/media)\n\n*This API requires the following crate features to be activated: `HtmlStyleElement`*"]
    pub fn set_media(this: &HtmlStyleElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_name = type ) ]
    #[doc = "Getter for the `type` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLStyleElement/type)\n\n*This API requires the following crate features to be activated: `HtmlStyleElement`*"]
    pub fn type_(this: &HtmlStyleElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = type ) ]
    #[doc = "Setter for the `type` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLStyleElement/type)\n\n*This API requires the following crate features to be activated: `HtmlStyleElement`*"]
    pub fn set_type(this: &HtmlStyleElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_name = sheet ) ]
    #[cfg(feature = "StyleSheet")]
    #[doc = "Getter for the `sheet` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLStyleElement/sheet)\n\n*This API requires the following crate features to be activated: `HtmlStyleElement`, `StyleSheet`*"]
    pub fn sheet(this: &HtmlStyleElement) -> Option<StyleSheet>;
}
