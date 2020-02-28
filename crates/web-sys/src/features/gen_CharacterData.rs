use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = CharacterData , typescript_name = CharacterData ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `CharacterData` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData)\n\n*This API requires the following crate features to be activated: `CharacterData`*"]
    pub type CharacterData;
    # [ wasm_bindgen ( structural , method , getter , js_name = data ) ]
    #[doc = "Getter for the `data` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/data)\n\n*This API requires the following crate features to be activated: `CharacterData`*"]
    pub fn data(this: &CharacterData) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = data ) ]
    #[doc = "Setter for the `data` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/data)\n\n*This API requires the following crate features to be activated: `CharacterData`*"]
    pub fn set_data(this: &CharacterData, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_name = length ) ]
    #[doc = "Getter for the `length` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/length)\n\n*This API requires the following crate features to be activated: `CharacterData`*"]
    pub fn length(this: &CharacterData) -> u32;
    # [ wasm_bindgen ( structural , method , getter , js_name = previousElementSibling ) ]
    #[cfg(feature = "Element")]
    #[doc = "Getter for the `previousElementSibling` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/previousElementSibling)\n\n*This API requires the following crate features to be activated: `CharacterData`, `Element`*"]
    pub fn previous_element_sibling(this: &CharacterData) -> Option<Element>;
    # [ wasm_bindgen ( structural , method , getter , js_name = nextElementSibling ) ]
    #[cfg(feature = "Element")]
    #[doc = "Getter for the `nextElementSibling` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/nextElementSibling)\n\n*This API requires the following crate features to be activated: `CharacterData`, `Element`*"]
    pub fn next_element_sibling(this: &CharacterData) -> Option<Element>;
    # [ wasm_bindgen ( catch , method , structural , js_name = appendData ) ]
    #[doc = "The `appendData()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/appendData)\n\n*This API requires the following crate features to be activated: `CharacterData`*"]
    pub fn append_data(this: &CharacterData, data: &str) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = deleteData ) ]
    #[doc = "The `deleteData()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/deleteData)\n\n*This API requires the following crate features to be activated: `CharacterData`*"]
    pub fn delete_data(this: &CharacterData, offset: u32, count: u32) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = insertData ) ]
    #[doc = "The `insertData()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/insertData)\n\n*This API requires the following crate features to be activated: `CharacterData`*"]
    pub fn insert_data(this: &CharacterData, offset: u32, data: &str) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = replaceData ) ]
    #[doc = "The `replaceData()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/replaceData)\n\n*This API requires the following crate features to be activated: `CharacterData`*"]
    pub fn replace_data(
        this: &CharacterData,
        offset: u32,
        count: u32,
        data: &str,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = substringData ) ]
    #[doc = "The `substringData()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/substringData)\n\n*This API requires the following crate features to be activated: `CharacterData`*"]
    pub fn substring_data(this: &CharacterData, offset: u32, count: u32)
        -> Result<String, JsValue>;
    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( catch , method , structural , variadic , js_name = after ) ]
    #[doc = "The `after()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/after)\n\n*This API requires the following crate features to be activated: `CharacterData`, `Node`*"]
    pub fn after_with_node(this: &CharacterData, nodes: &Node) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = after ) ]
    #[doc = "The `after()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/after)\n\n*This API requires the following crate features to be activated: `CharacterData`*"]
    pub fn after_with_node_0(this: &CharacterData) -> Result<(), JsValue>;
    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( catch , method , structural , js_name = after ) ]
    #[doc = "The `after()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/after)\n\n*This API requires the following crate features to be activated: `CharacterData`, `Node`*"]
    pub fn after_with_node_1(this: &CharacterData, nodes_1: &Node) -> Result<(), JsValue>;
    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( catch , method , structural , js_name = after ) ]
    #[doc = "The `after()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/after)\n\n*This API requires the following crate features to be activated: `CharacterData`, `Node`*"]
    pub fn after_with_node_2(
        this: &CharacterData,
        nodes_1: &Node,
        nodes_2: &Node,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( catch , method , structural , js_name = after ) ]
    #[doc = "The `after()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/after)\n\n*This API requires the following crate features to be activated: `CharacterData`, `Node`*"]
    pub fn after_with_node_3(
        this: &CharacterData,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( catch , method , structural , js_name = after ) ]
    #[doc = "The `after()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/after)\n\n*This API requires the following crate features to be activated: `CharacterData`, `Node`*"]
    pub fn after_with_node_4(
        this: &CharacterData,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( catch , method , structural , js_name = after ) ]
    #[doc = "The `after()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/after)\n\n*This API requires the following crate features to be activated: `CharacterData`, `Node`*"]
    pub fn after_with_node_5(
        this: &CharacterData,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
        nodes_5: &Node,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( catch , method , structural , js_name = after ) ]
    #[doc = "The `after()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/after)\n\n*This API requires the following crate features to be activated: `CharacterData`, `Node`*"]
    pub fn after_with_node_6(
        this: &CharacterData,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
        nodes_5: &Node,
        nodes_6: &Node,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( catch , method , structural , js_name = after ) ]
    #[doc = "The `after()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/after)\n\n*This API requires the following crate features to be activated: `CharacterData`, `Node`*"]
    pub fn after_with_node_7(
        this: &CharacterData,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
        nodes_5: &Node,
        nodes_6: &Node,
        nodes_7: &Node,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , variadic , js_name = after ) ]
    #[doc = "The `after()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/after)\n\n*This API requires the following crate features to be activated: `CharacterData`*"]
    pub fn after_with_str(this: &CharacterData, nodes: &str) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = after ) ]
    #[doc = "The `after()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/after)\n\n*This API requires the following crate features to be activated: `CharacterData`*"]
    pub fn after_with_str_0(this: &CharacterData) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = after ) ]
    #[doc = "The `after()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/after)\n\n*This API requires the following crate features to be activated: `CharacterData`*"]
    pub fn after_with_str_1(this: &CharacterData, nodes_1: &str) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = after ) ]
    #[doc = "The `after()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/after)\n\n*This API requires the following crate features to be activated: `CharacterData`*"]
    pub fn after_with_str_2(
        this: &CharacterData,
        nodes_1: &str,
        nodes_2: &str,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = after ) ]
    #[doc = "The `after()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/after)\n\n*This API requires the following crate features to be activated: `CharacterData`*"]
    pub fn after_with_str_3(
        this: &CharacterData,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = after ) ]
    #[doc = "The `after()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/after)\n\n*This API requires the following crate features to be activated: `CharacterData`*"]
    pub fn after_with_str_4(
        this: &CharacterData,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = after ) ]
    #[doc = "The `after()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/after)\n\n*This API requires the following crate features to be activated: `CharacterData`*"]
    pub fn after_with_str_5(
        this: &CharacterData,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
        nodes_5: &str,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = after ) ]
    #[doc = "The `after()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/after)\n\n*This API requires the following crate features to be activated: `CharacterData`*"]
    pub fn after_with_str_6(
        this: &CharacterData,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
        nodes_5: &str,
        nodes_6: &str,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = after ) ]
    #[doc = "The `after()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/after)\n\n*This API requires the following crate features to be activated: `CharacterData`*"]
    pub fn after_with_str_7(
        this: &CharacterData,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
        nodes_5: &str,
        nodes_6: &str,
        nodes_7: &str,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( catch , method , structural , variadic , js_name = before ) ]
    #[doc = "The `before()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/before)\n\n*This API requires the following crate features to be activated: `CharacterData`, `Node`*"]
    pub fn before_with_node(this: &CharacterData, nodes: &Node) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = before ) ]
    #[doc = "The `before()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/before)\n\n*This API requires the following crate features to be activated: `CharacterData`*"]
    pub fn before_with_node_0(this: &CharacterData) -> Result<(), JsValue>;
    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( catch , method , structural , js_name = before ) ]
    #[doc = "The `before()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/before)\n\n*This API requires the following crate features to be activated: `CharacterData`, `Node`*"]
    pub fn before_with_node_1(this: &CharacterData, nodes_1: &Node) -> Result<(), JsValue>;
    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( catch , method , structural , js_name = before ) ]
    #[doc = "The `before()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/before)\n\n*This API requires the following crate features to be activated: `CharacterData`, `Node`*"]
    pub fn before_with_node_2(
        this: &CharacterData,
        nodes_1: &Node,
        nodes_2: &Node,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( catch , method , structural , js_name = before ) ]
    #[doc = "The `before()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/before)\n\n*This API requires the following crate features to be activated: `CharacterData`, `Node`*"]
    pub fn before_with_node_3(
        this: &CharacterData,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( catch , method , structural , js_name = before ) ]
    #[doc = "The `before()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/before)\n\n*This API requires the following crate features to be activated: `CharacterData`, `Node`*"]
    pub fn before_with_node_4(
        this: &CharacterData,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( catch , method , structural , js_name = before ) ]
    #[doc = "The `before()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/before)\n\n*This API requires the following crate features to be activated: `CharacterData`, `Node`*"]
    pub fn before_with_node_5(
        this: &CharacterData,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
        nodes_5: &Node,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( catch , method , structural , js_name = before ) ]
    #[doc = "The `before()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/before)\n\n*This API requires the following crate features to be activated: `CharacterData`, `Node`*"]
    pub fn before_with_node_6(
        this: &CharacterData,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
        nodes_5: &Node,
        nodes_6: &Node,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( catch , method , structural , js_name = before ) ]
    #[doc = "The `before()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/before)\n\n*This API requires the following crate features to be activated: `CharacterData`, `Node`*"]
    pub fn before_with_node_7(
        this: &CharacterData,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
        nodes_5: &Node,
        nodes_6: &Node,
        nodes_7: &Node,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , variadic , js_name = before ) ]
    #[doc = "The `before()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/before)\n\n*This API requires the following crate features to be activated: `CharacterData`*"]
    pub fn before_with_str(this: &CharacterData, nodes: &str) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = before ) ]
    #[doc = "The `before()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/before)\n\n*This API requires the following crate features to be activated: `CharacterData`*"]
    pub fn before_with_str_0(this: &CharacterData) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = before ) ]
    #[doc = "The `before()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/before)\n\n*This API requires the following crate features to be activated: `CharacterData`*"]
    pub fn before_with_str_1(this: &CharacterData, nodes_1: &str) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = before ) ]
    #[doc = "The `before()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/before)\n\n*This API requires the following crate features to be activated: `CharacterData`*"]
    pub fn before_with_str_2(
        this: &CharacterData,
        nodes_1: &str,
        nodes_2: &str,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = before ) ]
    #[doc = "The `before()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/before)\n\n*This API requires the following crate features to be activated: `CharacterData`*"]
    pub fn before_with_str_3(
        this: &CharacterData,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = before ) ]
    #[doc = "The `before()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/before)\n\n*This API requires the following crate features to be activated: `CharacterData`*"]
    pub fn before_with_str_4(
        this: &CharacterData,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = before ) ]
    #[doc = "The `before()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/before)\n\n*This API requires the following crate features to be activated: `CharacterData`*"]
    pub fn before_with_str_5(
        this: &CharacterData,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
        nodes_5: &str,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = before ) ]
    #[doc = "The `before()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/before)\n\n*This API requires the following crate features to be activated: `CharacterData`*"]
    pub fn before_with_str_6(
        this: &CharacterData,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
        nodes_5: &str,
        nodes_6: &str,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = before ) ]
    #[doc = "The `before()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/before)\n\n*This API requires the following crate features to be activated: `CharacterData`*"]
    pub fn before_with_str_7(
        this: &CharacterData,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
        nodes_5: &str,
        nodes_6: &str,
        nodes_7: &str,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( method , structural , js_name = remove ) ]
    #[doc = "The `remove()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/remove)\n\n*This API requires the following crate features to be activated: `CharacterData`*"]
    pub fn remove(this: &CharacterData);
    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( catch , method , structural , variadic , js_name = replaceWith ) ]
    #[doc = "The `replaceWith()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/replaceWith)\n\n*This API requires the following crate features to be activated: `CharacterData`, `Node`*"]
    pub fn replace_with_with_node(this: &CharacterData, nodes: &Node) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = replaceWith ) ]
    #[doc = "The `replaceWith()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/replaceWith)\n\n*This API requires the following crate features to be activated: `CharacterData`*"]
    pub fn replace_with_with_node_0(this: &CharacterData) -> Result<(), JsValue>;
    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( catch , method , structural , js_name = replaceWith ) ]
    #[doc = "The `replaceWith()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/replaceWith)\n\n*This API requires the following crate features to be activated: `CharacterData`, `Node`*"]
    pub fn replace_with_with_node_1(this: &CharacterData, nodes_1: &Node) -> Result<(), JsValue>;
    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( catch , method , structural , js_name = replaceWith ) ]
    #[doc = "The `replaceWith()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/replaceWith)\n\n*This API requires the following crate features to be activated: `CharacterData`, `Node`*"]
    pub fn replace_with_with_node_2(
        this: &CharacterData,
        nodes_1: &Node,
        nodes_2: &Node,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( catch , method , structural , js_name = replaceWith ) ]
    #[doc = "The `replaceWith()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/replaceWith)\n\n*This API requires the following crate features to be activated: `CharacterData`, `Node`*"]
    pub fn replace_with_with_node_3(
        this: &CharacterData,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( catch , method , structural , js_name = replaceWith ) ]
    #[doc = "The `replaceWith()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/replaceWith)\n\n*This API requires the following crate features to be activated: `CharacterData`, `Node`*"]
    pub fn replace_with_with_node_4(
        this: &CharacterData,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( catch , method , structural , js_name = replaceWith ) ]
    #[doc = "The `replaceWith()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/replaceWith)\n\n*This API requires the following crate features to be activated: `CharacterData`, `Node`*"]
    pub fn replace_with_with_node_5(
        this: &CharacterData,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
        nodes_5: &Node,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( catch , method , structural , js_name = replaceWith ) ]
    #[doc = "The `replaceWith()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/replaceWith)\n\n*This API requires the following crate features to be activated: `CharacterData`, `Node`*"]
    pub fn replace_with_with_node_6(
        this: &CharacterData,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
        nodes_5: &Node,
        nodes_6: &Node,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( catch , method , structural , js_name = replaceWith ) ]
    #[doc = "The `replaceWith()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/replaceWith)\n\n*This API requires the following crate features to be activated: `CharacterData`, `Node`*"]
    pub fn replace_with_with_node_7(
        this: &CharacterData,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
        nodes_5: &Node,
        nodes_6: &Node,
        nodes_7: &Node,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , variadic , js_name = replaceWith ) ]
    #[doc = "The `replaceWith()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/replaceWith)\n\n*This API requires the following crate features to be activated: `CharacterData`*"]
    pub fn replace_with_with_str(this: &CharacterData, nodes: &str) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = replaceWith ) ]
    #[doc = "The `replaceWith()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/replaceWith)\n\n*This API requires the following crate features to be activated: `CharacterData`*"]
    pub fn replace_with_with_str_0(this: &CharacterData) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = replaceWith ) ]
    #[doc = "The `replaceWith()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/replaceWith)\n\n*This API requires the following crate features to be activated: `CharacterData`*"]
    pub fn replace_with_with_str_1(this: &CharacterData, nodes_1: &str) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = replaceWith ) ]
    #[doc = "The `replaceWith()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/replaceWith)\n\n*This API requires the following crate features to be activated: `CharacterData`*"]
    pub fn replace_with_with_str_2(
        this: &CharacterData,
        nodes_1: &str,
        nodes_2: &str,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = replaceWith ) ]
    #[doc = "The `replaceWith()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/replaceWith)\n\n*This API requires the following crate features to be activated: `CharacterData`*"]
    pub fn replace_with_with_str_3(
        this: &CharacterData,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = replaceWith ) ]
    #[doc = "The `replaceWith()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/replaceWith)\n\n*This API requires the following crate features to be activated: `CharacterData`*"]
    pub fn replace_with_with_str_4(
        this: &CharacterData,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = replaceWith ) ]
    #[doc = "The `replaceWith()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/replaceWith)\n\n*This API requires the following crate features to be activated: `CharacterData`*"]
    pub fn replace_with_with_str_5(
        this: &CharacterData,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
        nodes_5: &str,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = replaceWith ) ]
    #[doc = "The `replaceWith()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/replaceWith)\n\n*This API requires the following crate features to be activated: `CharacterData`*"]
    pub fn replace_with_with_str_6(
        this: &CharacterData,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
        nodes_5: &str,
        nodes_6: &str,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = replaceWith ) ]
    #[doc = "The `replaceWith()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/replaceWith)\n\n*This API requires the following crate features to be activated: `CharacterData`*"]
    pub fn replace_with_with_str_7(
        this: &CharacterData,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
        nodes_5: &str,
        nodes_6: &str,
        nodes_7: &str,
    ) -> Result<(), JsValue>;
}
