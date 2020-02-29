use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = Selection , typescript_type = "Selection" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `Selection` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Selection)
    ///
    ///*This API requires the following crate features to be activated: `Selection`*
    pub type Selection;

    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "Selection" , js_name = anchorNode ) ]
    ///Getter for the `anchorNode` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Selection/anchorNode)
    ///
    ///*This API requires the following crate features to be activated: `Node`, `Selection`*
    pub fn anchor_node(this: &Selection) -> Option<Node>;

    # [ wasm_bindgen ( structural , method , getter , js_class = "Selection" , js_name = anchorOffset ) ]
    ///Getter for the `anchorOffset` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Selection/anchorOffset)
    ///
    ///*This API requires the following crate features to be activated: `Selection`*
    pub fn anchor_offset(this: &Selection) -> u32;

    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "Selection" , js_name = focusNode ) ]
    ///Getter for the `focusNode` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Selection/focusNode)
    ///
    ///*This API requires the following crate features to be activated: `Node`, `Selection`*
    pub fn focus_node(this: &Selection) -> Option<Node>;

    # [ wasm_bindgen ( structural , method , getter , js_class = "Selection" , js_name = focusOffset ) ]
    ///Getter for the `focusOffset` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Selection/focusOffset)
    ///
    ///*This API requires the following crate features to be activated: `Selection`*
    pub fn focus_offset(this: &Selection) -> u32;

    # [ wasm_bindgen ( structural , method , getter , js_class = "Selection" , js_name = isCollapsed ) ]
    ///Getter for the `isCollapsed` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Selection/isCollapsed)
    ///
    ///*This API requires the following crate features to be activated: `Selection`*
    pub fn is_collapsed(this: &Selection) -> bool;

    # [ wasm_bindgen ( structural , method , getter , js_class = "Selection" , js_name = rangeCount ) ]
    ///Getter for the `rangeCount` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Selection/rangeCount)
    ///
    ///*This API requires the following crate features to be activated: `Selection`*
    pub fn range_count(this: &Selection) -> u32;

    # [ wasm_bindgen ( structural , method , getter , js_class = "Selection" , js_name = type ) ]
    ///Getter for the `type` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Selection/type)
    ///
    ///*This API requires the following crate features to be activated: `Selection`*
    pub fn type_(this: &Selection) -> String;

    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "Selection" , js_name = caretBidiLevel ) ]
    ///Getter for the `caretBidiLevel` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Selection/caretBidiLevel)
    ///
    ///*This API requires the following crate features to be activated: `Selection`*
    pub fn caret_bidi_level(this: &Selection) -> Result<Option<i16>, JsValue>;

    # [ wasm_bindgen ( structural , catch , method , setter , js_class = "Selection" , js_name = caretBidiLevel ) ]
    ///Setter for the `caretBidiLevel` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Selection/caretBidiLevel)
    ///
    ///*This API requires the following crate features to be activated: `Selection`*
    pub fn set_caret_bidi_level(this: &Selection, value: Option<i16>) -> Result<(), JsValue>;

    #[cfg(feature = "Range")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "Selection" , js_name = addRange ) ]
    ///The `addRange()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Selection/addRange)
    ///
    ///*This API requires the following crate features to be activated: `Range`, `Selection`*
    pub fn add_range(this: &Selection, range: &Range) -> Result<(), JsValue>;

    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "Selection" , js_name = collapse ) ]
    ///The `collapse()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Selection/collapse)
    ///
    ///*This API requires the following crate features to be activated: `Node`, `Selection`*
    pub fn collapse(this: &Selection, node: Option<&Node>) -> Result<(), JsValue>;

    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "Selection" , js_name = collapse ) ]
    ///The `collapse()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Selection/collapse)
    ///
    ///*This API requires the following crate features to be activated: `Node`, `Selection`*
    pub fn collapse_with_offset(
        this: &Selection,
        node: Option<&Node>,
        offset: u32,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Selection" , js_name = collapseToEnd ) ]
    ///The `collapseToEnd()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Selection/collapseToEnd)
    ///
    ///*This API requires the following crate features to be activated: `Selection`*
    pub fn collapse_to_end(this: &Selection) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Selection" , js_name = collapseToStart ) ]
    ///The `collapseToStart()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Selection/collapseToStart)
    ///
    ///*This API requires the following crate features to be activated: `Selection`*
    pub fn collapse_to_start(this: &Selection) -> Result<(), JsValue>;

    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "Selection" , js_name = containsNode ) ]
    ///The `containsNode()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Selection/containsNode)
    ///
    ///*This API requires the following crate features to be activated: `Node`, `Selection`*
    pub fn contains_node(this: &Selection, node: &Node) -> Result<bool, JsValue>;

    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "Selection" , js_name = containsNode ) ]
    ///The `containsNode()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Selection/containsNode)
    ///
    ///*This API requires the following crate features to be activated: `Node`, `Selection`*
    pub fn contains_node_with_allow_partial_containment(
        this: &Selection,
        node: &Node,
        allow_partial_containment: bool,
    ) -> Result<bool, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Selection" , js_name = deleteFromDocument ) ]
    ///The `deleteFromDocument()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Selection/deleteFromDocument)
    ///
    ///*This API requires the following crate features to be activated: `Selection`*
    pub fn delete_from_document(this: &Selection) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Selection" , js_name = empty ) ]
    ///The `empty()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Selection/empty)
    ///
    ///*This API requires the following crate features to be activated: `Selection`*
    pub fn empty(this: &Selection) -> Result<(), JsValue>;

    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "Selection" , js_name = extend ) ]
    ///The `extend()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Selection/extend)
    ///
    ///*This API requires the following crate features to be activated: `Node`, `Selection`*
    pub fn extend(this: &Selection, node: &Node) -> Result<(), JsValue>;

    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "Selection" , js_name = extend ) ]
    ///The `extend()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Selection/extend)
    ///
    ///*This API requires the following crate features to be activated: `Node`, `Selection`*
    pub fn extend_with_offset(this: &Selection, node: &Node, offset: u32) -> Result<(), JsValue>;

    #[cfg(feature = "Range")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "Selection" , js_name = getRangeAt ) ]
    ///The `getRangeAt()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Selection/getRangeAt)
    ///
    ///*This API requires the following crate features to be activated: `Range`, `Selection`*
    pub fn get_range_at(this: &Selection, index: u32) -> Result<Range, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Selection" , js_name = modify ) ]
    ///The `modify()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Selection/modify)
    ///
    ///*This API requires the following crate features to be activated: `Selection`*
    pub fn modify(
        this: &Selection,
        alter: &str,
        direction: &str,
        granularity: &str,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Selection" , js_name = removeAllRanges ) ]
    ///The `removeAllRanges()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Selection/removeAllRanges)
    ///
    ///*This API requires the following crate features to be activated: `Selection`*
    pub fn remove_all_ranges(this: &Selection) -> Result<(), JsValue>;

    #[cfg(feature = "Range")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "Selection" , js_name = removeRange ) ]
    ///The `removeRange()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Selection/removeRange)
    ///
    ///*This API requires the following crate features to be activated: `Range`, `Selection`*
    pub fn remove_range(this: &Selection, range: &Range) -> Result<(), JsValue>;

    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "Selection" , js_name = selectAllChildren ) ]
    ///The `selectAllChildren()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Selection/selectAllChildren)
    ///
    ///*This API requires the following crate features to be activated: `Node`, `Selection`*
    pub fn select_all_children(this: &Selection, node: &Node) -> Result<(), JsValue>;

    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "Selection" , js_name = setBaseAndExtent ) ]
    ///The `setBaseAndExtent()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Selection/setBaseAndExtent)
    ///
    ///*This API requires the following crate features to be activated: `Node`, `Selection`*
    pub fn set_base_and_extent(
        this: &Selection,
        anchor_node: &Node,
        anchor_offset: u32,
        focus_node: &Node,
        focus_offset: u32,
    ) -> Result<(), JsValue>;

    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "Selection" , js_name = setPosition ) ]
    ///The `setPosition()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Selection/setPosition)
    ///
    ///*This API requires the following crate features to be activated: `Node`, `Selection`*
    pub fn set_position(this: &Selection, node: Option<&Node>) -> Result<(), JsValue>;

    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "Selection" , js_name = setPosition ) ]
    ///The `setPosition()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Selection/setPosition)
    ///
    ///*This API requires the following crate features to be activated: `Node`, `Selection`*
    pub fn set_position_with_offset(
        this: &Selection,
        node: Option<&Node>,
        offset: u32,
    ) -> Result<(), JsValue>;

}
