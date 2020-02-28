use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = DocumentType , typescript_name = DocumentType ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DocumentType` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType)\n\n*This API requires the following crate features to be activated: `DocumentType`*"]
    pub type DocumentType;
    # [ wasm_bindgen ( structural , method , getter , js_name = name ) ]
    #[doc = "Getter for the `name` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/name)\n\n*This API requires the following crate features to be activated: `DocumentType`*"]
    pub fn name(this: &DocumentType) -> String;
    # [ wasm_bindgen ( structural , method , getter , js_name = publicId ) ]
    #[doc = "Getter for the `publicId` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/publicId)\n\n*This API requires the following crate features to be activated: `DocumentType`*"]
    pub fn public_id(this: &DocumentType) -> String;
    # [ wasm_bindgen ( structural , method , getter , js_name = systemId ) ]
    #[doc = "Getter for the `systemId` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/systemId)\n\n*This API requires the following crate features to be activated: `DocumentType`*"]
    pub fn system_id(this: &DocumentType) -> String;
    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( catch , method , structural , variadic , js_name = after ) ]
    #[doc = "The `after()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/after)\n\n*This API requires the following crate features to be activated: `DocumentType`, `Node`*"]
    pub fn after_with_node(this: &DocumentType, nodes: &Node) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = after ) ]
    #[doc = "The `after()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/after)\n\n*This API requires the following crate features to be activated: `DocumentType`*"]
    pub fn after_with_node_0(this: &DocumentType) -> Result<(), JsValue>;
    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( catch , method , structural , js_name = after ) ]
    #[doc = "The `after()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/after)\n\n*This API requires the following crate features to be activated: `DocumentType`, `Node`*"]
    pub fn after_with_node_1(this: &DocumentType, nodes_1: &Node) -> Result<(), JsValue>;
    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( catch , method , structural , js_name = after ) ]
    #[doc = "The `after()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/after)\n\n*This API requires the following crate features to be activated: `DocumentType`, `Node`*"]
    pub fn after_with_node_2(
        this: &DocumentType,
        nodes_1: &Node,
        nodes_2: &Node,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( catch , method , structural , js_name = after ) ]
    #[doc = "The `after()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/after)\n\n*This API requires the following crate features to be activated: `DocumentType`, `Node`*"]
    pub fn after_with_node_3(
        this: &DocumentType,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( catch , method , structural , js_name = after ) ]
    #[doc = "The `after()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/after)\n\n*This API requires the following crate features to be activated: `DocumentType`, `Node`*"]
    pub fn after_with_node_4(
        this: &DocumentType,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( catch , method , structural , js_name = after ) ]
    #[doc = "The `after()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/after)\n\n*This API requires the following crate features to be activated: `DocumentType`, `Node`*"]
    pub fn after_with_node_5(
        this: &DocumentType,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
        nodes_5: &Node,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( catch , method , structural , js_name = after ) ]
    #[doc = "The `after()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/after)\n\n*This API requires the following crate features to be activated: `DocumentType`, `Node`*"]
    pub fn after_with_node_6(
        this: &DocumentType,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
        nodes_5: &Node,
        nodes_6: &Node,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( catch , method , structural , js_name = after ) ]
    #[doc = "The `after()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/after)\n\n*This API requires the following crate features to be activated: `DocumentType`, `Node`*"]
    pub fn after_with_node_7(
        this: &DocumentType,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
        nodes_5: &Node,
        nodes_6: &Node,
        nodes_7: &Node,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , variadic , js_name = after ) ]
    #[doc = "The `after()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/after)\n\n*This API requires the following crate features to be activated: `DocumentType`*"]
    pub fn after_with_str(this: &DocumentType, nodes: &str) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = after ) ]
    #[doc = "The `after()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/after)\n\n*This API requires the following crate features to be activated: `DocumentType`*"]
    pub fn after_with_str_0(this: &DocumentType) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = after ) ]
    #[doc = "The `after()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/after)\n\n*This API requires the following crate features to be activated: `DocumentType`*"]
    pub fn after_with_str_1(this: &DocumentType, nodes_1: &str) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = after ) ]
    #[doc = "The `after()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/after)\n\n*This API requires the following crate features to be activated: `DocumentType`*"]
    pub fn after_with_str_2(
        this: &DocumentType,
        nodes_1: &str,
        nodes_2: &str,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = after ) ]
    #[doc = "The `after()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/after)\n\n*This API requires the following crate features to be activated: `DocumentType`*"]
    pub fn after_with_str_3(
        this: &DocumentType,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = after ) ]
    #[doc = "The `after()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/after)\n\n*This API requires the following crate features to be activated: `DocumentType`*"]
    pub fn after_with_str_4(
        this: &DocumentType,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = after ) ]
    #[doc = "The `after()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/after)\n\n*This API requires the following crate features to be activated: `DocumentType`*"]
    pub fn after_with_str_5(
        this: &DocumentType,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
        nodes_5: &str,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = after ) ]
    #[doc = "The `after()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/after)\n\n*This API requires the following crate features to be activated: `DocumentType`*"]
    pub fn after_with_str_6(
        this: &DocumentType,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
        nodes_5: &str,
        nodes_6: &str,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = after ) ]
    #[doc = "The `after()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/after)\n\n*This API requires the following crate features to be activated: `DocumentType`*"]
    pub fn after_with_str_7(
        this: &DocumentType,
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
    #[doc = "The `before()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/before)\n\n*This API requires the following crate features to be activated: `DocumentType`, `Node`*"]
    pub fn before_with_node(this: &DocumentType, nodes: &Node) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = before ) ]
    #[doc = "The `before()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/before)\n\n*This API requires the following crate features to be activated: `DocumentType`*"]
    pub fn before_with_node_0(this: &DocumentType) -> Result<(), JsValue>;
    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( catch , method , structural , js_name = before ) ]
    #[doc = "The `before()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/before)\n\n*This API requires the following crate features to be activated: `DocumentType`, `Node`*"]
    pub fn before_with_node_1(this: &DocumentType, nodes_1: &Node) -> Result<(), JsValue>;
    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( catch , method , structural , js_name = before ) ]
    #[doc = "The `before()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/before)\n\n*This API requires the following crate features to be activated: `DocumentType`, `Node`*"]
    pub fn before_with_node_2(
        this: &DocumentType,
        nodes_1: &Node,
        nodes_2: &Node,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( catch , method , structural , js_name = before ) ]
    #[doc = "The `before()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/before)\n\n*This API requires the following crate features to be activated: `DocumentType`, `Node`*"]
    pub fn before_with_node_3(
        this: &DocumentType,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( catch , method , structural , js_name = before ) ]
    #[doc = "The `before()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/before)\n\n*This API requires the following crate features to be activated: `DocumentType`, `Node`*"]
    pub fn before_with_node_4(
        this: &DocumentType,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( catch , method , structural , js_name = before ) ]
    #[doc = "The `before()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/before)\n\n*This API requires the following crate features to be activated: `DocumentType`, `Node`*"]
    pub fn before_with_node_5(
        this: &DocumentType,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
        nodes_5: &Node,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( catch , method , structural , js_name = before ) ]
    #[doc = "The `before()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/before)\n\n*This API requires the following crate features to be activated: `DocumentType`, `Node`*"]
    pub fn before_with_node_6(
        this: &DocumentType,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
        nodes_5: &Node,
        nodes_6: &Node,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( catch , method , structural , js_name = before ) ]
    #[doc = "The `before()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/before)\n\n*This API requires the following crate features to be activated: `DocumentType`, `Node`*"]
    pub fn before_with_node_7(
        this: &DocumentType,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
        nodes_5: &Node,
        nodes_6: &Node,
        nodes_7: &Node,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , variadic , js_name = before ) ]
    #[doc = "The `before()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/before)\n\n*This API requires the following crate features to be activated: `DocumentType`*"]
    pub fn before_with_str(this: &DocumentType, nodes: &str) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = before ) ]
    #[doc = "The `before()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/before)\n\n*This API requires the following crate features to be activated: `DocumentType`*"]
    pub fn before_with_str_0(this: &DocumentType) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = before ) ]
    #[doc = "The `before()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/before)\n\n*This API requires the following crate features to be activated: `DocumentType`*"]
    pub fn before_with_str_1(this: &DocumentType, nodes_1: &str) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = before ) ]
    #[doc = "The `before()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/before)\n\n*This API requires the following crate features to be activated: `DocumentType`*"]
    pub fn before_with_str_2(
        this: &DocumentType,
        nodes_1: &str,
        nodes_2: &str,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = before ) ]
    #[doc = "The `before()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/before)\n\n*This API requires the following crate features to be activated: `DocumentType`*"]
    pub fn before_with_str_3(
        this: &DocumentType,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = before ) ]
    #[doc = "The `before()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/before)\n\n*This API requires the following crate features to be activated: `DocumentType`*"]
    pub fn before_with_str_4(
        this: &DocumentType,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = before ) ]
    #[doc = "The `before()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/before)\n\n*This API requires the following crate features to be activated: `DocumentType`*"]
    pub fn before_with_str_5(
        this: &DocumentType,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
        nodes_5: &str,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = before ) ]
    #[doc = "The `before()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/before)\n\n*This API requires the following crate features to be activated: `DocumentType`*"]
    pub fn before_with_str_6(
        this: &DocumentType,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
        nodes_5: &str,
        nodes_6: &str,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = before ) ]
    #[doc = "The `before()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/before)\n\n*This API requires the following crate features to be activated: `DocumentType`*"]
    pub fn before_with_str_7(
        this: &DocumentType,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
        nodes_5: &str,
        nodes_6: &str,
        nodes_7: &str,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( method , structural , js_name = remove ) ]
    #[doc = "The `remove()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/remove)\n\n*This API requires the following crate features to be activated: `DocumentType`*"]
    pub fn remove(this: &DocumentType);
    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( catch , method , structural , variadic , js_name = replaceWith ) ]
    #[doc = "The `replaceWith()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/replaceWith)\n\n*This API requires the following crate features to be activated: `DocumentType`, `Node`*"]
    pub fn replace_with_with_node(this: &DocumentType, nodes: &Node) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = replaceWith ) ]
    #[doc = "The `replaceWith()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/replaceWith)\n\n*This API requires the following crate features to be activated: `DocumentType`*"]
    pub fn replace_with_with_node_0(this: &DocumentType) -> Result<(), JsValue>;
    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( catch , method , structural , js_name = replaceWith ) ]
    #[doc = "The `replaceWith()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/replaceWith)\n\n*This API requires the following crate features to be activated: `DocumentType`, `Node`*"]
    pub fn replace_with_with_node_1(this: &DocumentType, nodes_1: &Node) -> Result<(), JsValue>;
    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( catch , method , structural , js_name = replaceWith ) ]
    #[doc = "The `replaceWith()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/replaceWith)\n\n*This API requires the following crate features to be activated: `DocumentType`, `Node`*"]
    pub fn replace_with_with_node_2(
        this: &DocumentType,
        nodes_1: &Node,
        nodes_2: &Node,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( catch , method , structural , js_name = replaceWith ) ]
    #[doc = "The `replaceWith()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/replaceWith)\n\n*This API requires the following crate features to be activated: `DocumentType`, `Node`*"]
    pub fn replace_with_with_node_3(
        this: &DocumentType,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( catch , method , structural , js_name = replaceWith ) ]
    #[doc = "The `replaceWith()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/replaceWith)\n\n*This API requires the following crate features to be activated: `DocumentType`, `Node`*"]
    pub fn replace_with_with_node_4(
        this: &DocumentType,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( catch , method , structural , js_name = replaceWith ) ]
    #[doc = "The `replaceWith()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/replaceWith)\n\n*This API requires the following crate features to be activated: `DocumentType`, `Node`*"]
    pub fn replace_with_with_node_5(
        this: &DocumentType,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
        nodes_5: &Node,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( catch , method , structural , js_name = replaceWith ) ]
    #[doc = "The `replaceWith()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/replaceWith)\n\n*This API requires the following crate features to be activated: `DocumentType`, `Node`*"]
    pub fn replace_with_with_node_6(
        this: &DocumentType,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
        nodes_5: &Node,
        nodes_6: &Node,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( catch , method , structural , js_name = replaceWith ) ]
    #[doc = "The `replaceWith()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/replaceWith)\n\n*This API requires the following crate features to be activated: `DocumentType`, `Node`*"]
    pub fn replace_with_with_node_7(
        this: &DocumentType,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
        nodes_5: &Node,
        nodes_6: &Node,
        nodes_7: &Node,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , variadic , js_name = replaceWith ) ]
    #[doc = "The `replaceWith()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/replaceWith)\n\n*This API requires the following crate features to be activated: `DocumentType`*"]
    pub fn replace_with_with_str(this: &DocumentType, nodes: &str) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = replaceWith ) ]
    #[doc = "The `replaceWith()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/replaceWith)\n\n*This API requires the following crate features to be activated: `DocumentType`*"]
    pub fn replace_with_with_str_0(this: &DocumentType) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = replaceWith ) ]
    #[doc = "The `replaceWith()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/replaceWith)\n\n*This API requires the following crate features to be activated: `DocumentType`*"]
    pub fn replace_with_with_str_1(this: &DocumentType, nodes_1: &str) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = replaceWith ) ]
    #[doc = "The `replaceWith()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/replaceWith)\n\n*This API requires the following crate features to be activated: `DocumentType`*"]
    pub fn replace_with_with_str_2(
        this: &DocumentType,
        nodes_1: &str,
        nodes_2: &str,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = replaceWith ) ]
    #[doc = "The `replaceWith()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/replaceWith)\n\n*This API requires the following crate features to be activated: `DocumentType`*"]
    pub fn replace_with_with_str_3(
        this: &DocumentType,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = replaceWith ) ]
    #[doc = "The `replaceWith()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/replaceWith)\n\n*This API requires the following crate features to be activated: `DocumentType`*"]
    pub fn replace_with_with_str_4(
        this: &DocumentType,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = replaceWith ) ]
    #[doc = "The `replaceWith()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/replaceWith)\n\n*This API requires the following crate features to be activated: `DocumentType`*"]
    pub fn replace_with_with_str_5(
        this: &DocumentType,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
        nodes_5: &str,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = replaceWith ) ]
    #[doc = "The `replaceWith()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/replaceWith)\n\n*This API requires the following crate features to be activated: `DocumentType`*"]
    pub fn replace_with_with_str_6(
        this: &DocumentType,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
        nodes_5: &str,
        nodes_6: &str,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = replaceWith ) ]
    #[doc = "The `replaceWith()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/replaceWith)\n\n*This API requires the following crate features to be activated: `DocumentType`*"]
    pub fn replace_with_with_str_7(
        this: &DocumentType,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
        nodes_5: &str,
        nodes_6: &str,
        nodes_7: &str,
    ) -> Result<(), JsValue>;
}
