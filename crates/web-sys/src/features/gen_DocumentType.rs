use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = DocumentType , typescript_type = "DocumentType" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `DocumentType` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType)
    ///
    ///*This API requires the following crate features to be activated: `DocumentType`*
    pub type DocumentType;

    # [ wasm_bindgen ( structural , method , getter , js_class = "DocumentType" , js_name = name ) ]
    ///Getter for the `name` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/name)
    ///
    ///*This API requires the following crate features to be activated: `DocumentType`*
    pub fn name(this: &DocumentType) -> String;

    # [ wasm_bindgen ( structural , method , getter , js_class = "DocumentType" , js_name = publicId ) ]
    ///Getter for the `publicId` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/publicId)
    ///
    ///*This API requires the following crate features to be activated: `DocumentType`*
    pub fn public_id(this: &DocumentType) -> String;

    # [ wasm_bindgen ( structural , method , getter , js_class = "DocumentType" , js_name = systemId ) ]
    ///Getter for the `systemId` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/systemId)
    ///
    ///*This API requires the following crate features to be activated: `DocumentType`*
    pub fn system_id(this: &DocumentType) -> String;

    # [ wasm_bindgen ( catch , method , structural , variadic , js_class = "DocumentType" , js_name = after ) ]
    ///The `after()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/after)
    ///
    ///*This API requires the following crate features to be activated: `DocumentType`*
    pub fn after_with_node(this: &DocumentType, nodes: &::js_sys::Array) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "DocumentType" , js_name = after ) ]
    ///The `after()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/after)
    ///
    ///*This API requires the following crate features to be activated: `DocumentType`*
    pub fn after_with_node_0(this: &DocumentType) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "DocumentType" , js_name = after ) ]
    ///The `after()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/after)
    ///
    ///*This API requires the following crate features to be activated: `DocumentType`*
    pub fn after_with_node_1(this: &DocumentType, nodes_1: &Node) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "DocumentType" , js_name = after ) ]
    ///The `after()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/after)
    ///
    ///*This API requires the following crate features to be activated: `DocumentType`*
    pub fn after_with_node_2(
        this: &DocumentType,
        nodes_1: &Node,
        nodes_2: &Node,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "DocumentType" , js_name = after ) ]
    ///The `after()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/after)
    ///
    ///*This API requires the following crate features to be activated: `DocumentType`*
    pub fn after_with_node_3(
        this: &DocumentType,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "DocumentType" , js_name = after ) ]
    ///The `after()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/after)
    ///
    ///*This API requires the following crate features to be activated: `DocumentType`*
    pub fn after_with_node_4(
        this: &DocumentType,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "DocumentType" , js_name = after ) ]
    ///The `after()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/after)
    ///
    ///*This API requires the following crate features to be activated: `DocumentType`*
    pub fn after_with_node_5(
        this: &DocumentType,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
        nodes_5: &Node,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "DocumentType" , js_name = after ) ]
    ///The `after()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/after)
    ///
    ///*This API requires the following crate features to be activated: `DocumentType`*
    pub fn after_with_node_6(
        this: &DocumentType,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
        nodes_5: &Node,
        nodes_6: &Node,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "DocumentType" , js_name = after ) ]
    ///The `after()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/after)
    ///
    ///*This API requires the following crate features to be activated: `DocumentType`*
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

    # [ wasm_bindgen ( catch , method , structural , variadic , js_class = "DocumentType" , js_name = after ) ]
    ///The `after()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/after)
    ///
    ///*This API requires the following crate features to be activated: `DocumentType`*
    pub fn after_with_str(this: &DocumentType, nodes: &::js_sys::Array) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "DocumentType" , js_name = after ) ]
    ///The `after()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/after)
    ///
    ///*This API requires the following crate features to be activated: `DocumentType`*
    pub fn after_with_str_0(this: &DocumentType) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "DocumentType" , js_name = after ) ]
    ///The `after()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/after)
    ///
    ///*This API requires the following crate features to be activated: `DocumentType`*
    pub fn after_with_str_1(this: &DocumentType, nodes_1: &str) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "DocumentType" , js_name = after ) ]
    ///The `after()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/after)
    ///
    ///*This API requires the following crate features to be activated: `DocumentType`*
    pub fn after_with_str_2(
        this: &DocumentType,
        nodes_1: &str,
        nodes_2: &str,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "DocumentType" , js_name = after ) ]
    ///The `after()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/after)
    ///
    ///*This API requires the following crate features to be activated: `DocumentType`*
    pub fn after_with_str_3(
        this: &DocumentType,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "DocumentType" , js_name = after ) ]
    ///The `after()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/after)
    ///
    ///*This API requires the following crate features to be activated: `DocumentType`*
    pub fn after_with_str_4(
        this: &DocumentType,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "DocumentType" , js_name = after ) ]
    ///The `after()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/after)
    ///
    ///*This API requires the following crate features to be activated: `DocumentType`*
    pub fn after_with_str_5(
        this: &DocumentType,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
        nodes_5: &str,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "DocumentType" , js_name = after ) ]
    ///The `after()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/after)
    ///
    ///*This API requires the following crate features to be activated: `DocumentType`*
    pub fn after_with_str_6(
        this: &DocumentType,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
        nodes_5: &str,
        nodes_6: &str,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "DocumentType" , js_name = after ) ]
    ///The `after()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/after)
    ///
    ///*This API requires the following crate features to be activated: `DocumentType`*
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

    # [ wasm_bindgen ( catch , method , structural , variadic , js_class = "DocumentType" , js_name = before ) ]
    ///The `before()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/before)
    ///
    ///*This API requires the following crate features to be activated: `DocumentType`*
    pub fn before_with_node(this: &DocumentType, nodes: &::js_sys::Array) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "DocumentType" , js_name = before ) ]
    ///The `before()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/before)
    ///
    ///*This API requires the following crate features to be activated: `DocumentType`*
    pub fn before_with_node_0(this: &DocumentType) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "DocumentType" , js_name = before ) ]
    ///The `before()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/before)
    ///
    ///*This API requires the following crate features to be activated: `DocumentType`*
    pub fn before_with_node_1(this: &DocumentType, nodes_1: &Node) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "DocumentType" , js_name = before ) ]
    ///The `before()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/before)
    ///
    ///*This API requires the following crate features to be activated: `DocumentType`*
    pub fn before_with_node_2(
        this: &DocumentType,
        nodes_1: &Node,
        nodes_2: &Node,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "DocumentType" , js_name = before ) ]
    ///The `before()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/before)
    ///
    ///*This API requires the following crate features to be activated: `DocumentType`*
    pub fn before_with_node_3(
        this: &DocumentType,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "DocumentType" , js_name = before ) ]
    ///The `before()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/before)
    ///
    ///*This API requires the following crate features to be activated: `DocumentType`*
    pub fn before_with_node_4(
        this: &DocumentType,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "DocumentType" , js_name = before ) ]
    ///The `before()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/before)
    ///
    ///*This API requires the following crate features to be activated: `DocumentType`*
    pub fn before_with_node_5(
        this: &DocumentType,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
        nodes_5: &Node,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "DocumentType" , js_name = before ) ]
    ///The `before()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/before)
    ///
    ///*This API requires the following crate features to be activated: `DocumentType`*
    pub fn before_with_node_6(
        this: &DocumentType,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
        nodes_5: &Node,
        nodes_6: &Node,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "DocumentType" , js_name = before ) ]
    ///The `before()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/before)
    ///
    ///*This API requires the following crate features to be activated: `DocumentType`*
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

    # [ wasm_bindgen ( catch , method , structural , variadic , js_class = "DocumentType" , js_name = before ) ]
    ///The `before()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/before)
    ///
    ///*This API requires the following crate features to be activated: `DocumentType`*
    pub fn before_with_str(this: &DocumentType, nodes: &::js_sys::Array) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "DocumentType" , js_name = before ) ]
    ///The `before()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/before)
    ///
    ///*This API requires the following crate features to be activated: `DocumentType`*
    pub fn before_with_str_0(this: &DocumentType) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "DocumentType" , js_name = before ) ]
    ///The `before()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/before)
    ///
    ///*This API requires the following crate features to be activated: `DocumentType`*
    pub fn before_with_str_1(this: &DocumentType, nodes_1: &str) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "DocumentType" , js_name = before ) ]
    ///The `before()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/before)
    ///
    ///*This API requires the following crate features to be activated: `DocumentType`*
    pub fn before_with_str_2(
        this: &DocumentType,
        nodes_1: &str,
        nodes_2: &str,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "DocumentType" , js_name = before ) ]
    ///The `before()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/before)
    ///
    ///*This API requires the following crate features to be activated: `DocumentType`*
    pub fn before_with_str_3(
        this: &DocumentType,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "DocumentType" , js_name = before ) ]
    ///The `before()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/before)
    ///
    ///*This API requires the following crate features to be activated: `DocumentType`*
    pub fn before_with_str_4(
        this: &DocumentType,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "DocumentType" , js_name = before ) ]
    ///The `before()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/before)
    ///
    ///*This API requires the following crate features to be activated: `DocumentType`*
    pub fn before_with_str_5(
        this: &DocumentType,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
        nodes_5: &str,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "DocumentType" , js_name = before ) ]
    ///The `before()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/before)
    ///
    ///*This API requires the following crate features to be activated: `DocumentType`*
    pub fn before_with_str_6(
        this: &DocumentType,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
        nodes_5: &str,
        nodes_6: &str,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "DocumentType" , js_name = before ) ]
    ///The `before()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/before)
    ///
    ///*This API requires the following crate features to be activated: `DocumentType`*
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

    # [ wasm_bindgen ( method , structural , js_class = "DocumentType" , js_name = remove ) ]
    ///The `remove()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/remove)
    ///
    ///*This API requires the following crate features to be activated: `DocumentType`*
    pub fn remove(this: &DocumentType);

    # [ wasm_bindgen ( catch , method , structural , variadic , js_class = "DocumentType" , js_name = replaceWith ) ]
    ///The `replaceWith()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/replaceWith)
    ///
    ///*This API requires the following crate features to be activated: `DocumentType`*
    pub fn replace_with_with_node(
        this: &DocumentType,
        nodes: &::js_sys::Array,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "DocumentType" , js_name = replaceWith ) ]
    ///The `replaceWith()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/replaceWith)
    ///
    ///*This API requires the following crate features to be activated: `DocumentType`*
    pub fn replace_with_with_node_0(this: &DocumentType) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "DocumentType" , js_name = replaceWith ) ]
    ///The `replaceWith()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/replaceWith)
    ///
    ///*This API requires the following crate features to be activated: `DocumentType`*
    pub fn replace_with_with_node_1(this: &DocumentType, nodes_1: &Node) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "DocumentType" , js_name = replaceWith ) ]
    ///The `replaceWith()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/replaceWith)
    ///
    ///*This API requires the following crate features to be activated: `DocumentType`*
    pub fn replace_with_with_node_2(
        this: &DocumentType,
        nodes_1: &Node,
        nodes_2: &Node,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "DocumentType" , js_name = replaceWith ) ]
    ///The `replaceWith()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/replaceWith)
    ///
    ///*This API requires the following crate features to be activated: `DocumentType`*
    pub fn replace_with_with_node_3(
        this: &DocumentType,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "DocumentType" , js_name = replaceWith ) ]
    ///The `replaceWith()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/replaceWith)
    ///
    ///*This API requires the following crate features to be activated: `DocumentType`*
    pub fn replace_with_with_node_4(
        this: &DocumentType,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "DocumentType" , js_name = replaceWith ) ]
    ///The `replaceWith()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/replaceWith)
    ///
    ///*This API requires the following crate features to be activated: `DocumentType`*
    pub fn replace_with_with_node_5(
        this: &DocumentType,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
        nodes_5: &Node,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "DocumentType" , js_name = replaceWith ) ]
    ///The `replaceWith()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/replaceWith)
    ///
    ///*This API requires the following crate features to be activated: `DocumentType`*
    pub fn replace_with_with_node_6(
        this: &DocumentType,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
        nodes_5: &Node,
        nodes_6: &Node,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "DocumentType" , js_name = replaceWith ) ]
    ///The `replaceWith()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/replaceWith)
    ///
    ///*This API requires the following crate features to be activated: `DocumentType`*
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

    # [ wasm_bindgen ( catch , method , structural , variadic , js_class = "DocumentType" , js_name = replaceWith ) ]
    ///The `replaceWith()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/replaceWith)
    ///
    ///*This API requires the following crate features to be activated: `DocumentType`*
    pub fn replace_with_with_str(
        this: &DocumentType,
        nodes: &::js_sys::Array,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "DocumentType" , js_name = replaceWith ) ]
    ///The `replaceWith()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/replaceWith)
    ///
    ///*This API requires the following crate features to be activated: `DocumentType`*
    pub fn replace_with_with_str_0(this: &DocumentType) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "DocumentType" , js_name = replaceWith ) ]
    ///The `replaceWith()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/replaceWith)
    ///
    ///*This API requires the following crate features to be activated: `DocumentType`*
    pub fn replace_with_with_str_1(this: &DocumentType, nodes_1: &str) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "DocumentType" , js_name = replaceWith ) ]
    ///The `replaceWith()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/replaceWith)
    ///
    ///*This API requires the following crate features to be activated: `DocumentType`*
    pub fn replace_with_with_str_2(
        this: &DocumentType,
        nodes_1: &str,
        nodes_2: &str,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "DocumentType" , js_name = replaceWith ) ]
    ///The `replaceWith()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/replaceWith)
    ///
    ///*This API requires the following crate features to be activated: `DocumentType`*
    pub fn replace_with_with_str_3(
        this: &DocumentType,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "DocumentType" , js_name = replaceWith ) ]
    ///The `replaceWith()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/replaceWith)
    ///
    ///*This API requires the following crate features to be activated: `DocumentType`*
    pub fn replace_with_with_str_4(
        this: &DocumentType,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "DocumentType" , js_name = replaceWith ) ]
    ///The `replaceWith()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/replaceWith)
    ///
    ///*This API requires the following crate features to be activated: `DocumentType`*
    pub fn replace_with_with_str_5(
        this: &DocumentType,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
        nodes_5: &str,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "DocumentType" , js_name = replaceWith ) ]
    ///The `replaceWith()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/replaceWith)
    ///
    ///*This API requires the following crate features to be activated: `DocumentType`*
    pub fn replace_with_with_str_6(
        this: &DocumentType,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
        nodes_5: &str,
        nodes_6: &str,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "DocumentType" , js_name = replaceWith ) ]
    ///The `replaceWith()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/replaceWith)
    ///
    ///*This API requires the following crate features to be activated: `DocumentType`*
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
