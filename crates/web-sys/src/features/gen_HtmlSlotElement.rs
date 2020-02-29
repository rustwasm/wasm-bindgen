use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = HtmlElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = HTMLSlotElement , typescript_type = "HTMLSlotElement" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `HtmlSlotElement` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSlotElement)
    ///
    ///*This API requires the following crate features to be activated: `HtmlSlotElement`*
    pub type HtmlSlotElement;

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLSlotElement" , js_name = name ) ]
    ///Getter for the `name` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSlotElement/name)
    ///
    ///*This API requires the following crate features to be activated: `HtmlSlotElement`*
    pub fn name(this: &HtmlSlotElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLSlotElement" , js_name = name ) ]
    ///Setter for the `name` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSlotElement/name)
    ///
    ///*This API requires the following crate features to be activated: `HtmlSlotElement`*
    pub fn set_name(this: &HtmlSlotElement, value: &str);

    # [ wasm_bindgen ( method , structural , js_class = "HTMLSlotElement" , js_name = assignedNodes ) ]
    ///The `assignedNodes()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSlotElement/assignedNodes)
    ///
    ///*This API requires the following crate features to be activated: `HtmlSlotElement`*
    pub fn assigned_nodes(this: &HtmlSlotElement) -> ::js_sys::Array;

    #[cfg(feature = "AssignedNodesOptions")]
    # [ wasm_bindgen ( method , structural , js_class = "HTMLSlotElement" , js_name = assignedNodes ) ]
    ///The `assignedNodes()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSlotElement/assignedNodes)
    ///
    ///*This API requires the following crate features to be activated: `AssignedNodesOptions`, `HtmlSlotElement`*
    pub fn assigned_nodes_with_options(
        this: &HtmlSlotElement,
        options: &AssignedNodesOptions,
    ) -> ::js_sys::Array;

}
