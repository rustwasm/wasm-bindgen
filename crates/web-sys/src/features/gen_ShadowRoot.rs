use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = DocumentFragment , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = ShadowRoot , typescript_name = ShadowRoot ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ShadowRoot` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ShadowRoot)\n\n*This API requires the following crate features to be activated: `ShadowRoot`*"]
    pub type ShadowRoot;
    # [ wasm_bindgen ( structural , method , getter , js_class = "ShadowRoot" , js_name = mode ) ]
    #[cfg(feature = "ShadowRootMode")]
    #[doc = "Getter for the `mode` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ShadowRoot/mode)\n\n*This API requires the following crate features to be activated: `ShadowRoot`, `ShadowRootMode`*"]
    pub fn mode(this: &ShadowRoot) -> ShadowRootMode;
    # [ wasm_bindgen ( structural , method , getter , js_class = "ShadowRoot" , js_name = host ) ]
    #[cfg(feature = "Element")]
    #[doc = "Getter for the `host` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ShadowRoot/host)\n\n*This API requires the following crate features to be activated: `Element`, `ShadowRoot`*"]
    pub fn host(this: &ShadowRoot) -> Element;
    # [ wasm_bindgen ( structural , method , getter , js_class = "ShadowRoot" , js_name = innerHTML ) ]
    #[doc = "Getter for the `innerHTML` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ShadowRoot/innerHTML)\n\n*This API requires the following crate features to be activated: `ShadowRoot`*"]
    pub fn inner_html(this: &ShadowRoot) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_class = "ShadowRoot" , js_name = innerHTML ) ]
    #[doc = "Setter for the `innerHTML` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ShadowRoot/innerHTML)\n\n*This API requires the following crate features to be activated: `ShadowRoot`*"]
    pub fn set_inner_html(this: &ShadowRoot, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_class = "ShadowRoot" , js_name = activeElement ) ]
    #[cfg(feature = "Element")]
    #[doc = "Getter for the `activeElement` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ShadowRoot/activeElement)\n\n*This API requires the following crate features to be activated: `Element`, `ShadowRoot`*"]
    pub fn active_element(this: &ShadowRoot) -> Option<Element>;
    # [ wasm_bindgen ( structural , method , getter , js_class = "ShadowRoot" , js_name = styleSheets ) ]
    #[cfg(feature = "StyleSheetList")]
    #[doc = "Getter for the `styleSheets` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ShadowRoot/styleSheets)\n\n*This API requires the following crate features to be activated: `ShadowRoot`, `StyleSheetList`*"]
    pub fn style_sheets(this: &ShadowRoot) -> StyleSheetList;
    # [ wasm_bindgen ( structural , method , getter , js_class = "ShadowRoot" , js_name = pointerLockElement ) ]
    #[cfg(feature = "Element")]
    #[doc = "Getter for the `pointerLockElement` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ShadowRoot/pointerLockElement)\n\n*This API requires the following crate features to be activated: `Element`, `ShadowRoot`*"]
    pub fn pointer_lock_element(this: &ShadowRoot) -> Option<Element>;
    # [ wasm_bindgen ( structural , method , getter , js_class = "ShadowRoot" , js_name = fullscreenElement ) ]
    #[cfg(feature = "Element")]
    #[doc = "Getter for the `fullscreenElement` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ShadowRoot/fullscreenElement)\n\n*This API requires the following crate features to be activated: `Element`, `ShadowRoot`*"]
    pub fn fullscreen_element(this: &ShadowRoot) -> Option<Element>;
    #[cfg(feature = "Element")]
    # [ wasm_bindgen ( method , structural , js_class = "ShadowRoot" , js_name = getElementById ) ]
    #[doc = "The `getElementById()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ShadowRoot/getElementById)\n\n*This API requires the following crate features to be activated: `Element`, `ShadowRoot`*"]
    pub fn get_element_by_id(this: &ShadowRoot, element_id: &str) -> Option<Element>;
    #[cfg(feature = "HtmlCollection")]
    # [ wasm_bindgen ( method , structural , js_class = "ShadowRoot" , js_name = getElementsByClassName ) ]
    #[doc = "The `getElementsByClassName()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ShadowRoot/getElementsByClassName)\n\n*This API requires the following crate features to be activated: `HtmlCollection`, `ShadowRoot`*"]
    pub fn get_elements_by_class_name(this: &ShadowRoot, class_names: &str) -> HtmlCollection;
    #[cfg(feature = "HtmlCollection")]
    # [ wasm_bindgen ( method , structural , js_class = "ShadowRoot" , js_name = getElementsByTagName ) ]
    #[doc = "The `getElementsByTagName()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ShadowRoot/getElementsByTagName)\n\n*This API requires the following crate features to be activated: `HtmlCollection`, `ShadowRoot`*"]
    pub fn get_elements_by_tag_name(this: &ShadowRoot, local_name: &str) -> HtmlCollection;
    #[cfg(feature = "HtmlCollection")]
    # [ wasm_bindgen ( method , structural , js_class = "ShadowRoot" , js_name = getElementsByTagNameNS ) ]
    #[doc = "The `getElementsByTagNameNS()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ShadowRoot/getElementsByTagNameNS)\n\n*This API requires the following crate features to be activated: `HtmlCollection`, `ShadowRoot`*"]
    pub fn get_elements_by_tag_name_ns(
        this: &ShadowRoot,
        namespace: Option<&str>,
        local_name: &str,
    ) -> HtmlCollection;
    #[cfg(feature = "Element")]
    # [ wasm_bindgen ( method , structural , js_class = "ShadowRoot" , js_name = elementFromPoint ) ]
    #[doc = "The `elementFromPoint()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ShadowRoot/elementFromPoint)\n\n*This API requires the following crate features to be activated: `Element`, `ShadowRoot`*"]
    pub fn element_from_point(this: &ShadowRoot, x: f32, y: f32) -> Option<Element>;
    # [ wasm_bindgen ( method , structural , js_class = "ShadowRoot" , js_name = elementsFromPoint ) ]
    #[doc = "The `elementsFromPoint()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ShadowRoot/elementsFromPoint)\n\n*This API requires the following crate features to be activated: `ShadowRoot`*"]
    pub fn elements_from_point(this: &ShadowRoot, x: f32, y: f32) -> ::js_sys::Array;
}
