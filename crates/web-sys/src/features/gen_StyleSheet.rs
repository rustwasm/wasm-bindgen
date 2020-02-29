use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = StyleSheet , typescript_type = "StyleSheet" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `StyleSheet` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StyleSheet)
    ///
    ///*This API requires the following crate features to be activated: `StyleSheet`*
    pub type StyleSheet;

    # [ wasm_bindgen ( structural , method , getter , js_class = "StyleSheet" , js_name = type ) ]
    ///Getter for the `type` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StyleSheet/type)
    ///
    ///*This API requires the following crate features to be activated: `StyleSheet`*
    pub fn type_(this: &StyleSheet) -> String;

    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "StyleSheet" , js_name = href ) ]
    ///Getter for the `href` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StyleSheet/href)
    ///
    ///*This API requires the following crate features to be activated: `StyleSheet`*
    pub fn href(this: &StyleSheet) -> Result<Option<String>, JsValue>;

    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "StyleSheet" , js_name = ownerNode ) ]
    ///Getter for the `ownerNode` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StyleSheet/ownerNode)
    ///
    ///*This API requires the following crate features to be activated: `Node`, `StyleSheet`*
    pub fn owner_node(this: &StyleSheet) -> Option<Node>;

    # [ wasm_bindgen ( structural , method , getter , js_class = "StyleSheet" , js_name = parentStyleSheet ) ]
    ///Getter for the `parentStyleSheet` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StyleSheet/parentStyleSheet)
    ///
    ///*This API requires the following crate features to be activated: `StyleSheet`*
    pub fn parent_style_sheet(this: &StyleSheet) -> Option<StyleSheet>;

    # [ wasm_bindgen ( structural , method , getter , js_class = "StyleSheet" , js_name = title ) ]
    ///Getter for the `title` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StyleSheet/title)
    ///
    ///*This API requires the following crate features to be activated: `StyleSheet`*
    pub fn title(this: &StyleSheet) -> Option<String>;

    #[cfg(feature = "MediaList")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "StyleSheet" , js_name = media ) ]
    ///Getter for the `media` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StyleSheet/media)
    ///
    ///*This API requires the following crate features to be activated: `MediaList`, `StyleSheet`*
    pub fn media(this: &StyleSheet) -> MediaList;

    # [ wasm_bindgen ( structural , method , getter , js_class = "StyleSheet" , js_name = disabled ) ]
    ///Getter for the `disabled` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StyleSheet/disabled)
    ///
    ///*This API requires the following crate features to be activated: `StyleSheet`*
    pub fn disabled(this: &StyleSheet) -> bool;

    # [ wasm_bindgen ( structural , method , setter , js_class = "StyleSheet" , js_name = disabled ) ]
    ///Setter for the `disabled` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StyleSheet/disabled)
    ///
    ///*This API requires the following crate features to be activated: `StyleSheet`*
    pub fn set_disabled(this: &StyleSheet, value: bool);

}
