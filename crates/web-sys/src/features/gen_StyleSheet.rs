use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = StyleSheet , typescript_name = StyleSheet ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `StyleSheet` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StyleSheet)\n\n*This API requires the following crate features to be activated: `StyleSheet`*"]
    pub type StyleSheet;
    # [ wasm_bindgen ( structural , method , getter , js_name = type ) ]
    #[doc = "Getter for the `type` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StyleSheet/type)\n\n*This API requires the following crate features to be activated: `StyleSheet`*"]
    pub fn type_(this: &StyleSheet) -> String;
    # [ wasm_bindgen ( structural , catch , method , getter , js_name = href ) ]
    #[doc = "Getter for the `href` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StyleSheet/href)\n\n*This API requires the following crate features to be activated: `StyleSheet`*"]
    pub fn href(this: &StyleSheet) -> Result<Option<String>, JsValue>;
    # [ wasm_bindgen ( structural , method , getter , js_name = ownerNode ) ]
    #[cfg(feature = "Node")]
    #[doc = "Getter for the `ownerNode` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StyleSheet/ownerNode)\n\n*This API requires the following crate features to be activated: `Node`, `StyleSheet`*"]
    pub fn owner_node(this: &StyleSheet) -> Option<Node>;
    # [ wasm_bindgen ( structural , method , getter , js_name = parentStyleSheet ) ]
    #[doc = "Getter for the `parentStyleSheet` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StyleSheet/parentStyleSheet)\n\n*This API requires the following crate features to be activated: `StyleSheet`*"]
    pub fn parent_style_sheet(this: &StyleSheet) -> Option<StyleSheet>;
    # [ wasm_bindgen ( structural , method , getter , js_name = title ) ]
    #[doc = "Getter for the `title` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StyleSheet/title)\n\n*This API requires the following crate features to be activated: `StyleSheet`*"]
    pub fn title(this: &StyleSheet) -> Option<String>;
    # [ wasm_bindgen ( structural , method , getter , js_name = media ) ]
    #[cfg(feature = "MediaList")]
    #[doc = "Getter for the `media` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StyleSheet/media)\n\n*This API requires the following crate features to be activated: `MediaList`, `StyleSheet`*"]
    pub fn media(this: &StyleSheet) -> MediaList;
    # [ wasm_bindgen ( structural , method , getter , js_name = disabled ) ]
    #[doc = "Getter for the `disabled` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StyleSheet/disabled)\n\n*This API requires the following crate features to be activated: `StyleSheet`*"]
    pub fn disabled(this: &StyleSheet) -> bool;
    # [ wasm_bindgen ( structural , method , setter , js_name = disabled ) ]
    #[doc = "Setter for the `disabled` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StyleSheet/disabled)\n\n*This API requires the following crate features to be activated: `StyleSheet`*"]
    pub fn set_disabled(this: &StyleSheet, value: bool);
}
