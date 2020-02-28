use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = HtmlElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = HTMLSlotElement , typescript_name = HTMLSlotElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlSlotElement` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSlotElement)\n\n*This API requires the following crate features to be activated: `HtmlSlotElement`*"]
    pub type HtmlSlotElement;
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLSlotElement" , js_name = name ) ]
    #[doc = "Getter for the `name` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSlotElement/name)\n\n*This API requires the following crate features to be activated: `HtmlSlotElement`*"]
    pub fn name(this: &HtmlSlotElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLSlotElement" , js_name = name ) ]
    #[doc = "Setter for the `name` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSlotElement/name)\n\n*This API requires the following crate features to be activated: `HtmlSlotElement`*"]
    pub fn set_name(this: &HtmlSlotElement, value: &str);
    # [ wasm_bindgen ( method , structural , js_class = "HTMLSlotElement" , js_name = assignedNodes ) ]
    #[doc = "The `assignedNodes()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSlotElement/assignedNodes)\n\n*This API requires the following crate features to be activated: `HtmlSlotElement`*"]
    pub fn assigned_nodes(this: &HtmlSlotElement) -> ::js_sys::Array;
    #[cfg(feature = "AssignedNodesOptions")]
    # [ wasm_bindgen ( method , structural , js_class = "HTMLSlotElement" , js_name = assignedNodes ) ]
    #[doc = "The `assignedNodes()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSlotElement/assignedNodes)\n\n*This API requires the following crate features to be activated: `AssignedNodesOptions`, `HtmlSlotElement`*"]
    pub fn assigned_nodes_with_options(
        this: &HtmlSlotElement,
        options: &AssignedNodesOptions,
    ) -> ::js_sys::Array;
}
